pub mod dao{
    use sqlx::postgres::{PgPoolOptions, PgPool}; 
    use sqlx::Row;
    use std::io::{Error, ErrorKind};
    use crate::dto::user::dto::User;
    use crate::dto::userauthorization::dto::*;

    #[derive(Debug, Clone)]
    pub struct PatientDAO {
        pub connection: PgPool,
    }

    impl PatientDAO {
        /// Creates a new Patient Data Access Object, with a database pool for use by other calls
        /// todo: centralize the db pool connection instead of creating it here
        /// 
        pub async fn new(db_url: &str) -> Self {

            let db_pool = match PgPoolOptions::new()
                .max_connections(5)
                .connect(db_url)
                .await
            {
                Ok(pool) => pool,
                Err(e) => panic!("{}", e),
            };

            AuthObjects {
                connection: db_pool,
            }
        }

        /// Finds and returns any patients that are currently assigned to the user
        /// 
        pub async fn get_assigned_patients(&self, user_id: i64, include_discharged: bool) -> Result< Option<User>, std::io::Error> {
            todo();
            Ok( None )
        }

        /// Finds and returns any patients that are at a facility, regardless of if they are assigned to the user or not
        /// 
        pub async fn get_site_patients(&self, user_id: i64, include_discharged: bool) -> Result< Option<User>, std::io::Error> {
            todo();
            Ok( None )
        }

        /// Finds and returns the data for a specific patient
        /// 
        pub async fn get_patient_details(&self, user_id: i64, patient_id: i64) -> Result< Option<User>, std::io::Error> {
            todo();
            Ok( None )
        }

        /// Updates the fields of a specific patient
        /// 
        pub async fn update_patient_details(&self, user_id: i64, patient_id: i64) -> Result< Option<User>, std::io::Error> {
            todo();
            Ok( None )
        }
    }
}