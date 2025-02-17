use core::marker::PhantomData;

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Reflexive<T: Sized + Copy + Clone> {
    pub count: u32,
    pub address: Address,
    pub definition_data: u32,
    pub _phantom_data: PhantomData<T>
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct TagReference {
    pub fourcc: u32,
    pub _unknown_0x4: u32,
    pub _unknown_0x8: u32,
    pub tag_id: TagID,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Data {
    pub size: u32,
    pub flags: u32,
    pub file_offset: u32,
    pub data: Address,
    pub unknown: u32,
}
pub type FileData = Data;
pub type UTF16String = Data;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Address(pub u32);

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct TagID(pub u32);

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct ID(pub u32);

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct ScenarioScriptNodeValue(pub u32);

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct TagGroupFourCC(pub u32);

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Index(pub u16);
impl Index {
    pub const fn new(index: usize) -> Result<Self, &'static str> {
        if index == u16::MAX as usize {
            return Err("can't construct an index of 0xFFFF")
        }
        if index > u16::MAX as usize {
            return Err("index exceeds 0xFFFF")
        }
        Ok(Self(index as u16))
    }

    pub const fn new_none() -> Self {
        Self(u16::MAX)
    }

    pub const fn get(self) -> Option<usize> {
        match self.0 {
            u16::MAX => None,
            n => Some(n as usize)
        }
    }
}
