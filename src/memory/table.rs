use core::ffi::{c_char, CStr};
use core::fmt::{Debug, Formatter};
use c_mine::c_mine;
use crate::id::ID;
use crate::memory::allocate_into_game_state;

pub const DATA_FOURCC: u32 = 0x64407440;

/// Create an iterator for the table using a Halo data iterator.
///
/// # Safety
///
/// No guarantee is made that the table is not being accessed concurrently.
///
/// As such, any function that accesses the table elements is unsafe.
#[repr(C)]
pub struct DataTable<T: Sized + 'static, const SALT: u16> {
    /// Name of the data table
    name: [u8; 32],

    /// Maximum/reserved elements in the table
    maximum: u16,

    /// Size of each element
    element_size: u16,

    /// Valid if non-zero
    valid: u8,

    /// Zeroed identifiers are not allowed
    identifier_zero_invalid: u8,

    /// ???
    unknown_2: [u8; 2],

    /// Unused (d@t@)
    data_fourcc: u32,

    /// ??? cleared on clear
    unknown_3: u16,

    /// Size
    current_size: u16,

    /// Number of active/valid instances
    count: u16,

    /// Next ID to use
    next_id: u16,

    /// Pointer to first element in the table
    first: *mut TableElement<T>
}
impl<T: Sized + 'static, const SALT: u16> DataTable<T, SALT> {
    pub fn name(&self) -> &str {
        CStr::from_bytes_until_nul(&self.name)
            .expect("should have a null terminated name")
            .to_str()
            .expect("should be ASCII")
    }

    /// # Safety
    ///
    /// `first_element` must point to at least `maximum` elements.
    pub unsafe fn init(table_name: &str, maximum: usize, first_element: *mut TableElement<T>) -> DataTable<T, SALT> {
        assert!(maximum < (u16::MAX - 1) as usize, "table maximum is too big for a table {table_name}");

        let element_size = size_of::<TableElement<T>>();
        assert!(element_size < u16::MAX as usize, "table element size is too big for a table {table_name}");

        let mut name = [0u8; 32];
        let name_bytes = table_name.as_bytes();
        let name_len_truncated = name_bytes.len().min(name.len() - 1);
        name[..name_len_truncated].copy_from_slice(&name_bytes[..name_len_truncated]);

        let mut table = DataTable {
            name,
            maximum: maximum as u16,
            element_size: size_of::<T>() as u16,
            data_fourcc: DATA_FOURCC,
            identifier_zero_invalid: 0,
            first: first_element,
            next_id: 0,
            valid: 0,
            unknown_2: Default::default(),
            unknown_3: Default::default(),
            current_size: 0,
            count: 0,
        };
        table.reset_next_id();
        table
    }
    pub unsafe fn get_element(&mut self, id: ID<SALT>) -> Result<&mut TableElement<T>, GetElementError<T, SALT>> {
        let full_id = id.full_id();

        let (Some(index), Some(identifier)) = (id.index(), id.identifier()) else {
            return Err(GetElementError {
                table: self,
                id,
                err: GetElementByIdErrorKind::BadId("Null IDs are not allowed!")
            });
        };

        if identifier == 0 && self.identifier_zero_invalid != 0 {
            return Err(GetElementError {
                table: self,
                id,
                err: GetElementByIdErrorKind::BadId("Zeroed identifiers are not allowed in this table!")
            });
        }

        let current_size = self.current_size as usize;
        if index > current_size {
            return Err(GetElementError {
                table: self,
                id,
                err: GetElementByIdErrorKind::OutOfBounds { current_size },
            });
        }

        let element = (self.first as *mut u8)
            .wrapping_add(index * (self.element_size as usize)) as *mut TableElement<T>;

        let element = &mut *element;
        let element_identifier = element.identifier();

        if identifier != 0 && element_identifier != identifier {
            return Err(GetElementError {
                table: self,
                id,
                err: GetElementByIdErrorKind::MismatchedIdentifier { expected: identifier, actually: element_identifier }
            });
        }

        Ok(element)
    }
    pub fn get_instances(&self) -> &[TableElement<T>] {
        // SAFETY: Maximum should be correct
        unsafe {
            core::slice::from_raw_parts(self.first, self.maximum as usize)
        }
    }
    pub fn get_instances_mut(&mut self) -> &mut [TableElement<T>] {
        // SAFETY: Maximum should be correct
        unsafe {
            core::slice::from_raw_parts_mut(self.first, self.maximum as usize)
        }
    }
    pub unsafe fn clear(&mut self) {
        self.current_size = 0;
        self.count = 0;
        self.unknown_2 = [0u8; 2];
        for i in self.get_instances_mut() {
            i.identifier_bytes = [0u8; 4]
        }
        self.reset_next_id();
    }
    pub fn is_valid(&self) -> bool {
        self.valid != 0
    }
    pub fn verify(&self) -> Result<(), &'static str> {
        if self.first.is_null() {
            return Err("data pointer is null");
        }
        if self.data_fourcc != DATA_FOURCC {
            return Err("data fourcc ('d@t@') is invalid");
        }
        if self.maximum < self.count {
            return Err("maximum was less than count");
        }
        if self.maximum < self.current_size {
            return Err("maximum was less than current size");
        }
        if self.current_size < self.count {
            return Err("count was less than current size");
        }
        Ok(())
    }
    pub unsafe fn iter(&mut self) -> TableIterator<T, SALT> {
        let mut table_iterator: TableIterator<T, SALT> = core::mem::zeroed();
        table_iterator.init(Some(self));
        table_iterator
    }
    pub fn iter_salt(&self) -> u32 {
        (((self as *const _) as usize) as u32) ^ ITER_FOURCC
    }
    fn reset_next_id(&mut self) {
        self.next_id = (ID::<SALT>::from_index(0).expect("??? no id?").full_id() >> 16) as u16;
    }
}

