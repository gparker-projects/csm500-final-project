//! Defines a Data Transfer Object for an Intervention Detail, which represents
//! a measure taken or other facet of care performed as part of an Intervention.
//!
//!  CSM500 Project (April - October 2026)
//!  Graham Parker (Student ID: 240120522)

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::constants;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct InterventionDetail {
    #[serde(rename = "Id")]
    pub id: i64, // ID BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
    #[serde(rename = "intervention_id")]
    pub intervention_id: i64, // INTERVENTION_ID BIGINT REFERENCES INTERVENTION (ID) NOT NULL,
    #[serde(rename = "type_id")]
    pub type_id: i64, // TYPE_ID BIGINT REFERENCES COMMON_REFERENCE_TYPE (ID) NOT NULL,
    #[serde(rename = "value")]
    pub value: String, // VALUE VARCHAR(100) NOT NULL,
    #[serde(rename = "notes")]
    pub notes: String, // NOTES VARCHAR(2000),  
    #[serde(rename = "entry_timestamp")]
    pub entry_timestamp: NaiveDateTime,    // ENTRY_TIMESTAMP TIMESTAMP

    // joined fields from COMMON_REFERENCE_TYPE
    #[serde(rename = "intervention_type")]
    pub intervention_type: String, // NOTES VARCHAR(2000),  
} 

impl InterventionDetail {
    /// Basic constructor
    /// 
    /// Note: used by test cases only
    /// 
    #[allow(dead_code)] 
    pub fn new(id: i64,
               intervention_id: i64,
               type_id: i64,
               value: String, 
               notes: String, 
               entry_timestamp: NaiveDateTime,
               intervention_type: String
            ) -> Self {
        Self { 
               id,
               intervention_id,
               type_id,
               value,
               notes,
               entry_timestamp,
               intervention_type
        }
    }

    ///
    /// helper method to return the entry_timestamp (date and time) in a format that can be easily displayed
    ///
    pub fn entry_timestamp_for_display(&self) -> String{
        self.entry_timestamp.format(constants::SYSTEM_DATETIME_FORMAT).to_string()
    }

    pub fn type_name(&self) -> String{
        self.intervention_type.clone()
    }
}

/// Implements a .to_string() for the Intervention Detail 
/// 
impl fmt::Display for InterventionDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut results: String = "(InterventionDetail Id: ".to_owned() + &self.id.to_string() ;

        results =  results + &", intervention_type: " + &self.intervention_type;
        results =  results + &", value: " + &self.value + &")";

        f.write_str(&results)
    }
}