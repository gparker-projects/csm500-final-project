
pub mod dto{
    use serde::{Deserialize, Serialize};
    use chrono::{NaiveDateTime, Local};

    /// -------------------------------------------------------------------
    /// Defines a Data Transfer Object that contains permissions and departments
    /// associated with a user.
    /// -------------------------------------------------------------------
    /// 
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct UserAuthorization {
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

    impl UserAuthorization {
        /// Basic constructor
        /// 
        pub fn new(id: i64,
                name: String,
                user_name: String,
                email: String,
                created_timestamp: NaiveDateTime,
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