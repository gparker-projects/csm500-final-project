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
        id: u32, // D BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
        #[serde(rename = "InterventionCode")]
        intervention_code: String, // INTERVENTION_CODE VARCHAR(10),
        #[serde(rename = "Description")]
        description: String, // DESCRIPTION VARCHAR(2000),
        #[serde(rename = "Notes")]
        notes: String, // NOTES VARCHAR(2000),  
        #[serde(rename = "LocationId")]
        location_id: u32, // LOCATION_ID BIGINT REFERENCES LOCATION (ID),
        #[serde(rename = "UsersId")]
        users_id: u32 //   USERS_ID BIGINT REFERENCES USERS (ID)
    } 


    impl Intervention {
        /// Basic constructor
        /// 
        pub fn new(id: u32,
                intervention_code: String,
                description: String,
                notes: String,
                location_id: u32,
                users_id: u32
                ) -> Self {
            Self { 
                id,
                intervention_code,
                description,
                notes,
                location_id,
                users_id
            }
        }
    }
}