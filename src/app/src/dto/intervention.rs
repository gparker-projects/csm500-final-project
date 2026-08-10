/// -------------------------------------------------------------------
/// Defines a Data Transfer Object for a (Patient) Intervention, which represents
/// some form of medical treatment or operation performed/to be performed
/// on a patient.
/// -------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(serde::Deserialize)]
pub struct Intervention {
    #[serde(rename = "Id")]
    pub id: i64, // D BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
    #[serde(rename = "encounter_id")]
    pub encounter_id: i64, // encounter ID BIGINT 
    #[serde(rename = "intervention_code")]
    pub intervention_code: String, // INTERVENTION_CODE VARCHAR(10),
    #[serde(rename = "description")]
    pub description: String, // DESCRIPTION VARCHAR(2000),
    #[serde(rename = "notes")]
    pub notes: String, // NOTES VARCHAR(2000),  
    #[serde(rename = "location_id")]
    pub location_id: i64, // LOCATION_ID BIGINT REFERENCES LOCATION (ID),
    #[serde(rename = "users_id")]
    pub users_id: i64, //   USERS_ID BIGINT REFERENCES USERS (ID)
    #[serde(rename = "status_code")]
    pub status_code: String, // status_code VARCHAR(10),
} 


impl Intervention {
    /// Basic constructor
    /// 
    pub fn new(id: i64,
               encounter_id: i64,
               intervention_code: String,
               description: String,
               notes: String,
               location_id: i64,
               users_id: i64,
               status_code: String
            ) -> Self {
        Self { 
            id,
            encounter_id,
            intervention_code,
            description,
            notes,
            location_id,
            users_id,
            status_code
        }
    }

    ///
    /// helper method to return the intervention code, which represents a general group for the type of intervention,
    /// in a format that can be easily displayed
    /// 
    pub fn type_description_for_display(&self) -> String{
        return self.intervention_code.clone();
    }

    ///
    /// helper method to return the description of the current status code, in a format that can be easily displayed
    /// 
    pub fn status_for_display(&self) -> String{
        return self.status_code.clone();
    }

    ///
    /// helper method to return the date the intervention is/was scheduled to occur, in a format that can be easily displayed
    /// 
    pub fn scheduled_date_for_display(&self) -> String{
        return "Today()".to_owned();
    }

    ///
    /// helper method to return the date the intervention was performed, in a format that can be easily displayed
    /// 
    pub fn performed_date_for_display(&self) -> String{
        return "Today()".to_owned();
    }
}

/// Implements a .to_string() for the Intervention 
/// 
/// ref: https://loige.co/how-to-to-string-in-rust/
/// 
impl fmt::Display for Intervention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut results: String = "(intervention Id: ".to_owned() + &self.id.to_string() ;

        results =  results + &", intervention_code: " + &self.intervention_code;
        results =  results + &", status_code: " + &self.status_code + &")";

        f.write_str(&results)
    }
}