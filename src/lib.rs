#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use std::fmt::{Display, Formatter};
use std::error::Error as StdError;

#[derive(Debug, PartialEq)]
pub enum Error {
    ResultTooLarge
}

impl StdError for Error {
    fn description(&self) -> &'static str {
        match *self {
            Error::ResultTooLarge => "result too large"
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_str(StdError::description(self))
    }
}

pub fn write<T: Into<u64>>(buf: &mut Vec<u8>, value: T) -> Result<(), Error>
{
    let mut v: u64 = value.into();
    loop {
        buf.push(((v & 0x7f) | if v > 127 { 0x80 } else { 0 }) as u8);
        v >>= 7;
        if 0 == v { return Ok(()) }
    }
}

pub fn read<T: Iterator<Item=u8>>(bytes: &mut T, upper_bound: Option<u64>)
                                  -> Result<Option<u64>, Error>
{
    let mut shift = 0;
    let mut acc = 0;
    // let max = upper_bound.map(|ub| 64 - ub.leading_zeros()).unwrap_or(64);
    let max = upper_bound.unwrap_or(u64::max_value());

    for b in bytes {
        acc |= ((b & 0x7f) as u64) << shift;
        if acc > max { return Err(Error::ResultTooLarge) }
        shift += 7;
        if 0 == b & 0x80 { return Ok(Some(acc)) }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::TestResult;

    quickcheck! {

        fn round_trip(value: u64) -> bool {
            let mut buf = Vec::new();
            write(&mut buf, value).unwrap();
            value == read(&mut buf.into_iter(), None).unwrap().unwrap()
        }

        fn bounds_check(value: u64, bound: u64) -> TestResult {
            if value <= bound { return TestResult::discard() }
            let mut buf = Vec::new();
            write(&mut buf, value).unwrap();
            if Err(Error::ResultTooLarge) == read(&mut buf.into_iter(), Some(bound)) {
                TestResult::passed()
            } else { TestResult::failed() }
        }
    }
}
