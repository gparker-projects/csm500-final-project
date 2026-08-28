//! #Defines a Data Transfer Object for a Feature Preference, which 
//!  stores information related to a feature which a user has used
//!  at least once, but likely many times.
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime}; 

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct FeaturePreference {
    #[serde(rename = "Feature Preference Id")]
    pub id: i64, 
    #[serde(rename = "Display Order")]
    pub display_order: i64, 
    #[serde(rename = "weight")]
    pub weight: i64,
    #[serde(rename = "Calculation Date")]
    pub calculation_date: NaiveDateTime, 
    #[serde(rename = "User Id")]
    pub users_id: i64, 
    #[serde(rename = "Department Id")]
    pub department_id: i64, 
    #[serde(rename = "Feature Id")]
    pub feature_id: i64, 
}		

impl FeaturePreference {
    pub fn new(id: i64,
               display_order: i64,
               weight: i64,
               calculation_date: NaiveDateTime,
               users_id: i64,
               department_id: i64,
               feature_id: i64
            ) -> Self {
        Self { id,
               display_order,
               weight,
               calculation_date,
               users_id,
               department_id,
               feature_id
        }
    }
}