use std::fmt;
// use std::path::Path;
use std::ptr;
use std::slice::from_raw_parts;

// use libmseed_sys::MS3FileParam;
use libmseed_sys::MS3Record;
// use libmseed_sys::MS3TraceID;
// use libmseed_sys::MS3TraceList;
// use libmseed_sys::MS3TraceSeg;

use crate::error::check;
pub use crate::error::MSError;

mod error;
mod util;

#[derive(Debug)]
pub struct MSRecord(*mut MS3Record);

// #[derive(Debug)]
// pub struct MSFileParam {
//     path: String,
//     mspath: CString,
//     msfp: *mut MS3FileParam,
//     fpos: i64,
//     last: i8,
//     verbose: i8,
//     flags: u32,
// }

// #[derive(Debug)]
// pub struct MSTraceList {
//     // Pointer to Miniseed Trace List
//     mstl: *mut MS3TraceList,
//     // Miniseed file name
//     path: String,
// }

// #[derive(Debug)]
// pub struct MSTraceID(*mut MS3TraceID);
// #[derive(Debug)]
// pub struct MSTraceSegment(*mut MS3TraceSeg);

// #[derive(Debug)]
// pub struct MSTraceIDIterator {
//     mstid: *mut MS3TraceID,
// }
// #[derive(Debug)]
// pub struct MSTraceSegmentIterator {
//     mstseg: *mut MS3TraceSeg,
// }

/// An enumeration of possible sample types.
#[repr(i8)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MSSampleType {
    Ascii = 97,      // a
    Integer32 = 105, // i
    Float32 = 102,   // f
    Float64 = 100,   // d
}

impl MSSampleType {
    /// Create a `MSSampleType` from the given `ch`.
    pub fn try_from_char(ch: i8) -> Result<Self> {
        match ch {
            97 => Ok(Self::Ascii),      // a
            105 => Ok(Self::Integer32), // i
            102 => Ok(Self::Float32),   // f
            100 => Ok(Self::Float64),   // d
            other => Err(MSError::from_str(&format!(
                "invalid sample type: {}",
                other
            ))),
        }
    }
}

/// An enumeration of possible data encodings.
#[repr(i8)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MSDataEncoding {
    Ascii = libmseed_sys::DE_ASCII as i8,
    Integer16 = libmseed_sys::DE_INT16 as i8,
    Integer32 = libmseed_sys::DE_INT32 as i8,
    Float32 = libmseed_sys::DE_FLOAT32 as i8,
    Float64 = libmseed_sys::DE_FLOAT64 as i8,
    Steim1 = libmseed_sys::DE_STEIM1 as i8,
    Steim2 = libmseed_sys::DE_STEIM2 as i8,
    GeoScope24 = libmseed_sys::DE_GEOSCOPE24 as i8,
    GeoScope163 = libmseed_sys::DE_GEOSCOPE163 as i8,
    GeoScope164 = libmseed_sys::DE_GEOSCOPE164 as i8,
    CDSN = libmseed_sys::DE_CDSN as i8,
    SRO = libmseed_sys::DE_SRO as i8,
    DWWSSN = libmseed_sys::DE_DWWSSN as i8,
}

impl MSDataEncoding {
    /// Create a `MSDataEncoding` from the given `ch`.
    pub fn try_from_char(ch: i8) -> Result<Self> {
        match ch as u32 {
            libmseed_sys::DE_ASCII => Ok(Self::Ascii),
            libmseed_sys::DE_INT16 => Ok(Self::Integer16),
            libmseed_sys::DE_INT32 => Ok(Self::Integer32),
            libmseed_sys::DE_FLOAT32 => Ok(Self::Float32),
            libmseed_sys::DE_FLOAT64 => Ok(Self::Float64),
            libmseed_sys::DE_STEIM1 => Ok(Self::Steim1),
            libmseed_sys::DE_STEIM2 => Ok(Self::Steim2),
            libmseed_sys::DE_GEOSCOPE24 => Ok(Self::GeoScope24),
            libmseed_sys::DE_GEOSCOPE163 => Ok(Self::GeoScope163),
            libmseed_sys::DE_GEOSCOPE164 => Ok(Self::GeoScope164),
            libmseed_sys::DE_CDSN => Ok(Self::CDSN),
            libmseed_sys::DE_SRO => Ok(Self::SRO),
            libmseed_sys::DE_DWWSSN => Ok(Self::DWWSSN),
            other => Err(MSError::from_str(&format!(
                "invalid data encoding type: {}",
                other
            ))),
        }
    }
}

