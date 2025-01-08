use core::ffi::CStr;
use c_mine::c_mine;
use crate::id::ID;

pub const DATA_FOURCC: u32 = 0x64407440;

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
        assert!(name_bytes.len() < name.len(), "{table_name} is too long for a table!");
        name[..name_bytes.len()].copy_from_slice(name_bytes);

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
    pub fn clear(&mut self) {
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
    /// Create an iterator for the table using a Halo data iterator.
    ///
    /// # Safety
    ///
    /// Halo's data iterator cannot guarantee that the table has not changed.
    pub unsafe fn iter(&'static mut self) -> TableIterator<T, SALT> {
        let mut table_iterator: TableIterator<T, SALT> = core::mem::zeroed();
        table_iterator.init(self);
        table_iterator
    }
    fn reset_next_id(&mut self) {
        self.next_id = (ID::<SALT>::from_index(0).expect("??? no id?").full_id() >> 16) as u16;
    }
}

const ITER_FOURCC: u32 = 0x69746572;

/// Halo iterator for table
#[repr(C)]
pub struct TableIterator<T: Sized + 'static, const SALT: u16> {
    table: *mut DataTable<T, SALT>,
    current_index: u16,
    padding: [u8; 2],
    id: u32,
    salt: u32
}
impl<T: Sized + 'static, const SALT: u16> TableIterator<T, SALT> {
    pub unsafe fn init(&mut self, table: *mut DataTable<T, SALT>) {
        self.table = table as *mut _;
        self.current_index = 0;
        self.id = u32::MAX;
        self.salt = (self.table as u32) ^ ITER_FOURCC;
    }
    pub fn id(&self) -> ID<SALT> {
        ID::from_full_id(self.id)
    }
}
impl<T: Sized + 'static, const SALT: u16> Iterator for TableIterator<T, SALT> {
    type Item = &'static mut TableElement<T>;

    fn next(&mut self) -> Option<&'static mut TableElement<T>> {
        assert_eq!((self.table as u32) ^ ITER_FOURCC, self.salt, "Incorrect salt for iterator!");

        // SAFETY: This is fine 🔥🐶🔥
        let instance_data = unsafe { (&mut *self.table).first as *mut u8 };
        let instance_size = unsafe { (&mut *self.table).element_size as usize };
        let instance_count = unsafe { (&mut *self.table).maximum as usize };

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
// Ideally, these should not be called from Rust code.

#[c_mine]
pub extern "C" fn data_verify(table: Option<&'static DataTable<[u8; 0], 0>>) {
    let Some(table) = table else {
        panic!("null table passed into data_verify");
    };
    table.verify().expect("table is broken");
}

#[c_mine]
pub unsafe extern "C" fn data_iterator_new(iterator: &mut TableIterator<[u8; 0], 0>, table: Option<&'static mut DataTable<[u8; 0], 0>>) {
    let Some(table) = table else {
        panic!("null table passed into data_iterator_new");
    };
    assert!(table.is_valid(), "init iterator with invalid table");
    table.verify().expect("init iterator with failed verify");

    iterator.init(table as *mut _);
}

#[c_mine]
pub extern "C" fn data_iterator_next(iterator: &mut TableIterator<[u8; 0], 0>) -> Option<&mut TableElement<[u8; 0]>> {
    assert!(unsafe { &*iterator.table }.is_valid(), "iterating iterator with invalid table");

    // It just so happens that Iterator::next() nicely maps to the exact
    // FFI-compatible type that Halo wants (a nullable pointer). Yay!

    iterator.next()
}

#[c_mine]
pub unsafe extern "C" fn datum_get(table: Option<&'static DataTable<[u8; 0], 0>>, id: ID<0>) -> &'static mut TableElement<[u8; 0]> {
    let Some(table) = table else {
        panic!("null table passed into datum_get");
    };

    let full_id = id.full_id();

    let (Some(index), Some(identifier)) = (id.index(), id.identifier()) else {
        panic!("datum_get({name}, {full_id:08X}) - Null IDs are not allowed!", name=table.name());
    };

    if identifier == 0 && table.identifier_zero_invalid != 0 {
        panic!("datum_get({name}, {full_id:08X}) - Zeroed identifiers are not allowed in this table!", name=table.name());
    }

    let current_size = table.current_size;
    if index > current_size as usize {
        panic!("datum_get({name}, {full_id:08X}) - Index {index} is out-of-bounds (DataTable::current_size={current_size})", name=table.name());
    }

    let element = (table.first as *mut u8)
        .wrapping_add(index * (table.element_size as usize)) as *mut TableElement<[u8; 0]>;

    let element = &mut *element;
    let element_identifier = element.identifier();

    if identifier != 0 && element_identifier != identifier {
        panic!("datum_get({name}, {full_id:08X}) - Mismatched identifier (expected 0x{identifier:04X}, was actually 0x{element_identifier:04X})", name=table.name());
    }

    element
}
