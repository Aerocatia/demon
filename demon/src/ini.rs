use alloc::string::String;
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};
use gerbil_ini::{Ini, IniMode};
use minxp::env::current_exe;
use minxp::fs::read;

pub static INI: IniData = IniData { ini: UnsafeCell::new(MaybeUninit::uninit()), initialized: AtomicBool::new(false), initialization_attempted: AtomicBool::new(false) };

pub struct IniData {
    initialized: AtomicBool,
    initialization_attempted: AtomicBool,
    ini: UnsafeCell<MaybeUninit<Ini>>
}

// Safety: Trust me bro.
unsafe impl Sync for IniData {}

impl IniData {
    /// Get the value from the ini
    ///
    /// Returns `None` if the section/key do not exist.
    pub fn get(&self, section: &str, key: &str) -> Option<&str> {
        if self.initialized.load(Ordering::Relaxed) {
            // Safety: `initialized` is set to true, meaning this is initialized.
            unsafe { (&*self.ini.get()).assume_init_ref() }.get_value(section, key)
        }
        else {
            None
        }
    }

    /// Load the ini from demon.ini
    ///
    /// # Panics
    ///
    /// This function will panic if called more than once in a program's execution.
    pub fn try_load(&self) {
        if self.initialization_attempted.swap(true, Ordering::Relaxed) {
            panic!("initialization attempted already; do not call this more than once")
        }

        let ini_path = current_exe().unwrap().parent().unwrap().join("demon.ini");
        let data = read(&ini_path).unwrap_or_default();
        let string = String::from_utf8(data).expect("demon.ini is not UTF-8");
        let ini = Ini::parse(&string, IniMode::SimpleTrimmed).expect("cannot parse demon.ini");

        // Safety: This value still exists in RAM, and this function is only being called once.
        unsafe { (&mut *self.ini.get()).write(ini); }

        self.initialized.swap(true, Ordering::Relaxed);
    }
}

#[macro_export]
macro_rules! ini {
    ($section:literal, $key:literal) => {
        crate::ini::INI.get($section, $key)
    };
}

#[macro_export]
macro_rules! ini_bool {
    ($section:literal, $key:literal) => {{
        match crate::ini::INI.get($section, $key) {
            Some("1" | "true" | "TRUE") => Some(true),
            Some("0" | "false" | "FALSE") => Some(false),
            None => None,
            Some(n) => panic!("Expected true/false or 0/1 from ini value {}.{}, got `{n}`", $section, $key)
        }
    }};
}
