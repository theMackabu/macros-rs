use std::{fs, path::Path};

pub struct Exists<'p> {
    path: &'p str,
}

impl<'p> Exists<'p> {
    pub fn check(path: &'p str) -> Self { Self { path } }
    pub fn folder(&self) -> bool { Path::new(self.path).is_dir() }
    pub fn file(&self) -> bool { Path::new(self.path).exists() }
    pub fn empty(&self) -> bool { fs::metadata(Path::new(self.path)).map(|m| m.len() == 0).unwrap_or(true) }
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_file_exists {
    ($path: expr) => {
        $crate::fs::Exists::check($path).file()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_folder_exists {
    ($path: expr) => {
        $crate::fs::Exists::check($path).folder()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_path_empty {
    ($path: expr) => {
        $crate::fs::Exists::check($path).empty()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _lib_exists {
    ($path: expr) => {
        $crate::fs::Exists::check($path)
    };
}

#[doc(inline)]
pub use _lib_file_exists as file_exists;

#[doc(inline)]
pub use _lib_folder_exists as folder_exists;

#[doc(inline)]
pub use _lib_path_empty as path_empty;

#[doc(inline)]
pub use _lib_exists as exists;
