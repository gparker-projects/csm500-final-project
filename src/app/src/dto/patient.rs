
mod dto{
    use chrono::{DateTime, Local};

    /// -------------------------------------------------------------------
    /// Defines a Data Transfer Object for a Patient
    /// -------------------------------------------------------------------
    /// 
    #[derive(serde::Deserialize)]
    pub struct Patient {
        #[serde(rename = "Id")]
        id: u32, // D BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
        #[serde(rename = "LegalFirstName")]
        legal_first_name: String, // LEGAL_FIRST_NAME VARCHAR(100), 
        #[serde(rename = "LegalLastName")]
        legal_last_name: String, // LEGAL_LAST_NAME VARCHAR(100), 
        #[serde(rename = "LegalMiddleNames")]
        legal_middle_names: String, // LEGAL_MIDDLE_NAMES VARCHAR(100), 
        #[serde(rename = "SIN")]
        sin: u32, // SIN NUMERIC(7) UNIQUE, 
        #[serde(rename = "BirthDate")]
        birth_date: DateTime<Local>, // BIRTHDATE TIMESTAMP,
        #[serde(rename = "LocationID")]
        location_id: u32, // LOCATION_ID BIGINT REFERENCES LOCATION (ID),
    }

    impl Patient {
        /// Basic constructor
        /// 
        pub fn new(id: u32,
                legal_first_name: String,
                legal_last_name: String,
                legal_middle_names: String,
                sin: u32,
                birth_date: DateTime<Local>,
                location_id: u32
                ) -> Self {
            Self { 
                id,
                legal_first_name,
                legal_last_name,
                legal_middle_names,
                sin,
                birth_date,
                location_id
            }
        }
    }
}