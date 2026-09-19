//! # Defines a Data Transfer Object for a Patient
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc}; 
use std::fmt;
use crate::constants;
use crate::ui::data_forms::AdmitDataForm;
use crate::dto::convert_utils::ConvertUtils;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
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
    #[serde(rename = "phn")]
    pub phn: i64, // PHN BIGINT UNIQUE, 
    #[serde(rename = "birthdate")]
    pub birth_date: NaiveDateTime, // BIRTHDATE TIMESTAMP,
    #[serde(rename = "location_id")]
    pub location_id: i64, // LOCATION_ID BIGINT REFERENCES LOCATION (ID),
    #[serde(rename = "location_short_name")]
    pub location_short_name: String, // SHORT_NAME VARCHAR2(16)

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

    /// ### new()
    ///    Basic constructor for Patient. Note: used by test cases only
    /// 
    /// #### Parameters:
    /// * id (i64): the id of the Patient
    /// * encounter_id (i64): the id of a Encounter related to the Patient
    /// * legal_first_name (String): the legal first name of the Patient
    /// * legal_last_name (String): the legal last name of the Patient
    /// * legal_middle_names (String): the legal middle names of the Patient
    /// * phn (64): the Personal Health Number (PHN) of the Patient
    /// * birth_date (NaiveDateTime): the date of birth of the Patient
    /// * location_id (i64): the current location of the Patient
    /// * location_short_name (String): a short descriptive name of the current location of the Patient

    /// * admit_timestamp (NaiveDateTime): the date/time the Patient was Admitted, from the Encounter record
    /// * admit_notes (String): the comments related to Admission of the Patient, from the Encounter record
    /// * discharge_timestamp Option<NaiveDateTime>: the date/time the Patient was Dischared (if available, from the Encounter record
    /// * discharge_notes (String): the comments related to Discharge of the Patient, from the Encounter record
    /// 
    /// #### Returns:
    /// * Patient: a fully constructed Patient object
    /// 
    #[allow(dead_code)] 
    pub fn new(id: i64,
                encounter_id: i64,
                legal_first_name: String,
                legal_last_name: String,
                legal_middle_names: String,
                phn: i64,
                birth_date: NaiveDateTime,
                location_id: i64,
                location_short_name: String,

                admit_timestamp: NaiveDateTime,
                admit_notes: String,
                discharge_timestamp: Option<NaiveDateTime>,
                discharge_notes: String
            ) -> Self {
        Self { 
                id,
                encounter_id,
                legal_first_name,
                legal_last_name,
                legal_middle_names,
                phn,
                birth_date,
                location_id,
                location_short_name,

                admit_timestamp,
                admit_notes,
                discharge_timestamp,
                discharge_notes
        }
    }

    /// ### birth_date_for_display()
    ///    Accessor method to return the birth date (date portion only) in a format that can be easily displayed
    /// 
    /// #### Returns:
    ///   String: a string representation of the birth date, in YYYY-MON-DD format
    /// 
    pub fn birth_date_for_display(&self) -> String{
        self.birth_date.format("%Y-%b-%d").to_string()
    }

    /// ### phn_to_string()
    ///    Accessor method to return the Personal Health Number (PHN), in a format that can be easily displayed
    /// 
    /// #### Returns:
    ///   String: a string representation of the PHN, in 9######### format. If not valid (-1), an empty string is returned.
    /// 
    pub fn phn_to_string(&self) -> String{
        match self.phn {
            constants::INVALID_OTHER_ID => String::new(),
            _ => self.phn.to_string(),
        }
    }

    /// ### admit_timestamp_for_display()
    ///    Accessor method to return the Admit Timestamp (date and time) in a format that can be easily displayed
    /// 
    /// #### Returns:
    ///   String: a string representation of the Admit Timestamp, in YYYY-MON-DD HH:MM:SS format
    /// 
    pub fn admit_timestamp_for_display(&self) -> String{
        self.admit_timestamp.format("%Y-%b-%d %H:%M:%S").to_string()
    }

    /// ### age()
    ///    Accessor method to return the age of the patient, in relation to the current date/time (UTC)
    /// 
    /// #### Returns:
    ///   String: a string representation of the age, in years
    /// 
    pub fn age(&self) -> String{
      ((Utc::now().naive_utc() - self.birth_date).num_days() / 365).to_string()
    }

    /// ### to_patient()
    ///    Converts an AdmitDataForm into a Patient DTO
    /// 
    /// #### Parameters:
    /// * frm (AdmitDataForm): the AdmitDataForm to construct the patient object
    /// 
    /// #### Returns:
    /// * Patient: a fully constructed patient
    /// 
    pub fn to_patient(frm: AdmitDataForm) -> Patient{

        // convert birthdate, if possible
        let tmp_birthdate = match NaiveDateTime::parse_from_str(&frm.birthdate, "%Y-%m-%d %H:%M:%S"){
            Ok(p) => p,
            Err(_e) => {
                //println!("ConvertUtils::to_naivedatetime()"); 
                //println!("..String provided: {}", frm.birthdate.clone()); 
                //println!("..Date format error: {}", e); 
                Utc::now().naive_utc()
            },
        };

        Patient {
                id: ConvertUtils::to_i64(frm.patient_id),
                encounter_id: ConvertUtils::to_i64(frm.encounter_id),
                legal_first_name: frm.patient_first_name,
                legal_last_name: frm.patient_last_name,
                legal_middle_names: frm.patient_middle_name,
                phn: ConvertUtils::to_i64(frm.phn),
                birth_date: tmp_birthdate,
                location_id: ConvertUtils::to_i64(frm.location_id),
                location_short_name: String::new(), // /AdmitDataForm does not have a location_short_name, so default
                admit_timestamp: Utc::now().naive_utc(), // patients admit "now" when they are created
                admit_notes: frm.admit_notes,
                discharge_timestamp: None, // AdmitDataForm does not have a Discharge Timestamp, so default
                discharge_notes: String::new() // AdmitDataForm does not have a Discharge Notes, so default
        }
    }
}

/// ### Implements a .to_string() for the Patient object
/// 
/// ### References:
///   https://loige.co/how-to-to-string-in-rust/
/// 
/// #### Returns:
/// * Patient: a brief, identifying String describing the patient, in the format:
///    (patient Id: #, \nlegal_first_name: <first_name> \nlegal_last_name: <last_name> )
/// 
impl fmt::Display for Patient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut results: String = "(patient Id: ".to_owned() + &self.id.to_string() ;

        results =  results + &"\nlegal_first_name: " + &self.legal_first_name;
        results =  results + &"\nlegal_last_name: " + &self.legal_last_name + &")";

        f.write_str(&results)
    }
}