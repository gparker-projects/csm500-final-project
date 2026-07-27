use chrono::{DateTime, Utc};

mod dto{
    /// -------------------------------------------------------------------
    /// Defines a Data Transfer Object for a (Patient) Intervention, which represents
    /// some form of medical treatment or operation performed/to be performed
    /// on a patient.
    /// -------------------------------------------------------------------
    /// 
    #[derive(serde::Deserialize)]
    pub struct Intervention {
        #[serde(rename = "Id")]
        ID: Integer, // D BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
        #[serde(rename = "InterventionCode")]
        InterventionCode: String, // INTERVENTION_CODE VARCHAR(10),
        #[serde(rename = "Description")]
        Description: String, // DESCRIPTION VARCHAR(2000),
        #[serde(rename = "Notes")]
        Notes: String, // NOTES VARCHAR(2000),  
        #[serde(rename = "LocationId")]
        LocationId: Integer, // LOCATION_ID BIGINT REFERENCES LOCATION (ID),
        #[serde(rename = "UsersId")]
        UsersId: Integer //   USERS_ID BIGINT REFERENCES USERS (ID)
    } 


    impl Intervention {
        /// Basic constructor
        /// 
        pub fn new(Id: Integer,
                InterventionCode: String,
                Description: String,
                Notes: String,
                LocationId: Integer,
                UsersId: Integer
                ) -> Self {
            Self { 
                Id,
                InterventionCode,
                Description,
                Notes,
                LocationId,
                UsersId
            }
        }
    }
}