/// An enumeration of possible errors that can happen when working with MiniSEED records.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MSErrorCode {
    /// Generic unspecified error
    GenericError,
    /// Data not SEED
    NotSEED,
    /// Length of data read was incorrect
    WrongLength,
    /// SEED record length out of range
    OutOfRange,
    /// Unknown data encoding format
    UnknownFormat,
    /// Steim, invalid compression
    SteimBadCompressionFlag,
    /// Invalid CRC
    InvalidCRC,
}

/// A specialized library `Result` type.
type Result<T> = std::result::Result<T, MSError>;

// impl MSTraceList {
//     pub fn new<S: AsRef<Path>>(file: S) -> Self {
//         let path: String = file.as_ref().to_string_lossy().into_owned();
//         let mstl: *mut MS3TraceList = ptr::null_mut();
//         MSTraceList { mstl, path }
//     }
//     pub fn read(&mut self) {
//         let mspath = CString::new(self.path.clone()).unwrap();
//         let verbose = 0;
//         let splitversion = 0;
//         let flags = libmseed_sys::MSF_UNPACKDATA;
//         let tolerance = ptr::null_mut();
//         let rv = unsafe {
//             libmseed_sys::ms3_readtracelist(
//                 (&mut self.mstl) as *mut *mut MS3TraceList,
//                 mspath.as_ptr(),
//                 tolerance,
//                 splitversion,
//                 flags,
//                 verbose,
//             )
//         };
//         assert_eq!(rv, MS_NOERROR);
//     }
//     fn ptr(&self) -> MS3TraceList {
//         unsafe { *self.mstl }
//     }
//     pub fn numtraces(&self) -> u32 {
//         self.ptr().numtraces
//     }
//     pub fn traces(&self) -> MSTraceIDIterator {
//         MSTraceIDIterator {
//             mstid: self.ptr().traces,
//         }
//     }
// }

// impl MSTraceID {
//     fn ptr(&self) -> MS3TraceID {
//         unsafe { *self.0 }
//     }
//     pub fn segments(&self) -> MSTraceSegmentIterator {
//         MSTraceSegmentIterator {
//             mstseg: self.ptr().first,
//         }
//     }
//     pub fn network(&self) -> String {
//         sid_to_nslc(&self.ptr().sid).net
//     }
//     pub fn station(&self) -> String {
//         sid_to_nslc(&self.ptr().sid).sta
//     }
//     pub fn location(&self) -> String {
//         sid_to_nslc(&self.ptr().sid).loc
//     }
//     pub fn channel(&self) -> String {
//         sid_to_nslc(&self.ptr().sid).cha
//     }
//     pub fn start_time(&self) -> time::OffsetDateTime {
//         nstime_to_time(self.ptr().earliest)
//     }
//     pub fn end_time(&self) -> time::OffsetDateTime {
//         nstime_to_time(self.ptr().latest)
//     }
//     pub fn pubversion(&self) -> u8 {
//         self.ptr().pubversion
//     }
//     pub fn numsegments(&self) -> u32 {
//         self.ptr().numsegments
//     }
// }

// impl Iterator for MSTraceIDIterator {
//     type Item = MSTraceID;
//     fn next(&mut self) -> Option<Self::Item> {
//         if (*self).mstid == ptr::null_mut() {
//             None
//         } else {
//             let prev = self.mstid;
//             self.mstid = unsafe { (*self.mstid).next };
//             Some(MSTraceID(prev))
//         }
//     }
// }

