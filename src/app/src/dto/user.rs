//! Defines a Data Transfer Object for a User
//!
//! CSM500 Project (April - October 2026)
//! Graham Parker (Student ID: 240120522)
//! 

use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime}; 
 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "ID")]
    pub id: i64, // iD BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
    #[serde(rename = "NAME")]
    pub name: String, // NAME VARCHAR(200) NOT NULL, 
    #[serde(rename = "USERNAME")]
    pub user_name: String, // USERNAME VARCHAR(50) UNIQUE NOT NULL, 
    #[serde(rename = "EMAIL")]
    pub email: String, // EMAIL VARCHAR(100),  
    #[serde(rename = "CREATED_AT")]
    pub created_timestamp: NaiveDateTime, // CREATED_AT TIMESTAMP DEFAULT NOW(),
    #[serde(rename = "PASSWORD")]
    pub password: String, //   PASSWORD VARCHAR(30),
} 

impl User {
    /// ### User::New()
    ///    Basic constructor for a new User. Note: used by test cases only
    /// 
    /// #### Parameters:
    /// * id: i64 - numeric id of the user, as represented in the database, or -1 if new/temporary
    /// * name: String - display name of the user
    /// * user_name: String - user account for the user, for authentication
    /// * email: String - email address of the user
    /// * created_timestamp: NaiveDateTime - the date/time the user was created
    /// * password: String - a password, for authentication
    /// 
    /// #### Returns:
    /// * a newly initialized User object
    /// 
    #[allow(dead_code)] 
    pub fn new(id: i64,
                name: String,
                user_name: String,
                email: String,
                created_timestamp: NaiveDateTime,
                password: String
            ) -> Self {
        Self { id,
                name,
                user_name,
                email,
                created_timestamp,
                password
        }
    }
}