
use chrono::{DateTime, Utc};

mod dto{
    /// -------------------------------------------------------------------
    /// Defines a Data Transfer Object for a Location, which is a very specific and 
    /// identifiable physical position within the hospital. This is separated
    /// from the Site, which is a higher level and more generalized area.
    /// -------------------------------------------------------------------
    /// 
    #[derive(serde::Deserialize)]
    pub struct Location {
        #[serde(rename = "Id")]
        ID: Integer, // ID BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
        #[serde(rename = "Name")]
        Name: String, // NAME VARCHAR(200) UNIQUE NOT NULL, 
        #[serde(rename = "Building")]
        Building: String, // BUILDING VARCHAR(200), 
        #[serde(rename = "Floor")]
        Floor: String, // FLOOR VARCHAR(200), 
        #[serde(rename = "Wing")]
        Wing: String, //  WING VARCHAR(200),
        #[serde(rename = "Notes")]
        Notes: String, //   NOTES VARCHAR(2000),
        #[serde(rename = "SiteId")]
        SiteId: Integer, //  SITE_ID BIGINT NOT NULL REFERENCES SITE (ID),
    } 

    impl Location {
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