// impl Iterator for MSTraceSegmentIterator {
//     type Item = MSTraceSegment;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.mstseg == ptr::null_mut() {
//             None
//         } else {
//             let prev = self.mstseg;
//             self.mstseg = unsafe { (*self.mstseg).next };
//             Some(MSTraceSegment(prev))
//         }
//     }
// }

// impl MSSampleType {
//     pub fn as_char(&self) -> i8 {
//         match self {
//             MSSampleType::Integer32 => 'i' as i8,
//             MSSampleType::Float32 => 'f' as i8,
//             MSSampleType::Float64 => 'd' as i8,
//         }
//     }
// }

// impl MSTraceSegment {
//     fn ptr(&self) -> MS3TraceSeg {
//         unsafe { *self.0 }
//     }
//     fn sampletype(&self) -> MSSampleType {
//         let s = self.ptr();
//         match s.sampletype {
//             105 => MSSampleType::Integer32, // i
//             102 => MSSampleType::Float32,   // f
//             100 => MSSampleType::Float64,   // d
//             _ => panic!("Unknown sample type: {}", s.sampletype),
//         }
//     }
//     pub fn start_time(&self) -> time::OffsetDateTime {
//         nstime_to_time(self.ptr().starttime)
//     }
//     pub fn end_time(&self) -> time::OffsetDateTime {
//         nstime_to_time(self.ptr().endtime)
//     }
//     pub fn samprate(&self) -> f64 {
//         self.ptr().samprate
//     }
//     pub fn samplecnt(&self) -> i64 {
//         self.ptr().samplecnt
//     }
//     pub fn numsamples(&self) -> i64 {
//         self.ptr().numsamples
//     }
//     pub fn datasize(&self) -> usize {
//         self.ptr().datasize
//     }
//     pub fn data_unpacked(&self) -> bool {
//         self.samplecnt() == self.numsamples() && self.datasize() > 0
//     }

//     fn convert_data(&self, t: MSSampleType) -> bool {
//         if !self.data_unpacked() {
//             return false;
//         }
//         let truncate = 0;
//         if t != self.sampletype() {
//             unsafe { libmseed_sys::mstl3_convertsamples(self.0, t.as_char(), truncate) == 0 }
//         } else {
//             true
//         }
//     }
//     pub fn to_vec_i32(&self) -> Vec<i32> {
//         if !self.data_unpacked() {
//             return vec![];
//         }
//         self.convert_data(MSSampleType::Integer32);
//         let s = self.ptr();
//         unsafe { from_raw_parts(s.datasamples as *mut i32, s.samplecnt as usize) }.to_vec()
//     }
//     pub fn to_vec_f32(&self) -> Vec<f32> {
//         if !self.data_unpacked() {
//             return vec![];
//         }
//         self.convert_data(MSSampleType::Float32);
//         let s = self.ptr();
//         unsafe { from_raw_parts(s.datasamples as *mut f32, s.samplecnt as usize) }.to_vec()
//     }
//     pub fn to_vec_f64(&self) -> Vec<f64> {
//         if !self.data_unpacked() {
//             return vec![];
//         }
//         self.convert_data(MSSampleType::Float64);
//         let s = self.ptr();
//         unsafe { from_raw_parts(s.datasamples as *mut f64, s.samplecnt as usize) }.to_vec()
//     }
// }

// TODO(damb): implement remaining member functions
impl MSRecord {
    fn ptr(&self) -> MS3Record {
        unsafe { *self.0 }
    }

