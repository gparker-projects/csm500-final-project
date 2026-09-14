use chrono::{NaiveDateTime, Duration, Utc}; 

use crate::constants;

pub struct ConvertUtils;

impl ConvertUtils {
    pub fn to_i64(s: String) -> i64 {
        match s.parse::<i64>(){
            Ok(p) => p,
            Err(_) => constants::INVALID_OTHER_ID
        }
    }

    pub fn to_naivedatetime(s: String) -> NaiveDateTime {
        match s.parse::<NaiveDateTime>(){
            Ok(p) => p,
            Err(_) => Utc::now().naive_utc(),
        }
    }

    pub fn is_aged (d: NaiveDateTime, hours: i64) -> bool {
        let now = chrono::Utc::now().naive_utc();
        if now - d > Duration::hours(hours) {
            return true;
        }
        false
    }

    /*fn valid_date(value: &str) -> Result<(), ValidationError> {
        NaiveDateTime::parse_from_str(value, "%Y-%m-%d")
          .map_err(|_| ValidationError::new("invalid_date_format"))?;
        Ok(())
    }*/
}