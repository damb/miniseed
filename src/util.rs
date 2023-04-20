use std::ffi::CString;
use std::fmt;

use crate::error::{check, MSError};
use crate::Result;

pub(crate) fn nstime_to_time(nst: i64) -> Result<time::OffsetDateTime> {
    let mut year = 0;
    let mut yday = 0;
    let mut hour = 0;
    let mut min = 0;
    let mut sec = 0;
    let mut nsec = 0;
    unsafe {
        // TODO(damb): handle time error i.e. `NSTERROR`
        check(libmseed_sys::ms_nstime2time(
            nst, &mut year, &mut yday, &mut hour, &mut min, &mut sec, &mut nsec,
        ))?
    };

    let date = time::Date::try_from_yo(year.into(), yday)
        .map_err(|e| MSError::from_str(&e.to_string()))?;
    let datetime = date
        .try_with_hms_nano(hour, min, sec, nsec)
        .map_err(|e| MSError::from_str(&e.to_string()))?;
    Ok(datetime.assume_utc())
}

/// Convert a nanosecond time into a time string
pub fn nstime_to_string(nst: i64) -> Result<String> {
    let show_subseconds = 1;
    let time_format = libmseed_sys::ms_timeformat_t_SEEDORDINAL;
    let time = CString::new("                                     ")
        .unwrap()
        .into_raw();
    unsafe {
        if libmseed_sys::ms_nstime2timestr(nst, time, time_format, show_subseconds).is_null() {
            return Err(MSError::from_str("failed to convert nstime to string"));
        }

        Ok(CString::from_raw(time).into_string().unwrap())
    }
}

/// Utility function safely converting a slice of `i8` values into a `String`
pub(crate) fn i8_to_string(buf: &[i8]) -> String {
    let v: Vec<u8> = buf
        .iter()
        .map(|x| *x as u8) // cast i8 as u8
        .filter(|x| *x != 0u8) // remove null bytes
        .collect();

    String::from_utf8_lossy(&v).to_string()
}

/// A structure representing network, station, location, and channel identifiers.
#[derive(Debug)]
pub(crate) struct NSLC {
    pub net: String,
    pub sta: String,
    pub loc: String,
    pub cha: String,
}

impl NSLC {
    /// Create a new `NSLC` structure from a stream identifier buffer slice.
    pub fn try_from_sid(sid: &[i8]) -> Result<Self> {
        let s0 = "               ";
        let sid = CString::new(i8_to_string(sid)).unwrap().into_raw();
        let xnet = CString::new(s0).unwrap().into_raw();
        let xsta = CString::new(s0).unwrap().into_raw();
        let xloc = CString::new(s0).unwrap().into_raw();
        let xcha = CString::new(s0).unwrap().into_raw();
        unsafe {
            check(libmseed_sys::ms_sid2nslc(sid, xnet, xsta, xloc, xcha))?;
            let net = CString::from_raw(xnet).into_string().unwrap();
            let sta = CString::from_raw(xsta).into_string().unwrap();
            let loc = CString::from_raw(xloc).into_string().unwrap();
            let cha = CString::from_raw(xcha).into_string().unwrap();
            Ok(Self { net, sta, loc, cha })
        }
    }
}

impl fmt::Display for NSLC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}_{}_{}", self.net, self.sta, self.loc, self.cha)
    }
}
