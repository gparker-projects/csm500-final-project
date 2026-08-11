use sqlx::postgres::{PgPoolOptions, PgPool}; 
use sqlx::Row;
use std::io::{Error, ErrorKind};
use chrono::NaiveDateTime;
use crate::dto::patient::*;
use crate::dto::intervention::*;
use crate::dto::encounter::*;

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
        let query = format!(r##"SELECT p.id "patient_id", e.id "encounter_id", e.location_id "location_id", legal_first_name, legal_last_name, COALESCE(legal_middle_names, '') as "legal_middle_names",
                                                COALESCE(admit_notes, '') as "admit_notes", COALESCE(discharge_notes, '') as "discharge_notes",
                                                birthdate, admit_timestamp,
                                                discharge_timestamp as "discharge_timestamp?",
                                                sin, phn
                                            FROM patient p
                                            join encounter e on p.id = e.patient_id
                                            where patient_id = {}"##, patient_id);

        //println!("get_patient_details Query: {}", query);

        match sqlx::query(&query)
        .fetch_optional(&self.connection)
        .await
        {
            Ok( Some(row) ) => {
                println!("Building patient {} ", patient_id);
                
                let tmp_pat_id: i64 = row.get("patient_id"); // patient_id
                let tmp_enc_id: i64 = row.get("encounter_id"); // encounter_id
                let tmp_loc_id: i64 = row.get("location_id"); // location_id
                
                let tmp_legal_first_name = row.get("legal_first_name"); //legal_first_name
                let tmp_legal_last_name = row.get("legal_last_name"); // legal_last_name
                let tmp_legal_middle_names = row.get("legal_middle_names");

                let tmp_admit_notes = row.get("admit_notes"); // admission_notes
                let tmp_discharge_notes = row.get("discharge_notes");  // discharge_notes

                let tmp_birthdate: chrono::NaiveDateTime = row.get("birthdate"); //tmp_birthdate
                let tmp_admit_timestamp: chrono::NaiveDateTime = row.get("admit_timestamp");// admit_timestamp
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
    pub async fn get_assigned_patients(&self, user_id: i64, _include_discharged: bool) -> Result< Option< Vec<Patient> >, std::io::Error> {
        let query = format!(r##"SELECT p.id, e.id, e.location_id, legal_first_name, legal_last_name, COALESCE(legal_middle_names, '') as "legal_middle_names",
                                        COALESCE(admit_notes, '') as "admit_notes", COALESCE(discharge_notes, '') as "discharge_notes",
                                        birthdate, admit_timestamp,
                                        discharge_timestamp as "discharge_timestamp?",
                                        sin, phn
                                    FROM patient p
                                    join encounter e on p.id = e.patient_id
                                    where location_id in (
                                        select l.id
                                        from location l
                                        where site_id in (
                                        select site_id
                                        from user_permission up
                                        where users_id = {}
                                            and up.site_id = l.site_id)  )"##, user_id);
        //println!("get_assigned_patients Query: {}", query);

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
            println!("Loading {} patients",  rows.len());
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

                let tmp_birthdate: chrono::NaiveDateTime = row.8; //tmp_birthdate
                let tmp_admit_timestamp: chrono::NaiveDateTime = row.9;// admit_timestamp
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

    /// Finds and returns all encounters based on an encounter
    /// 
    pub async fn get_encounters(&self, patient_id: i64) -> Result< Option< Vec<Encounter> >, std::io::Error> {
        let query = format!(r##"
                                    select e.id "encounter_id", admit_timestamp,
                                        discharge_timestamp as "discharge_timestamp?",
                                        COALESCE(admit_notes, '') as "admit_notes",
                                        COALESCE(discharge_notes, '') as "discharge_notes",
                                        s.name "encounter_site_name",
                                        CASE WHEN discharge_timestamp is null then 'Y' else 'N' end as "current_encounter"
                                    from encounter e
                                    join location l on e.location_id = l.id
                                    join site s on s.id = l.site_id
                                    where patient_id = {}
                                    order by admit_timestamp desc
                                    "##, patient_id);
        //println!("get_encounters Query: {}", query);

        let rows: Vec<(i64, // encounter_id
                       NaiveDateTime, // admit_timestamp
                       Option<chrono::NaiveDateTime>, // discharge_timestamp
                       String, String, // admit and discharge notes
                       String, // encounter_site_name
                       String  // current_encounter
                      )> = sqlx::query_as(&query)
        .fetch_all(&self.connection) 
        .await
        .unwrap_or_default();

        if rows.is_empty() {
            println!("No encounters found for patient_id: {} [{}]", patient_id, rows.len());
            return Ok( Some( Vec::new() ) );
        }
        else{
            println!("Loading {} encounters",  rows.len());
            let mut results: Vec<Encounter> = Vec::with_capacity(rows.len());
            for row in rows {
                let tmp_enc_id: i64 = row.0; // encounter_id
                let tmp_admit_timestamp: NaiveDateTime = row.1;// admit_timestamp
                let tmp_discharge_timestamp: Option<NaiveDateTime> = row.2;
                let tmp_admit_notes = row.3; // admission_notes
                let tmp_discharge_notes = row.4;  // discharge_notes
                let encounter_site_name = row.5; // admission_notes
                let is_current_encounter = row.6; // admission_notes

                results.push(
                    Encounter {
                        id: tmp_enc_id,
                        admit_notes: tmp_admit_notes,
                        admit_timestamp: tmp_admit_timestamp, 
                        discharge_notes: tmp_discharge_notes,
                        discharge_timestamp: tmp_discharge_timestamp,
                        patient_id: patient_id,
                        encounter_site_name: encounter_site_name,
                        is_current_encounter: is_current_encounter,
                    }
                );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }


    /// Finds and returns any patients that are at a facility, regardless of if they are assigned to the user or not
    ///
    pub async fn get_site_patients(&self, _user_id: i64, _include_discharged: bool) -> Result< Option<Patient>, std::io::Error> {
        todo!();
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

        pub async fn get_encounter(&self, _encounter_id: i64) -> Result< Option<Encounter>, std::io::Error> {
            todo!();
            /*
            select id "intervention_id", intervention_code, description, notes, location_id, users_id, status_code
            from intervention
            where encounter_id = 3
                */
        }

    /// Finds and returns all interventions based on an encounter
    /// 
    pub async fn get_interventions(&self, encounter_id: i64) -> Result< Option< Vec<Intervention> >, std::io::Error> {

  let query = format!(r##"select id "intervention_id", encounter_id, intervention_code,
                                  description, notes,
                                  location_id, users_id, status_code
                                  from intervention
                                  where encounter_id = {}"##, encounter_id);
        //println!("get_user_permissions Query: {}", query);

        let rows: Vec<(i64, i64, String,
                       String, String,
                       i64, i64, String
                        )> = sqlx::query_as(&query)
        .fetch_all(&self.connection) 
        .await
        .unwrap_or_default();

        if rows.is_empty() {
            println!("No interventions found for encounter_id: {} [{}]", encounter_id, rows.len());
            return Ok( Some( Vec::new() ) );
        }
        else{
            println!("Loading {} Interventions",  rows.len());
            let mut results: Vec<Intervention> = Vec::with_capacity(rows.len());
            for row in rows {
                let tmp_intv_id: i64 = row.0; // intervention_id
                let tmp_enc_id: i64 = row.1; // encounter_id
                
                let tmp_intervention_code = row.2; //intervention_code
                let tmp_description = row.3; // description
                let tmp_notes = row.4; // notes

                let tmp_location_id: i64 = row.5; // location_id
                let tmp_users_id: i64 = row.6; // users_id

                let tmp_status_code = row.7; //status_code

                results.push(
                    Intervention {
                        id: tmp_intv_id,
                        encounter_id: tmp_enc_id,
                        intervention_code: tmp_intervention_code,
                        description: tmp_description,
                        notes: tmp_notes,
                        location_id: tmp_location_id,
                        users_id: tmp_users_id,
                        status_code: tmp_status_code
                    }
                );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }


}