    /// Parse a `MSRecord` from a slice of bytes.
    pub fn parse(buf: &mut [u8], unpack: bool) -> Result<Self> {
        let msr: *mut MS3Record = ptr::null_mut();
        let mut msr = unsafe { libmseed_sys::msr3_init(msr) };
        if msr.is_null() {
            return Err(MSError::from_str("failed to initialize record"));
        }

        let mut flags: u32 = 0;
        if unpack {
            flags |= libmseed_sys::MSF_UNPACKDATA;
        }

        let ret = unsafe {
            let buf = &mut *(buf as *mut [u8] as *mut [i8]);
            check(libmseed_sys::msr3_parse(
                buf.as_mut().as_mut_ptr(),
                buf.as_mut().len() as u64,
                (&mut msr) as *mut *mut MS3Record,
                flags,
                0,
            ))
        };

        match ret {
            Ok(_) => Ok(Self(msr)),
            Err(err) => Err(err),
        }
    }

    /// Unpack data samples of the record and return the number of unpacked samples.
    ///
    /// If the data is already unpacked, the number of unpacked samples is returned, as well.
    pub fn unpack_data(&mut self) -> Result<i64> {
        if !self.ptr().datasamples.is_null() {
            return Ok(self.num_samples());
        }
        unsafe {
            check(libmseed_sys::msr3_unpack_data(
                (&mut self.ptr()) as *mut MS3Record,
                0,
            ))
        }
    }

    /// Return a lossy version of the stream indentifier.
    pub fn sid_lossy(&self) -> String {
        util::i8_to_string(&(self.ptr().sid))
    }

    /// Return the network code identifier of the record.
    pub fn network(&self) -> Result<String> {
        let nslc = util::NSLC::try_from_sid(&self.ptr().sid)?;
        Ok(nslc.net)
    }

    /// Return the station code identifier of the record.
    pub fn station(&self) -> Result<String> {
        let nslc = util::NSLC::try_from_sid(&self.ptr().sid)?;
        Ok(nslc.sta)
    }

    /// Return the location code identifier of the record.
    pub fn location(&self) -> Result<String> {
        let nslc = util::NSLC::try_from_sid(&self.ptr().sid)?;
        Ok(nslc.sta)
    }

    /// Return the channel code identifier of the record.
    pub fn channel(&self) -> Result<String> {
        let nslc = util::NSLC::try_from_sid(&self.ptr().sid)?;
        Ok(nslc.cha)
    }

    /// Return the start time of the record.
    pub fn start_time(&self) -> Result<time::OffsetDateTime> {
        util::nstime_to_time(self.ptr().starttime)
    }

    /// Return the data encoding format of the record.
    pub fn encoding(&self) -> Result<MSDataEncoding> {
        MSDataEncoding::try_from_char(self.ptr().encoding)
    }

    /// Return the record publication version.
    pub fn pub_version(&self) -> u8 {
        self.ptr().pubversion
    }

    /// Return the number of data samples as indicated by the raw record.
    pub fn sample_cnt(&self) -> i64 {
        self.ptr().samplecnt
    }

    /// Return the CRC of the record.
    pub fn crc(&self) -> u32 {
        self.ptr().crc
    }

    /// Return the length of the extra headers in bytes.
    pub fn extra_length(&self) -> u16 {
        self.ptr().extralength
    }

    /// Return the length of the data payload in bytes.
    pub fn data_length(&self) -> u16 {
        self.ptr().datalength
    }

    //TODO(damb): extra()

    /// Return the data samples of the record.
    ///
    /// Note that the data samples are unpacked, if required. An empty slice is returned if unpacking
    /// the data samples failed.
    pub fn data_samples<T>(&mut self) -> &[T] {
        if self.ptr().datasamples.is_null() && self.unpack_data().is_err() {
            return &[];
        }

        unsafe {
            from_raw_parts(
                self.ptr().datasamples as *mut T,
                self.ptr().samplecnt as usize,
            )
        }
    }

    /// Return the size of the (unpacked) data samples in bytes.
    pub fn data_size(&self) -> usize {
        self.ptr().datasize
    }

    /// Return the number of (unpacked) data samples of the record.
    pub fn num_samples(&self) -> i64 {
        self.ptr().numsamples
    }

    /// Return the record sample type.
    pub fn sample_type(&self) -> Result<MSSampleType> {
        MSSampleType::try_from_char(self.ptr().sampletype)
    }
}

