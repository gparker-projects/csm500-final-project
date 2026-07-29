
mod dto{
    /// -------------------------------------------------------------------
    /// Defines a Data Transfer Object for a Site
    /// -------------------------------------------------------------------
    /// 
    #[derive(serde::Deserialize)]
    pub struct Site {
        #[serde(rename = "Id")]
        id: u32, // D BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
        #[serde(rename = "Name")]
        name: String, // NAME VARCHAR(200) UNIQUE NOT NULL, 
        #[serde(rename = "Address")]
        address: String, // ADDRESS VARCHAR(200) NOT NULL, 
        #[serde(rename = "MunicipalName")]
        municipal_name: String, // MUNICIPAL_NAME VARCHAR(200) NOT NULL, 
        #[serde(rename = "PostalCode")]
        postal_code: String //   POSTAL_CODE VARCHAR(6) NOT NULL,
    } 

    impl Site {
        /// Basic constructor
        /// 
        pub fn new(id: u32,
                name: String,
                address: String,
                municipal_name: String,
                postal_code: String
                ) -> Self {
            Self { 
                id,
                name,
                address,
                municipal_name,
                postal_code
            }
        }
    }
}