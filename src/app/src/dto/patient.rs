
mod dto{
    use chrono::{DateTime, Local};

    /// -------------------------------------------------------------------
    /// Defines a Data Transfer Object for a Patient
    /// -------------------------------------------------------------------
    /// 
    #[derive(serde::Deserialize)]
    pub struct Patient {
        #[serde(rename = "Id")]
        ID: u32, // D BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
        #[serde(rename = "LegalFirstName")]
        LegalFirstName: String, // LEGAL_FIRST_NAME VARCHAR(100), 
        #[serde(rename = "LegalLastName")]
        LegalLastName: String, // LEGAL_LAST_NAME VARCHAR(100), 
        #[serde(rename = "LegalMiddleNames")]
        LegalMiddleNames: String, // LEGAL_MIDDLE_NAMES VARCHAR(100), 
        #[serde(rename = "SIN")]
        SIN: u32, // SIN NUMERIC(7) UNIQUE, 
        #[serde(rename = "BirthDate")]
        BirthDate: DateTime<Local>, // BIRTHDATE TIMESTAMP,
        #[serde(rename = "LocationID")]
        LocationID: u32, // LOCATION_ID BIGINT REFERENCES LOCATION (ID),
    }

    impl Patient {
        /// Basic constructor
        /// 
        pub fn new(ID: u32,
                LegalFirstName: String,
                LegalLastName: String,
                LegalMiddleNames: String,
                SIN: u32,
                BirthDate: DateTime<Local>,
                LocationID: u32
                ) -> Self {
            Self { 
                ID,
                LegalFirstName,
                LegalLastName,
                LegalMiddleNames,
                SIN,
                BirthDate,
                LocationID
            }
        }
    }
}