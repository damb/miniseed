use std::error;
use std::ffi::CStr;
use std::fmt;

use num::cast::AsPrimitive;

use crate::{MSErrorCode, Result};

const MS_NOERROR: i64 = libmseed_sys::MS_NOERROR as i64;
const MS_ENDOFFILE: i64 = libmseed_sys::MS_ENDOFFILE as i64;

/// Utility function which turns a libmseed error into a result.
pub(crate) fn check<T: PartialOrd + AsPrimitive<i64>>(code: T) -> Result<T> {
    let c = code.as_();
    if c >= MS_NOERROR {
        return Ok(code);
    }

    Err(MSError::from_raw(c as i32))
}

/// Utility function which checks for `MS_ENDOFFILE`.
pub(crate) fn check_eof<T: PartialEq + AsPrimitive<i64>>(code: T) -> bool {
    code.as_() == MS_ENDOFFILE
}

/// A structure representing libmseed errors.
#[derive(Debug, PartialEq)]
pub struct MSError {
    code: i32,
    message: String,
}

pub(crate) const MS_GENERROR: i32 = libmseed_sys::MS_GENERROR as i32;
pub(crate) const MS_NOTSEED: i32 = libmseed_sys::MS_NOTSEED as i32;
pub(crate) const MS_WRONGLENGTH: i32 = libmseed_sys::MS_WRONGLENGTH as i32;
pub(crate) const MS_OUTOFRANGE: i32 = libmseed_sys::MS_OUTOFRANGE as i32;
pub(crate) const MS_UNKNOWNFORMAT: i32 = libmseed_sys::MS_UNKNOWNFORMAT as i32;
pub(crate) const MS_STBADCOMPFLAG: i32 = libmseed_sys::MS_STBADCOMPFLAG as i32;
pub(crate) const MS_INVALIDCRC: i32 = libmseed_sys::MS_INVALIDCRC as i32;

impl MSError {
    /// Create a new error from a given raw error code.
    pub(crate) fn from_raw(code: i32) -> Self {
        unsafe {
            let message = CStr::from_ptr(libmseed_sys::ms_errorstr(code)).to_bytes();
            let message = String::from_utf8_lossy(message).into_owned();

            Self { code, message }
        }
    }

    /// Create a new error from the given string as the error.
    ///
    /// The error returned will have the code `MS_GENERROR`.
    pub fn from_str(s: &str) -> Self {
        Self {
            code: MS_GENERROR,
            message: s.to_string(),
        }
    }

    /// Return the error code associated with this error.
    pub fn code(&self) -> MSErrorCode {
        match self.raw_code() {
            MS_NOTSEED => MSErrorCode::NotSEED,
            MS_WRONGLENGTH => MSErrorCode::WrongLength,
            MS_OUTOFRANGE => MSErrorCode::OutOfRange,
            MS_UNKNOWNFORMAT => MSErrorCode::UnknownFormat,
            MS_STBADCOMPFLAG => MSErrorCode::SteimBadCompressionFlag,
            MS_INVALIDCRC => MSErrorCode::InvalidCRC,
            _ => MSErrorCode::GenericError,
        }
    }

    /// Return the raw error code associated with this error.
    pub fn raw_code(&self) -> i32 {
        match self.code {
            MS_NOTSEED => MS_NOTSEED,
            MS_WRONGLENGTH => MS_WRONGLENGTH,
            MS_OUTOFRANGE => MS_OUTOFRANGE,
            MS_UNKNOWNFORMAT => MS_UNKNOWNFORMAT,
            MS_STBADCOMPFLAG => MS_STBADCOMPFLAG,
            MS_INVALIDCRC => MS_INVALIDCRC,
            _ => MS_GENERROR,
        }
    }

    /// Return the message associated with this error
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl error::Error for MSError {}

impl fmt::Display for MSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}; code={:?} ({})",
            self.message,
            self.code(),
            self.code
        )?;

        Ok(())
    }
}
