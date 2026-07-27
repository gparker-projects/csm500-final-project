
use chrono::{DateTime, Utc};

mod dao{
    /// -------------------------------------------------------------------
    /// Defines a Data Object for a Site
    /// -------------------------------------------------------------------
    /// 
    #[derive(serde::Deserialize)]
    pub struct Site {
        #[serde(rename = "Id")]
        ID: Integer, // D BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
        #[serde(rename = "Name")]
        Name: String, // NAME VARCHAR(200) UNIQUE NOT NULL, 
        #[serde(rename = "Address")]
        Address: String, // ADDRESS VARCHAR(200) NOT NULL, 
        #[serde(rename = "MunicipalName")]
        MunicipalName: String, // MUNICIPAL_NAME VARCHAR(200) NOT NULL, 
        #[serde(rename = "PostalCode")]
        PostalCode: String //   POSTAL_CODE VARCHAR(6) NOT NULL,
    } 

    impl Site {
        /// Basic constructor
        /// 
        pub fn new(ID: Integer,
                Name: String,
                Address: String,
                MunicipalName: String,
                PostalCode: String
                ) -> Self {
            Self { 
                ID,
                Name,
                Address,
                MunicipalName,
                PostalCode
            }
        }
    }
}