
mod dto{
    use chrono::{DateTime, Local};
    use serde::{Deserialize, Serialize};

    /// -------------------------------------------------------------------
    /// Defines a Data Transfer Object for a User
    /// -------------------------------------------------------------------
    /// 
    #[derive(serde::Deserialize)]
    pub struct User {
        #[serde(rename = "Id")]
        id: u32, // D BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
        #[serde(rename = "Name")]
        name: String, // NAME VARCHAR(200) NOT NULL, 
        #[serde(rename = "UserName")]
        user_name: String, // USERNAME VARCHAR(50) UNIQUE NOT NULL, 
        #[serde(rename = "Email")]
        email: String, // EMAIL VARCHAR(100),  
        #[serde(rename = "CreatedTimestamp")]
        created_timestamp: DateTime<Local>, // CREATED_AT TIMESTAMP DEFAULT NOW(),
        #[serde(rename = "Password")]
        password: String, //   PASSWORD VARCHAR(30),
    } 

    impl User {
        /// Basic constructor
        /// 
        pub fn new(id: u32,
                name: String,
                user_name: String,
                email: String,
                created_timestamp: DateTime<Local>,
                password: String
                ) -> Self {
            Self { 
                id,
                name,
                user_name,
                email,
                created_timestamp,
                password
            }
        }
    }
}