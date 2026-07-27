
use chrono::{DateTime, Utc};

mod dao{
    /// -------------------------------------------------------------------
    /// Defines a Data Object for a Patient
    /// -------------------------------------------------------------------
    /// 
    #[derive(serde::Deserialize)]
    pub struct Patient {
        #[serde(rename = "Id")]
        ID: Integer, // D BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
        #[serde(rename = "LegalFirstName")]
        LegalFirstName: String, // LEGAL_FIRST_NAME VARCHAR(100), 
        #[serde(rename = "LegalLastName")]
        LegalLastName: String, // LEGAL_LAST_NAME VARCHAR(100), 
        #[serde(rename = "LegalMiddleNames")]
        LegalMiddleNames: String, // LEGAL_MIDDLE_NAMES VARCHAR(100), 
        #[serde(rename = "SIN")]
        SIN: Integer, // SIN NUMERIC(7) UNIQUE, 
        #[serde(rename = "BirthDate")]
        BirthDate: DateTime, // BIRTHDATE TIMESTAMP,
        #[serde(rename = "LocationID")]
        LocationID: Integer, // LOCATION_ID BIGINT REFERENCES LOCATION (ID),
    }

    impl Patient {
        /// Basic constructor
        /// 
        pub fn new(ID: Integer,
                LegalFirstName: String,
                LegalLastName: String,
                LegalMiddleNames: String,
                SIN: Integer,
                BirthDate: DateTime,
                LocationID: Integer
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