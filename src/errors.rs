use super::*;

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
    ConversionError,
    InvalidLibraryResult,
    OutOfBounds,
    NodeNotFound,
    Other,
}

impl std::fmt::Display for CgnsErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct CgnsError {
    cause: Option<Box<dyn std::error::Error>>,
    kind: CgnsErrorKind,
}

impl std::fmt::Display for CgnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}
impl std::fmt::Debug for CgnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(cause) = &self.cause {
            write!(f, "CgnsError({}): {}", self.kind, cause)
        } else {
            write!(f, "CgnsError({})", self.kind)
        }
    }
}
impl std::error::Error for CgnsError {}

impl From<std::ffi::NulError> for CgnsError {
    fn from(err: std::ffi::NulError) -> Self {
        CgnsError {
            cause: Some(Box::new(err)),
            kind: CgnsErrorKind::ConversionError,
        }
    }
}

impl From<std::str::Utf8Error> for CgnsError {
    fn from(err: std::str::Utf8Error) -> Self {
        CgnsError {
            cause: Some(Box::new(err)),
            kind: CgnsErrorKind::ConversionError,
        }
    }
}

/*
impl From<std::ffi::FromBytesWithNulError> for CgnsError {
    fn from(err: std::ffi::FromBytesWithNulError) -> Self {
        CgnsError {
            cause: Box::new(err),
            kind: CgnsErrorKind::ConversionError,
        }
    }
}
*/

impl CgnsError {
    pub fn library(ier: i32, messsage: String) -> Self {
        Self {
            kind: CgnsErrorKind::Library,
            cause: Some(Box::new(CgnsLibraryError { ier, messsage })),
        }
    }
    pub fn out_of_bounds() -> Self {
        Self {
            kind: CgnsErrorKind::OutOfBounds,
            cause: None,
        }
    }

    pub fn invalid_lib_result() -> Self {
        Self {
            kind: CgnsErrorKind::InvalidLibraryResult,
            cause: None,
        }
    }

    pub fn node_not_found() -> Self {
        Self {
            kind: CgnsErrorKind::NodeNotFound,
            cause: None,
        }
    }
}

pub type CgnsResult<T> = Result<T, CgnsError>;

pub fn to_cgns_result(ier: i32) -> CgnsResult<()> {
    use std::ffi::CStr;
    if ier != 0 {
        let error: String = unsafe { CStr::from_ptr(cgns_bindings::cg_get_error()) }
            .to_str()
            .unwrap()
            .to_string();
        Err(CgnsError::library(ier, error))
    } else {
        Ok(())
    }
}

pub fn to_cgio_result(ier: i32) -> CgnsResult<()> {
    use cgio_bindings::{cgio_error_message, CGIO_MAX_ERROR_LENGTH};
    use std::{ffi::CStr, mem::MaybeUninit, os::raw::c_char};
    if ier != 0 {
        let mut message = [MaybeUninit::<c_char>::uninit(); CGIO_MAX_ERROR_LENGTH as usize];
        let ier_ = unsafe { cgio_error_message(message.as_mut_ptr() as *mut c_char) };
        assert_eq!(
            ier, ier_,
            "The `cgio_error_message` function should always return the
             same error code as the one we called this function with."
        );
        let error: String = unsafe { CStr::from_ptr(message.as_ptr() as *const c_char) }
            .to_str()
            .unwrap()
            .to_string();
        Err(CgnsError::library(ier, error))
    } else {
        Ok(())
    }
}
