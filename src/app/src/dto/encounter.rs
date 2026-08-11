/// -------------------------------------------------------------------
/// Defines a Data Transfer Object for a (Patient) Encounter, which represents
/// an event whereby a patient has attended the hospital to have one or 
/// more interventions applied to them.
/// -------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime}; 
use std::fmt;

#[derive(serde::Deserialize)]
#[derive(Clone)]
pub struct Encounter {
    #[serde(rename = "id")]
    pub id: i64, // ID BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
    #[serde(rename = "admit_notes")]
    pub admit_notes: String, // ADMIT_NOTES VARCHAR(2000),
    #[serde(rename = "admit_timestamp")]
    pub admit_timestamp: NaiveDateTime, // ADMIT_TIMESTAMP TIMESTAMP DEFAULT NOW(),
    #[serde(rename = "discharge_notes")]
    pub discharge_notes: String, // DISCHARGE_NOTES VARCHAR(2000),  
    #[serde(rename = "discharge_timestamp")]
    pub discharge_timestamp: Option<NaiveDateTime>, 
    #[serde(rename = "patient_id")]
    pub patient_id: i64,//       PATIENT_ID BIGINT REFERENCES PATIENT (ID),
    #[serde(rename = "encounter_site_name")]
    pub encounter_site_name: String,
    #[serde(rename = "is_current_encounter")]
    pub is_current_encounter: String 
} 

impl Encounter{
    /// Basic constructor
    /// 
    pub fn new(id: i64,
            admit_notes: String,
            admit_timestamp: NaiveDateTime,
            discharge_notes: String,
            discharge_timestamp: Option<NaiveDateTime>,
            patient_id: i64,
            encounter_site_name: String,
            is_current_encounter: String // WARNING: This field is subjective and must be calculated in relation to other Encounters in a set.
            ) -> Self {                  //          It has been included as a datum in the DTO as that is where it is needed/relevant for user display
        Self { 
            id,
            admit_notes,
            admit_timestamp,
            discharge_notes,
            discharge_timestamp,
            patient_id,
            encounter_site_name,
            is_current_encounter
        }
    }

    ///
    /// helper method to return the admit date (entire timestamp) in a format that can be easily displayed
    /// 
    pub fn admit_timestamp_for_display(&self) -> String{
        return self.admit_timestamp.format("%d/%m/%Y %H:%M:%S").to_string();
    }

    ///
    /// helper method to return the discharge date (entire timestamp) in a format that can be easily displayed
    /// 
    pub fn discharge_timestamp_for_display(&self) -> String{
        return self.discharge_timestamp.unwrap().format("%d/%m/%Y %H:%M:%S").to_string();
    }

}

/// Implements a .to_string() for the Intervention 
/// 
/// ref: https://loige.co/how-to-to-string-in-rust/
/// 
impl fmt::Display for Encounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut results: String = "(encounter Id: ".to_owned() + &self.id.to_string() ;      

        results =  results + &", admit_timestamp: " + &self.admit_timestamp_for_display();
        results =  results + &", encounter_site_name: " + &self.encounter_site_name;
        results =  results + &", is_current_encounter: " + &self.is_current_encounter + &")";

        f.write_str(&results)
    }
}