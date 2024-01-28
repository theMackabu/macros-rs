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

#[macro_export]
macro_rules! file_exists {
    ($path: expr) => {
        $crate::file::Exists::check($path).file()
    };
}

#[macro_export]
macro_rules! folder_exists {
    ($path: expr) => {
        $crate::file::Exists::check($path).folder()
    };
}

#[macro_export]
macro_rules! path_empty {
    ($path: expr) => {
        $crate::file::Exists::check($path).empty()
    };
}

#[macro_export]
macro_rules! exists {
    ($path: expr) => {
        $crate::file::Exists::check($path)
    };
}
