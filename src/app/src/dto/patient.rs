use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc}; 
use std::fmt;

/// -------------------------------------------------------------------
/// Defines a Data Transfer Object for a Patient
/// -------------------------------------------------------------------
/// 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Patient {
    // fields from the PATIENT table
    #[serde(rename = "Id")]
    pub id: i64, // patient_id
    #[serde(rename = "encounter_id")]
    pub encounter_id: i64, // patient_id
    #[serde(rename = "legal_first_name")]
    pub legal_first_name: String, // LEGAL_FIRST_NAME VARCHAR(100), 
    #[serde(rename = "legal_last_name")]
    pub legal_last_name: String, // LEGAL_LAST_NAME VARCHAR(100), 
    #[serde(rename = "legal_middle_names")]
    pub legal_middle_names: String, // LEGAL_MIDDLE_NAMES VARCHAR(100), 
    #[serde(rename = "sin")]
    pub sin: i32, // SIN INTEGER UNIQUE, 
    #[serde(rename = "phn")]
    pub phn: i64, // PHN BIGINT UNIQUE, 
    #[serde(rename = "birthdate")]
    pub birth_date: NaiveDateTime, // BIRTHDATE TIMESTAMP,
    #[serde(rename = "location_id")]
    pub location_id: i64, // LOCATION_ID BIGINT REFERENCES LOCATION (ID),

    // fields from the ENCOUNTER table
    #[serde(rename = "admit_timestamp")]
    pub admit_timestamp: NaiveDateTime, 
    #[serde(rename = "admit_notes")]
    pub admit_notes: String,
    #[serde(rename = "discharge_timestamp")]
    pub discharge_timestamp: Option<NaiveDateTime>, 
    #[serde(rename = "discharge_notes")]
    pub discharge_notes: String
}

impl Patient {
    /// Basic constructor
    /// 
    pub fn new(id: i64,
                encounter_id: i64,
                legal_first_name: String,
                legal_last_name: String,
                legal_middle_names: String,
                sin: i32,
                phn: i64,
                birth_date: NaiveDateTime,
                location_id: i64,

                admit_timestamp: NaiveDateTime,
                admit_notes: String,
                discharge_timestamp: Option<NaiveDateTime>,
                discharge_notes: String,
            ) -> Self {
        Self { 
                id,
                encounter_id,
                legal_first_name,
                legal_last_name,
                legal_middle_names,
                sin,
                phn,
                birth_date,
                location_id,

                admit_timestamp,
                admit_notes,
                discharge_timestamp,
                discharge_notes
        }
    }

    ///
    /// accessor method to return the birth date (date portion only) in a format that can be easily displayed
    /// 
    pub fn birth_date_for_display(&self) -> String{
        self.birth_date.format("%Y-%b-%d").to_string()
    }

    ///
    /// accessor method to return the birth date (date portion only) in a format that can be easily displayed
    /// 
    pub fn get_phn(&self) -> String{
        self.phn.to_string()
    }

    ///
    /// accessor method to return the birth date (date and time) in a format that can be easily displayed
    /// 
    pub fn admit_timestamp_for_display(&self) -> String{
        self.admit_timestamp.format("%Y-%b-%d %H:%M:%S").to_string()
    }

    ///
    /// accessor method to return the discharge date (date and time) in a format that can be easily displayed
    /// 
    pub fn discharge_timestamp_for_display(&self) -> String{
        self.discharge_timestamp.unwrap().format("%Y-%b-%d %H:%M:%S").to_string()
    }

    ///
    /// accessor method to return the birth date (date portion only) in a format that can be easily displayed
    /// 
    pub fn age(&self) -> String{
      ((Utc::now().naive_utc() - self.birth_date).num_days() / 365).to_string()
    }

}

/// Implements a .to_string() for the Patient 
/// 
/// ref: https://loige.co/how-to-to-string-in-rust/
/// 
impl fmt::Display for Patient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut results: String = "(patient Id: ".to_owned() + &self.id.to_string() ;

        results =  results + &"\nlegal_first_name: " + &self.legal_first_name;
        results =  results + &"\nlegal_last_name: " + &self.legal_last_name + &")";

        f.write_str(&results)
    }
}