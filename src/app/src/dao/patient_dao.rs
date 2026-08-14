use sqlx::postgres::{PgPoolOptions, PgPool}; 
use sqlx::Row;
use chrono::NaiveDateTime;

use crate::dao::db_query;
use crate::dto::{patient::*, encounter::*, intervention::*, intervention_detail::*};

#[derive(Debug, Clone)]
pub struct PatientWrapper {
    pub patient: Patient,
    pub current_encounter: Encounter,
    pub most_recent_intervention: Intervention,
    pub intervention_detail: Vec<InterventionDetail>
}

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

        PatientDAO {
            connection: db_pool,
        }
    }

    /// Finds and returns the data for a specific patient
    /// 
    pub async fn get_patient_details(&self, _user_id: i64, patient_id: i64) -> Result< Option<Patient>, std::io::Error> {
        let tmp: String = db_query::QRY_SINGLE_PATIENT_DETAILS.to_owned();
        let query = tmp.replace("{}", &patient_id.to_string());

        //println!("get_patient_details Query: {}", query);

        match sqlx::query(&query)
        .fetch_optional(&self.connection)
        .await
        {
            Ok( Some(row) ) => {
                //println!("Building patient {} ", patient_id);
                
                let tmp_pat_id: i64 = row.get("patient_id"); // patient_id
                let tmp_enc_id: i64 = row.get("encounter_id"); // encounter_id
                let tmp_loc_id: i64 = row.get("location_id"); // location_id
                
                let tmp_legal_first_name = row.get("legal_first_name"); //legal_first_name
                let tmp_legal_last_name = row.get("legal_last_name"); // legal_last_name
                let tmp_legal_middle_names = row.get("legal_middle_names");

                let tmp_admit_notes = row.get("admit_notes"); // admission_notes
                let tmp_discharge_notes = row.get("discharge_notes");  // discharge_notes

                let tmp_birthdate: NaiveDateTime = row.get("birthdate"); //tmp_birthdate
                let tmp_admit_timestamp: NaiveDateTime = row.get("admit_timestamp");// admit_timestamp
                let tmp_discharge_timestamp = row.get("discharge_timestamp?");// chrono::NaiveDateTime; 
                let tmp_sin:    i32 = row.get("sin"); // SIN
                let tmp_phn:    i64 = row.get("phn"); // SIN

                Ok(  Some( 
                        Patient {
                            id: tmp_pat_id,
                            encounter_id: tmp_enc_id,
                            legal_first_name:  tmp_legal_first_name, //"DUMMY".to_string(),
                            legal_last_name: tmp_legal_last_name,//"DUMMY".to_string(), 
                            legal_middle_names: tmp_legal_middle_names, //"DUMMY".to_string(),
                            sin: tmp_sin,
                            phn: tmp_phn,
                            birth_date: tmp_birthdate, //Utc::now().naive_utc(), 
                            location_id: tmp_loc_id,
                            admit_timestamp:tmp_admit_timestamp, //Utc::now().naive_utc(), 
                            admit_notes: tmp_admit_notes,//"DUMMY".to_string(), 
                            discharge_timestamp: tmp_discharge_timestamp,//Utc::now().naive_utc(), 
                            discharge_notes: tmp_discharge_notes,//"DUMMY".to_string(), 
                        }
                    )
                )
            }
            Ok(None) => {
                println!("No patient found for: {}", patient_id);
                Ok( None )
            }
            Err(err) => {
                println!("Error on patient for: {} ({})", patient_id, err);
                Ok( None )
            }
        }
    }

    /// Finds and returns any patients that are currently assigned to the user
    /// 
    /// REFs: https://docs.rs/sqlx/latest/sqlx/fn.query_as.html
    ///       https://stackoverflow.com/questions/67243108/mapping-nm-relations-into-vec-using-sqlx
    ///       https://doc.rust-lang.org/std/io/struct.Error.html - for return Error
    /// 
    pub async fn get_patients_at_users_site_no_discharge(&self, user_id: i64, _include_discharged: bool) -> Result< Option< Vec<Patient> >, std::io::Error> {

        let tmp: String = db_query::QRY_ALL_PATIENTS_AT_USERS_SITE_NO_DISCHARGE.to_owned();
        let query = tmp.replace("{}", &user_id.to_string());

        let rows: Vec<(i64, i64, i64, String, String, String,
                        String, String,
                        chrono::NaiveDateTime, chrono::NaiveDateTime, 
                        Option<chrono::NaiveDateTime>,
                        i32, i64
                        )> = sqlx::query_as(&query)
        .fetch_all(&self.connection) 
        .await
        .unwrap_or_default();

        if rows.is_empty() {
            println!("No patients found for user_id: {} [{}]", user_id, rows.len());
            return Ok( Some( Vec::new() ) );
        }
        else{
            //println!("get_patients_at_users_site_no_discharge() -> Loading {} patients",  rows.len());
            let mut results: Vec<Patient> = Vec::with_capacity(rows.len());
            for row in rows {
                let tmp_pat_id: i64 = row.0; // patient_id
                let tmp_enc_id: i64 = row.1; // encounter_id
                let tmp_loc_id: i64 = row.2; // location_id
                
                let tmp_legal_first_name = row.3; //legal_first_name
                let tmp_legal_last_name = row.4; // legal_last_name
                let tmp_legal_middle_names = row.5;

                let tmp_admit_notes = row.6; // admission_notes
                let tmp_discharge_notes = row.7;  // discharge_notes

                let tmp_birthdate: NaiveDateTime = row.8; //tmp_birthdate
                let tmp_admit_timestamp: NaiveDateTime = row.9;// admit_timestamp
                let tmp_discharge_timestamp = row.10;// chrono::NaiveDateTime; 
                
                let tmp_sin:    i32 = row.11; // SIN
                let tmp_phn:    i64 = row.12; // phn

                results.push(
                    Patient {
                        id: tmp_pat_id,
                        encounter_id: tmp_enc_id,
                        legal_first_name:  tmp_legal_first_name, 
                        legal_last_name: tmp_legal_last_name,
                        legal_middle_names: tmp_legal_middle_names, 
                        sin: tmp_sin,
                        phn: tmp_phn,
                        birth_date: tmp_birthdate,
                        location_id: tmp_loc_id,
                        admit_timestamp:tmp_admit_timestamp,
                        admit_notes: tmp_admit_notes,
                        discharge_timestamp: tmp_discharge_timestamp,
                        discharge_notes: tmp_discharge_notes,
                    }
                );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }

        /// Finds and returns the data for a specific patient
    /// 
    /// 
    pub async fn get_admit_patient(&self, _user_id: i64, _patient_id: i64) -> Result< Option<Patient>, std::io::Error> {
        todo!();
    }

    /// Updates the fields of a specific patient
    /// 
    pub async fn update_patient_details(&self, _user_id: i64, _patient_id: i64) -> Result< Option<Patient>, std::io::Error> {
        /*
        
            */
        todo!();
    }



}