impl fmt::Display for MSRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = self.ptr();
        write!(
            f,
            "{}, {}, {}, {} samples, {} Hz, {:?}",
            self.sid_lossy(),
            v.pubversion,
            v.reclen,
            v.samplecnt,
            v.samprate,
            util::nstime_to_string(v.starttime).unwrap_or("invalid".to_string())
        )
    }
}

// impl MSFileParam {
//     pub fn new<S: AsRef<Path>>(file: S) -> MSFileParam {
//         let path: String = file.as_ref().to_string_lossy().into_owned();
//         let mspath = CString::new(path.clone()).unwrap();
//         let msfp: *mut MS3FileParam = ptr::null_mut();
//         Self {
//             path,
//             msfp,
//             mspath,
//             fpos: 0,
//             last: 0,
//             flags: libmseed_sys::MSF_UNPACKDATA,
//             verbose: 0,
//         }
//     }
//     pub fn unpack_data(&mut self, unpack: bool) {
//         if unpack {
//             self.flags |= libmseed_sys::MSF_UNPACKDATA;
//         } else {
//             self.flags &= !libmseed_sys::MSF_UNPACKDATA;
//         }
//     }
//     pub fn validate_crc(&mut self, validate: bool) {
//         if validate {
//             self.flags |= libmseed_sys::MSF_VALIDATECRC;
//         } else {
//             self.flags &= !libmseed_sys::MSF_VALIDATECRC;
//         }
//     }
//     pub fn verbose(&mut self, verbose: bool) {
//         self.verbose = if verbose { 1 } else { 0 };
//     }
//     pub fn filename(&self) -> &str {
//         &self.path
//     }

//     fn read_record(&mut self) -> Result<Option<MSRecord>> {
//         let mut msr: *mut MS3Record = ptr::null_mut();
//         let rv = unsafe {
//             libmseed_sys::ms3_readmsr_r(
//                 (&mut self.msfp) as *mut *mut MS3FileParam,
//                 (&mut msr) as *mut *mut MS3Record,
//                 self.mspath.as_ptr(),
//                 &mut self.fpos,
//                 &mut self.last,
//                 self.flags,
//                 self.verbose,
//             )
//         };

//         if check_eof(rv) {
//             return Ok(None);
//         }

//         match check(rv) {
//             Ok(_) => Ok(Some(MSRecord(msr))),
//             Err(err) => Err(err),
//         }
//     }
// }

// impl Iterator for MSFileParam {
//     type Item = Result<MSRecord>;
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.read_record() {
//             Ok(None) => None,
//             Ok(Some(msr)) => Some(Ok(msr)),
//             Err(e) => Some(Err(e)),
//         }
//     }
// }

// impl Drop for MSFileParam {
//     fn drop(&mut self) {
//         let mut msr: *mut MS3Record = ptr::null_mut();
//         let rv = unsafe {
//             libmseed_sys::ms3_readmsr_r(
//                 (&mut self.msfp) as *mut *mut MS3FileParam,
//                 (&mut msr) as *mut *mut MS3Record,
//                 ptr::null_mut(),
//                 ptr::null_mut(),
//                 ptr::null_mut(),
//                 0,
//                 0,
//             )
//         };
//         assert!(rv == MS_NOERROR);
//     }
// }

#[cfg(test)]
mod tests {
    // use super::*;
    // #[test]
    // fn file_param() {
    //     let fp = MSFileParam::new("./tests/multiple.seed");
    //     for r in fp {
    //         if let Ok(rec) = r {
    //             println!("{} {}", rec, rec.numsamples());
    //         }
    //     }
    // }
    // #[test]
    // fn trace_list() {
    //     let mut fp = MSTraceList::new("./tests/multiple.seed");
    //     fp.read();
    //     assert_eq!(fp.numtraces(), 1);
    //     for trace in fp.traces() {
    //         for segment in trace.segments() {
    //             let out = segment.to_vec_i32();
    //             assert_eq!(out.len(), 288000);
    //         }
    //     }
    // }
}
