/// -------------------------------------------------------------------
/// Defines a Data Transfer Object for a (Patient) Intervention, which represents
/// some form of medical treatment or operation performed/to be performed
/// on a patient.
/// -------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime}; 
use std::fmt;

use crate::constants;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Intervention {
    #[serde(rename = "Id")]
    pub id: i64, // D BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
    #[serde(rename = "encounter_id")]
    pub encounter_id: i64, // INTERVENTION_CODE VARCHAR(10),
    #[serde(rename = "description")]
    pub description: String, // DESCRIPTION VARCHAR(2000),
    #[serde(rename = "notes")]
    pub notes: String, // NOTES VARCHAR(2000),  
    #[serde(rename = "location_id")]
    pub location_id: i64, // LOCATION_ID BIGINT REFERENCES LOCATION (ID),
    #[serde(rename = "users_id")]
    pub users_id: i64, //   USERS_ID BIGINT REFERENCES USERS (ID)
    #[serde(rename = "status_code")]
    pub status_code: String, // really the status description
    #[serde(rename = "intervention_type_id")]
    pub intervention_type_id: i64,
    #[serde(rename = "status_id")]
    pub status_id: i64,
    #[serde(rename = "intervention_type")]
    pub intervention_type: String,
    #[serde(rename = "room_identifier")]
    pub room_identifier: String,
    #[serde(rename = "scheduled_timestamp")]
    pub scheduled_timestamp: Option<NaiveDateTime>, 
    #[serde(rename = "performed_timestamp")]
    pub performed_timestamp: Option<NaiveDateTime>,
} 

impl Intervention {
    /// Basic constructor
    /// 
    pub fn new(id: i64,
               encounter_id: i64,
               description: String,
               notes: String,
               location_id: i64,
               users_id: i64,
               status_code: String, // really the status description
               intervention_type_id: i64,
               status_id: i64,
               intervention_type: String,
               room_identifier: String, 
               scheduled_timestamp: Option<NaiveDateTime>,
               performed_timestamp: Option<NaiveDateTime>
            ) -> Self {    
        Self {
            id,
            encounter_id,
            description,
            notes,
            location_id,
            users_id,
            status_code,
            intervention_type_id,
            status_id,
            intervention_type,
            room_identifier,
            scheduled_timestamp,
            performed_timestamp
        }
    }

    ///
    /// helper method to return the intervention code, which represents a general group for the type of intervention,
    /// in a format that can be easily displayed
    /// 
    pub fn type_description_for_display(&self) -> String{
        return self.intervention_type.clone();
    }

    ///
    /// helper method to return the description of the current status code, in a format that can be easily displayed
    /// 
    pub fn status_for_display(&self) -> String{
        return self.status_code.clone();
    }

    ///
    /// helper method to return the date/time the intervention is/was scheduled to occur, in a format that can be easily displayed
    /// 
    pub fn scheduled_timestamp_for_display(&self) -> String{
        match self.scheduled_timestamp{
            Some(ts) => ts.format(constants::SYSTEM_DATETIME_FORMAT ).to_string(),
            None => "".to_string()
        }       
    }

    ///
    /// helper method to return the date/time the intervention was performed, in a format that can be easily displayed
    /// 
    pub fn performed_timestamp_for_display(&self) -> String{
        match self.performed_timestamp{
            Some(ts) => ts.format(constants::SYSTEM_DATETIME_FORMAT ).to_string(),
            None => "".to_string()
        }        
    }
}

/// Implements a .to_string() for the Intervention 
/// 
/// ref: https://loige.co/how-to-to-string-in-rust/
/// 
impl fmt::Display for Intervention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut results: String = "(intervention Id: ".to_owned() + &self.id.to_string() ;

        results =  results + &", intervention_type: " + &self.intervention_type;
        results =  results + &", status_code: " + &self.status_code+ &")";

        f.write_str(&results)
    }
}