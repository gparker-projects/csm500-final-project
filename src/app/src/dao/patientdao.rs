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

        /// Finds and returns the data for a specific patient
        /// 
        pub async fn get_admit_patient(&self, user_id: i64, patient_id: i64) -> Result< Option<User>, std::io::Error> {
            todo();

            /**
             * data for patient, encounter, patient_care_assignment
             */
            Ok( None )
        }

        /// Finds and returns any patients that are currently assigned to the user
        /// 
        pub async fn get_assigned_patients(&self, user_id: i64, include_discharged: bool) -> Result< Option<User>, std::io::Error> {
            todo();
            /*
            SELECT p.id "patient_id", e.id "encounter_id", e.location_id, legal_first_name, legal_last_name, legal_middle_names, sin, birthdate, admit_timestamp, admission_notes, discharge_notes, discharge_timestamp
            FROM patient p
            join encounter e on p.id = e.patient_id
            where location_id = (
                select l.id
                from location l
                where site_id in (
                select site_id
                from user_permission up
                where users_id = 2
                    and up.site_id = l.site_id)
            )
            */
            Ok( None )
        }

        /// Finds and returns any patients that are at a facility, regardless of if they are assigned to the user or not
        /// 
        pub async fn get_site_patients(&self, user_id: i64, include_discharged: bool) -> Result< Option<User>, std::io::Error> {
            todo();
            /*
                SELECT p.id "patient_id", e.id "encounter_id", e.location_id, legal_first_name, legal_last_name, legal_middle_names, sin, birthdate, admit_timestamp, admit_notes, discharge_notes, discharge_timestamp
                FROM patient p
                join encounter e on p.id = e.patient_id
                where location_id in (
                    select l.id
                    from location l
                    where site_id in (
                    select site_id
                    from user_permission up
                    where users_id = 2
                        and up.site_id = l.site_id)
                )
            */
            Ok( None )
        }

        /// Finds and returns the data for a specific patient
        /// 
        pub async fn get_patient_details(&self, user_id: i64, patient_id: i64) -> Result< Option<User>, std::io::Error> {
            todo();

            /**
             * select * from patient, encounter
             */
            Ok( None )
        }

        /// Updates the fields of a specific patient
        /// 
        pub async fn update_patient_details(&self, user_id: i64, patient_id: i64) -> Result< Option<User>, std::io::Error> {
            /*
            
             */
            todo();
            Ok( None )
        }

        /// Finds and returns all interventions based on an encounter
        /// 
        pub async fn get_interventions(&self, encounter_id: i64) -> Result< Option<User>, std::io::Error> {
            todo();
            /*
            select id "intervention_id", intervention_code, description, notes, location_id, users_id, status_code
            from intervention
            where encounter_id = 3
             */
            Ok( None )
        }

        /// Finds and returns all encounters based on an encounter
        /// 
        pub async fn get_encounters(&self, encounter_id: i64) -> Result< Option<User>, std::io::Error> {
            todo();
            /*
            select id "intervention_id", intervention_code, description, notes, location_id, users_id, status_code
            from intervention
            where encounter_id = 3
             */
            Ok( None )
        }

    }
}