pub struct GetElementError<'a, T: Sized + 'static, const SALT: u16> {
    table: &'a DataTable<T, SALT>,
    id: ID<SALT>,
    err: GetElementByIdErrorKind
}

impl<'a, T: Sized + 'static, const SALT: u16> Debug for GetElementError<'a, T, SALT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("<{}>::get_element(0x{:08X}): ", self.table.name(), self.id.full_id()))?;

        match self.err {
            GetElementByIdErrorKind::MismatchedIdentifier { expected, actually } => f.write_fmt(
                format_args!("Mismatched identifier (expected 0x{expected:04X}, was actually 0x{actually:04X})")
            ),
            GetElementByIdErrorKind::BadId(reason) => f.write_str(reason),
            GetElementByIdErrorKind::OutOfBounds { current_size } => f.write_fmt(format_args!("Out of bounds! (current_size=0x{current_size:04X})"))
        }
    }
}

#[derive(Copy, Clone)]
pub enum GetElementByIdErrorKind {
    OutOfBounds { current_size: usize },
    MismatchedIdentifier { expected: u16, actually: u16 },
    BadId(&'static str)
}

const ITER_FOURCC: u32 = 0x69746572;

/// Halo iterator for table
#[repr(C)]
pub struct TableIterator<'a, T: Sized + 'static, const SALT: u16> {
    table: Option<&'a mut DataTable<T, SALT>>,
    current_index: u16,
    padding: [u8; 2],
    id: u32,
    salt: u32
}
impl<'a, T: Sized + 'static, const SALT: u16> TableIterator<'a, T, SALT> {
    pub fn init(&mut self, table: Option<&'a mut DataTable<T, SALT>>) {
        let Some(table) = table else {
            panic!("trying to iterate a null table");
        };

        self.salt = table.iter_salt();
        self.table = Some(table);
        self.current_index = 0;
        self.id = u32::MAX;
    }
    pub fn id(&self) -> ID<SALT> {
        ID::from_full_id(self.id)
    }
}
impl<'a, T: Sized + 'static, const SALT: u16> Iterator for TableIterator<'a, T, SALT> {
    type Item = &'a mut TableElement<T>;

    fn next(&mut self) -> Option<&'a mut TableElement<T>> {
        let Some(ref table) = self.table else {
            panic!("iterating an iterator with a null table");
        };

        assert_eq!(table.iter_salt(), self.salt, "Incorrect salt for iterator!");

        // SAFETY: This is fine 🔥🐶🔥
        let instance_data = table.first as *mut u8;
        let instance_size = table.element_size as usize;
        let instance_count = table.maximum as usize;

        while (self.current_index as usize) < instance_count {
            let index = self.current_index as usize;
            self.current_index += 1;

            let instance: &mut TableElement<T> = unsafe {
                &mut *(instance_data.wrapping_add(index * instance_size) as *mut TableElement<T>)
            };

            if instance.is_active() {
                let bytes = instance.identifier_bytes;
                let salt = (u16::from_ne_bytes([bytes[0], bytes[1]]) as u32) << 16;
                self.id = salt | (index as u32);
                return Some(instance);
            }
        }

        None
    }
}

