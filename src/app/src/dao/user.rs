use chrono::{DateTime, Utc};

mod dao{
    /// -------------------------------------------------------------------
    /// Defines a Data Object for a User
    /// -------------------------------------------------------------------
    /// 
    #[derive(serde::Deserialize)]
    pub struct User {
        #[serde(rename = "Id")]
        ID: Integer, // D BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
        #[serde(rename = "Name")]
        Name: String, // NAME VARCHAR(200) NOT NULL, 
        #[serde(rename = "UserName")]
        UserName: String, // USERNAME VARCHAR(50) UNIQUE NOT NULL, 
        #[serde(rename = "Email")]
        Email: String, // EMAIL VARCHAR(100),  
        #[serde(rename = "CreatedTimestamp")]
        CreatedTimestamp: DateTime, // CREATED_AT TIMESTAMP DEFAULT NOW(),
        #[serde(rename = "Password")]
        Password: String, //   PASSWORD VARCHAR(30),
    } 

    impl User {
        /// Basic constructor
        /// 
        pub fn new(ID: Integer,
                Name: String,
                UserName: String,
                Email: String,
                CreatedTimestamp: DateTime,
                Password: String
                ) -> Self {
            Self { 
                ID,
                Name,
                UserName,
                Email,
                CreatedTimestamp,
                Password
            }
        }
    }
}