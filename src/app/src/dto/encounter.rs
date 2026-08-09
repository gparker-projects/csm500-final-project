use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime}; 
use std::fmt;


/// -------------------------------------------------------------------
/// Defines a Data Transfer Object for a (Patient) Encounter, which represents
/// an event whereby a patient has attended the hospital to have one or 
/// more interventions applied to them.
/// -------------------------------------------------------------------
/// 
#[derive(serde::Deserialize)]
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
            is_current_encounter: String
            ) -> Self {
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
}