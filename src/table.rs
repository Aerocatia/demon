use core::ptr::null_mut;
use c_mine::c_mine;
use crate::id::ID;

pub const DATA_FOURCC: u32 = 0x64407440;

#[repr(C)]
pub struct Table<T: Sized + 'static, const SALT: u16> {
    /// Name of the memes
    name: [u8; 32],

    /// Maximum/reserved elements in the table
    maximum: u16,

    /// Size of each element
    element_size: u16,

    /// ???
    unknown_1: [u8; 4],

    /// Unused (d@t@)
    data_fourcc: u32,

    /// ??? cleared on clear
    unknown_2: u16,

    /// Size
    current_size: u16,

    /// Number of active/valid instances
    count: u16,

    /// Next ID to use
    next_id: u16,

    /// Pointer to first element in the table
    first: *mut TableElement<T>
}
impl<T: Sized + 'static, const SALT: u16> Table<T, SALT> {
    /// # Safety
    ///
    /// `first_element` must point to at least `maximum` elements.
    pub unsafe fn init(table_name: &str, maximum: usize, first_element: *mut TableElement<T>) -> Table<T, SALT> {
        assert!(maximum < (u16::MAX - 1) as usize, "table maximum is too big for a table {table_name}");

        let element_size = size_of::<TableElement<T>>();
        assert!(element_size < u16::MAX as usize, "table element size is too big for a table {table_name}");

        let mut name = [0u8; 32];
        let name_bytes = table_name.as_bytes();
        assert!(name_bytes.len() < name.len(), "{table_name} is too long for a table!");
        name[..name_bytes.len()].copy_from_slice(name_bytes);

        let mut table = Table {
            name,
            maximum: maximum as u16,
            element_size: size_of::<T>() as u16,
            data_fourcc: DATA_FOURCC,
            first: first_element,
            next_id: 0,
            unknown_1: Default::default(),
            unknown_2: Default::default(),
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
        self.unknown_2 = 0;
        for i in self.get_instances_mut() {
            i.salt_bytes = [0u8; 4]
        }
        self.reset_next_id();
    }
    fn reset_next_id(&mut self) {
        self.next_id = (ID::<SALT>::from_index(0).expect("??? no id?").full_id() >> 16) as u16;
    }
}

const ITER_FOURCC: u32 = 0x69746572;

/// Halo iterator for table
#[repr(C)]
pub struct TableIterator<T: Sized + 'static, const SALT: u16> {
    table: *mut Table<T, SALT>,
    current_index: u16,
    padding: [u8; 2],
    id: u32,
    salt: u32
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
                let bytes = instance.salt_bytes;
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
    pub salt_bytes: [u8; 4],
    pub item: T
}
impl<T: Sized + 'static> TableElement<T> {
    pub const fn is_active(&self) -> bool {
        self.salt_bytes[0] != 0 || self.salt_bytes[1] != 0
    }
}

#[c_mine]
pub unsafe extern "C" fn iterator_next(iterator: &mut TableIterator<[u8; 0], 0>) -> *mut TableElement<[u8; 0]> {
    match iterator.next() {
        Some(t) => t as *mut _,
        None => null_mut()
    }
}