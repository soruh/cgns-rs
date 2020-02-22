#![feature(optin_builtin_traits)] // TODO: check if we can use this

use libcgns_sys as bindings;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};

pub static LIB_IN_USE: AtomicBool = AtomicBool::new(false);

pub struct CgnsLibraryError {
    ier: i32,
    messsage: String,
}
impl std::fmt::Display for CgnsLibraryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.messsage)
    }
}
impl std::fmt::Debug for CgnsLibraryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CgnsError ({}): {}", self.ier, self.messsage)
    }
}

impl std::error::Error for CgnsLibraryError {}

#[derive(Debug)]
pub enum CgnsErrorKind {
    Library,
    InputError,
    Other,
}

impl std::fmt::Display for CgnsErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct CgnsError {
    cause: Box<dyn std::error::Error>,
    kind: CgnsErrorKind,
}

impl std::fmt::Display for CgnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}
impl std::fmt::Debug for CgnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CgnsError({}): {}", self.kind, self.cause)
    }
}
impl std::error::Error for CgnsError {}

impl From<std::ffi::NulError> for CgnsError {
    fn from(err: std::ffi::NulError) -> Self {
        CgnsError {
            cause: Box::new(err),
            kind: CgnsErrorKind::InputError,
        }
    }
}

impl CgnsError {
    pub fn library(ier: i32, messsage: String) -> Self {
        Self {
            kind: CgnsErrorKind::Library,
            cause: Box::new(CgnsLibraryError { ier, messsage }),
        }
    }
}

pub type CgnsResult<T> = Result<T, CgnsError>; // TODO: error enum

macro_rules! to_cgns_result {
    ($ier: expr) => {{
        let ier = $ier;
        if ier != 0 {
            let error: String = unsafe { CStr::from_ptr(bindings::cg_get_error()) }
                .to_str()
                .unwrap()
                .to_string();
            Err(CgnsError::library(ier, error))
        } else {
            Ok(())
        }
    }};
}

pub struct Library {
    _phantom: PhantomData<()>,
}

impl !Send for Library {}
impl !Sync for Library {}

impl Library {
    #[inline]
    pub fn new() -> Self {
        Self::take()
    }
    pub fn take() -> Self {
        assert!(
            !LIB_IN_USE.compare_and_swap(false, true, Ordering::Release),
            "The CGNS library is already in use."
        );
        Self {
            _phantom: Default::default(),
        }
    }

    pub fn open(&mut self, filename: &str, mode: CgnsOpenMode) -> CgnsResult<File> {
        let filename = CString::new(filename)?;
        let mut file_number = 0;

        to_cgns_result!(unsafe {
            bindings::cg_open(filename.as_ptr(), mode as i32, &mut file_number)
        })?;

        Ok(File {
            file_number,
            _phantom: Default::default(),
        })
    }
}

#[repr(u32)]
pub enum CgnsOpenMode {
    // Closed = bindings::CG_MODE_CLOSED,
    Modify = bindings::CG_MODE_MODIFY,
    Read = bindings::CG_MODE_READ,
    Write = bindings::CG_MODE_WRITE,
}

#[repr(u32)]
pub enum CgnsFileType {
    ADF = bindings::CG_FILE_ADF,
    ADF2 = bindings::CG_FILE_ADF2,
    HDF5 = bindings::CG_FILE_HDF5,
    NONE = bindings::CG_FILE_NONE,
}

impl Drop for Library {
    fn drop(&mut self) {
        assert!(
            LIB_IN_USE.compare_and_swap(true, false, Ordering::Release),
            "Singleton was instanciated twice"
        );
    }
}

impl std::fmt::Debug for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Library")
    }
}

pub struct File<'f> {
    file_number: i32,
    _phantom: PhantomData<&'f ()>,
}

impl<'f> std::fmt::Debug for File<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "File[{}]", self.file_number)
    }
}

impl<'f> File<'f> {
    fn close_by_ref(&mut self) -> CgnsResult<()> {
        to_cgns_result!(unsafe { bindings::cg_close(self.file_number) })
    }

    // make sure this `File` isn't used after we close it
    pub fn close(mut self) -> CgnsResult<()> {
        self.close_by_ref()?;

        std::mem::forget(self); // we don't want to call `close` twice...

        Ok(())
    }

    /// exposes the cgns internal file_number (`fn`) of this file
    pub fn file_number(&self) -> i32 {
        self.file_number
    }

    pub fn save_as(
        &mut self,
        filename: &str,
        file_type: CgnsFileType,
        follow_links: bool,
    ) -> CgnsResult<()> {
        let filename = CString::new(filename)?;

        to_cgns_result!(unsafe {
            bindings::cg_save_as(
                self.file_number,
                filename.as_ptr(),
                file_type as i32,
                follow_links as i32,
            )
        })
    }
}

impl<'f> Drop for File<'f> {
    fn drop(&mut self) {
        self.close_by_ref()
            .expect(&format!("Failed to close {:?}", self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lib_single_instance() {
        let lib = Library::take();
        drop(lib);
    }

    #[test]
    fn lib_two_instances_sequential() {
        let lib = Library::take();
        drop(lib);

        let lib2 = Library::take();
        drop(lib2);
    }

    #[test]
    #[should_panic]
    fn lib_two_instances_parralel() {
        let lib = Library::take();
        let lib2 = Library::take();

        drop(lib);
        drop(lib2);
    }

    #[test]
    fn open_file() {
        let mut lib = Library::new();

        let file = lib
            .open("test.cgns", CgnsOpenMode::Write)
            .expect("Failed to open file");

        file.close().expect("Failed to close file");
    }
}