#[repr(C)]
pub struct TableElement<T: Sized + 'static> {
    pub identifier_bytes: [u8; 4],
    pub item: T
}
impl<T: Sized + 'static> TableElement<T> {
    pub const fn is_active(&self) -> bool {
        self.identifier() != 0
    }
    pub const fn identifier(&self) -> u16 {
        u16::from_ne_bytes([self.identifier_bytes[0], self.identifier_bytes[1]])
    }
}

// RE: table FFI functions
//
// We don't know what the type or salt is, but the iterator does not need it,
// so we're using [u8; 0] with a zero salt
//
// You should not use these functions in Rust code!

#[c_mine]
pub extern "C" fn data_verify(table: Option<&'static DataTable<[u8; 0], 0>>) {
    let Some(table) = table else {
        panic!("null table passed into data_verify");
    };
    table.verify().expect("table is broken");
}

#[c_mine]
pub unsafe extern "C" fn data_iterator_new(iterator: &mut TableIterator<'static, [u8; 0], 0>, table: Option<&'static mut DataTable<[u8; 0], 0>>) {
    let Some(table) = table else {
        panic!("null table passed into data_iterator_new");
    };
    assert!(table.is_valid(), "init iterator with invalid table");
    table.verify().expect("init iterator with failed verify");

    iterator.init(Some(table));
}

#[c_mine]
pub extern "C" fn data_iterator_next(iterator: &'static mut TableIterator<[u8; 0], 0>) -> Option<&'static mut TableElement<[u8; 0]>> {
    let Some(ref table) = iterator.table else {
        panic!("iterating table with a null table...");
    };

    assert!(table.is_valid(), "iterating invalid table");

    // It just so happens that Iterator::next() nicely maps to the exact
    // FFI-compatible type that Halo wants (a nullable pointer). Yay!

    iterator.next()
}

#[c_mine]
pub unsafe extern "C" fn datum_get(table: Option<&'static mut DataTable<[u8; 0], 0>>, id: ID<0>) -> &'static mut TableElement<[u8; 0]> {
    let Some(table) = table else {
        panic!("null table passed into datum_get");
    };

    table.get_element(id).expect("Failed to get element:")
}

#[c_mine]
pub extern "C" fn data_allocation_size(count: u16, element_size: u16) -> usize {
    size_of::<DataTable<[u8; 0], 0>>() + (count as usize) * (element_size as usize)
}

#[c_mine]
pub unsafe extern "C" fn data_initialize(data_table: &mut DataTable<[u8; 0], 0>, name: *const c_char, count: u16, element_size: u16) {
    let name = CStr::from_ptr(name).to_str().expect("initializing data table but name is not valid UTF-8");
    let ptr: *mut DataTable<[u8; 0], 0> = data_table as *mut _;
    *data_table = DataTable::init(
        name,
        count as usize,
        ptr.wrapping_add(1) as *mut _
    );
    data_table.element_size = element_size;
}

#[c_mine]
pub unsafe extern "C" fn game_state_data_new(name: *const c_char, count: u16, element_size: u16) -> *mut DataTable<[u8; 0], 0> {
    let size = data_allocation_size.get()(count, element_size);
    let data = allocate_into_game_state(
        || { CStr::from_ptr(name).to_str().expect("game_state_data_new with invalid name") },
        size
    ) as *mut DataTable<[u8; 0], 0>;
    data_initialize.get()(&mut *data, name, count, element_size);
    data
}
