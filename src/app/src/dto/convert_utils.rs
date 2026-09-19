//! # Provides basic conversion utility methods
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 
//! 

use chrono::{Duration, NaiveDateTime, Timelike, Utc}; 
use crate::constants;

pub struct ConvertUtils;

impl ConvertUtils {

    /// to_i64()
    /// 
    ///  Converts the string to an i64 
    /// 
    /// Parameters:
    ///  * raw_i64_string: String to be converted into an i64
    /// 
    /// Returns:
    ///  * i64: the resulting converted string as an i64, if successful. If not successful, a value of -1 is returned.
    ///  
    pub fn to_i64(raw_i64_string: String) -> i64 {
        match raw_i64_string.parse::<i64>(){
            Ok(p) => p,
            Err(_) => constants::INVALID_OTHER_ID
        }
    }

    /// is_aged()
    /// 
    ///  Provides a boolean for when the given NaiveDateTime is older than [hours] ago. True when the value is older, False when it is not.
    /// 
    /// Parameters:
    ///  * d: NaiveDateTime - the base date being compared
    ///  * hours: i64 - the number of elapsed hours being compares
    /// 
    /// Returns:
    ///  * bool: True if the date/time has elapsed beyond the number of hours specified. False if it has not.
    ///  
    pub fn is_aged (d: NaiveDateTime, hours: i64) -> bool {
        let now = chrono::Utc::now().naive_utc();
        if now - d > Duration::hours(hours) {
            return true;
        }
        false
    }

    /// is_equal_to_yyyy_mm_dd_hh_mm_ss()
    /// 
    ///  NaiveDateTime records the milliseconds and nanoseconds, making comparison of them awkward. This utility function
    ///  strips those portions of two objects and performs a true/false equivalency comparison
    /// 
    /// References:
    ///  * https://docs.rs/chrono-wasi/latest/chrono/naive/struct.NaiveDateTime.html
    /// 
    /// Parameters:
    ///  * a: NaiveDateTime, b: NaiveDateTime
    /// 
    /// Returns:
    ///   bool: True if the two NaiveDateTime match, after excluding their milli and nano seconds
    /// 
    pub fn is_equal_to_yyyy_mm_dd_hh_mm_ss(a: NaiveDateTime, b: NaiveDateTime) -> bool {
        let a_no_milliseconds = a.with_nanosecond(0).unwrap();
        let b_no_milliseconds = b.with_nanosecond(0).unwrap();
        //println!("is_equal_to_yyyy_mm_dd_hh_mm_ss()");
        //println!("..a={} == b={}", a, b);
        //println!("..a*={} == b*={}", a_no_milliseconds, b_no_milliseconds);

        return a_no_milliseconds == b_no_milliseconds
    }
}