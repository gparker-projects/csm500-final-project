
pub mod dto{
    use serde::{Deserialize, Serialize};
    use chrono::{NaiveDateTime}; //, Local};

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
        pub sin: i32, // SIN NUMERIC(9) UNIQUE, 
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
                   birth_date,
                   location_id,

                   admit_timestamp,
                   admit_notes,
                   discharge_timestamp,
                   discharge_notes
            }
        }
    }
}