//! # Defines a Data Access Object (DAO) for a Patient entity, which enables retrieval
//!   and assembly of a Patient object, based on data in the database.
//!
//!  CSM500 Project (April - October 2026)
//!  Graham Parker (Student ID: 240120522)

use sqlx::postgres::{PgPool}; 
use sqlx::Row;
use chrono::NaiveDateTime;
use tracing;

use crate::constants;
use crate::dao::db_query;
use crate::dto::{patient::*, encounter::*, intervention::*};
use crate::ui::{data_forms::*};

#[derive(Debug, Clone, Default)]
pub struct PatientWrapper {
    pub patient: Patient,
    pub current_encounter: Encounter,
    pub most_recent_intervention: Option<Intervention>//,
}

#[derive(Debug, Clone)]
pub struct PatientDAO {
    pub connection: PgPool,
}

impl PatientDAO {

    /// ### PatientDAO::new()
    ///    Creates a new Patient Data Access Object, with a database pool for use by other calls
    /// 
    /// #### Parameters:
    /// * db_connection (PgPool): a PgPool for establishing a database connection
    /// 
    /// #### Returns:
    /// * PatientDAO: the PatientDAO object that was created
    /// 
    pub async fn new(db_connection: PgPool) -> Self {
        PatientDAO {
            connection: db_connection,
        }
    }  

    /// ### get_patient_details_optional_discharged()
    ///    Finds and returns the data for a specific patient, as a Patient struct, who may or may not have been discharged
    /// 
    /// #### Parameters:
    /// * _audit_user_id (i64): the id of the user making the data request, for audit purposes
    /// * patient_id (i64): the id of the patient to be obtained
    /// 
    /// #### Returns:
    /// * Option<Patient>: the Patient, if found
    /// * sqlx::Error: An error, if applicable
    /// 
    pub async fn get_patient_details_optional_discharged(&self, _audit_user_id: i64, patient_id: i64, ignore_discharge: bool) -> Result< Option<Patient>, std::io::Error> {
        let tmp: String = match ignore_discharge {
            true => db_query::QRY_SINGLE_PATIENT_DETAILS_STILL_ADMITTED_NOT_DISCHARGED.to_owned(), 
            false => db_query::QRY_SINGLE_PATIENT_DETAILS_DISREGARD_DISCHARGE.to_owned(),
        };
        let query = tmp.replace("{}", &patient_id.to_string());

        //tracing::debug!("get_patient_details Query: {}", query);
        println!("get_patient_details_optional_discharged() Query: {}", query);

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
                let tmp_phn:    i64 = row.get("phn"); // SIN

                let tmp_location_short_name = row.get("location_short_name");  // location_short_name

                Ok(  Some( 
                        Patient {
                            id: tmp_pat_id,
                            encounter_id: tmp_enc_id,
                            legal_first_name:  tmp_legal_first_name, //"DUMMY".to_string(),
                            legal_last_name: tmp_legal_last_name,//"DUMMY".to_string(), 
                            legal_middle_names: tmp_legal_middle_names, //"DUMMY".to_string(),
                            phn: tmp_phn,
                            birth_date: tmp_birthdate,
                            location_id: tmp_loc_id,
                            location_short_name: tmp_location_short_name,
                            admit_timestamp:tmp_admit_timestamp, 
                            admit_notes: tmp_admit_notes,
                            discharge_timestamp: tmp_discharge_timestamp,
                            discharge_notes: tmp_discharge_notes,
                        }
                    )
                )
            }
            _ => {
                tracing::debug!("get_patient_details() Query: {}", query);
                tracing::debug!("No patient found for patient_id = {}", patient_id);
                Ok( None )
            }
        }
    }

    /// ### get_patient_details_not_discharged()
    ///    Finds and returns the data for a specific patient, as a Patient struct, who has NOT been discharged
    /// 
    /// #### Parameters:
    /// * _audit_user_id (i64): the id of the user making the data request, for audit purposes
    /// * patient_id (i64): the id of the patient to be obtained
    /// 
    /// #### Returns:
    /// * Option<Patient>: the Patient, if found
    /// * sqlx::Error: An error, if applicable
    /// 
    pub async fn get_patient_details_not_discharged(&self, _audit_user_id: i64, patient_id: i64) -> Result< Option<Patient>, std::io::Error> {
        return self.get_patient_details_optional_discharged(_audit_user_id, patient_id, true).await;
    }

    /// ### update_encounter_from_discharge_form()
    ///    Given an AdmitFormData, create a new Encounter reocrd, or update an existing one
    /// 
    /// #### Parameters:
    /// * form (DischargeDataForm): a DischargeDataForm object describing the data to be used for the update
    /// * _audit_user_id (i64): the id of the user making the data request, for audit purposes
    /// 
    /// #### Returns:
    /// * i64: the id of the Encounter record that is created, if applicable
    /// * sqlx::Error: An error, if applicable
    /// 
    pub async fn update_encounter_from_discharge_form(&self, form: DischargeDataForm, _audit_user_id: i64)-> Result<i64, sqlx::Error> {
        tracing::debug!("update_encounter_from_discharge_form()");
        println!("update_encounter_from_discharge_form()");

        let query_level_0 = db_query::UPDATE_ENCOUNTER_FOR_DISCHARGE.to_string();
        let query_level_1 = &query_level_0.replace("{discharge_notes}", &form.discharge_notes.clone().trim());
        let query_level_2 = &query_level_1.replace("{encounter_id}", &form.encounter_id.clone().trim());

        println!("..SQL query:\n{}", query_level_2);

        let result = sqlx::query(&query_level_2)
                                                        .fetch_one(&self.connection)
                                                        .await
                                                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
        match result {
            Ok( row ) => {
                Ok( row.get("id") ) // return the patient id that was created or updated
            },
            Err(_) => {
                Ok( constants::INVALID_OTHER_ID )
            }
        }
    }

    /// ### upsert_encounter_from_admit_form()
    ///    Given an AdmitFormData, create a new Encounter rcocrd, or update an existing one
    /// 
    /// #### Parameters:
    /// * form (AdmitDataForm): an AdmitDataForm object describing the data to be used for the update
    /// * _audit_user_id (i64): the id of the user making the data request, for audit purposes
    /// 
    /// #### Returns:
    /// * i64: the id of the Encounter record that is created or updated, as applicable
    /// * sqlx::Error: An error, if applicable
    /// 
    pub async fn upsert_encounter_from_admit_form(&self, form: AdmitDataForm, _audit_user_id: i64)-> Result<i64, sqlx::Error> {
        tracing::debug!("> upsert_encounter_from_admit_form");
        println!("> upsert_encounter_from_admit_form");

        let query_level_0: String;

        // if encounter id is not specified, we INSERT
        if &form.encounter_id == &constants::NOT_SPECIFIED_ID.to_string() {
            query_level_0 = db_query::INSERT_ENCOUNTER.to_string();
        }
        else{
            query_level_0 = db_query::UPDATE_ENCOUNTER.to_string().replace("{encounter_id}", &form.encounter_id.clone().trim());
        }

        let query_level_1 = &query_level_0.replace("{admit_notes}", &form.admit_notes.clone().trim());
        let query_level_2 = &query_level_1.replace("{patient_id}", &form.patient_id.clone().trim());
        let query = &query_level_2.replace("{location_id}", &form.location_id.clone().trim());

        tracing::debug!(" >> Encounter Upsert: {}", query);
        //println!(" >> Encounter Upsert: {}", query);

        let result = sqlx::query(&query)
                                                        .fetch_one(&self.connection)
                                                        .await
                                                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
        match result {
            Ok( row ) => {
                Ok( row.get("id") ) // return the patient id that was created or updated
            },
            Err(_) => {
                Ok( constants::INVALID_OTHER_ID )
            }
        }
    }
    
    /// ### upsert_patient_from_admit_form()
    ///    Given an AdmitFormData, create a new Encounter rcocrd, or update an existing one
    /// 
    /// #### Parameters:
    /// * form (AdmitDataForm): an AdmitDataForm object describing the data to be used for the update
    /// * _audit_user_id (i64): the id of the user making the data request, for audit purposes
    /// 
    /// #### Returns:
    /// * i64: the id of the Encounter record that is created or updated, as applicable
    /// * sqlx::Error: An error, if applicable
    /// 
    /// #### Refs
    /// * https://medium.com/@francis.stephan/developing-a-web-app-with-rust-part-4-sqlx-data-validation-deployment-final-remarks-303e78c2a546
    /// 
    pub async fn upsert_patient_from_admit_form(&self, form: AdmitDataForm, _audit_user_id: i64)-> Result<i64, sqlx::Error> {
        tracing::debug!("> upsert_patient_from_admit_form");
        println!("> upsert_patient_from_admit_form");
            
        let query_level_0 = db_query::UPSERT_PATIENT;
        let query_level_1 = &query_level_0.replace("{legal_last_name}", &form.patient_last_name.clone().trim());
        let query_level_2 = &query_level_1.replace("{legal_first_name}", &form.patient_first_name.clone().trim());
        let query_level_3 = &query_level_2.replace("{legal_middle_names}", &form.patient_middle_name.clone().trim());
        let query_level_4 = &query_level_3.replace("{birthdate}", &form.birthdate.clone());
        let query_level_5 = &query_level_4.replace("{phn}", &form.phn.clone());
        let query = query_level_5.clone();

        tracing::debug!("..Upsert: {}", query_level_5);
        println!("..Upsert: {}", query_level_5);

        let result = sqlx::query(&query)
                                                        .fetch_one(&self.connection)
                                                        .await
                                                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
        match result {
            Ok( row ) => {
                Ok( row.get("id") ) // return the patient id that was created or updated
            },
            Err(_) => {
                Ok( constants::INVALID_OTHER_ID )
            }
        }
    }

  
    /// ### get_patients_at_users_site_no_discharge()
    ///    Finds and returns any patients that are currently assigned to the user
    /// 
    /// #### Parameters:
    /// * user_id (i64): the id of the user that will be used to identify the department, in the lookup
    /// 
    /// #### Returns:
    /// * Option< Vec<Patient>: a vector of Patients, if found
    /// * sqlx::Error: An error, if applicable
    /// 
    /// #### Refs
    /// * https://docs.rs/sqlx/latest/sqlx/fn.query_as.html
    /// * https://stackoverflow.com/questions/67243108/mapping-nm-relations-into-vec-using-sqlx
    /// * https://doc.rust-lang.org/std/io/struct.Error.html - for return Error
    /// 
    pub async fn get_patients_at_users_site_no_discharge(&self, user_id: i64) -> Result< Option< Vec<Patient> >, std::io::Error> {
        let tmp: String = db_query::QRY_ALL_PATIENTS_AT_USERS_SITE_NO_DISCHARGE.to_owned();
        let query = tmp.replace("{}", &user_id.to_string());

        //tracing::debug!(">get_patients_at_users_site_no_discharge() Query: {}", query);
        //println!(">get_patients_at_users_site_no_discharge() Query: {}", query);

        let rows: Vec<(i64, i64, i64, String, String, String,
                        String, String,
                        chrono::NaiveDateTime, chrono::NaiveDateTime, 
                        Option<chrono::NaiveDateTime>,
                        i64, String
                        )> = sqlx::query_as(&query)
        .fetch_all(&self.connection) 
        .await
        .unwrap_or_default();

        if rows.is_empty() {
            //tracing::debug!("No patients found for user_id: {} [{}]", user_id, rows.len());
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
                
                let tmp_phn: i64 = row.11; // phn

                let tmp_location_short_name = row.12; // location_short_name

                results.push(
                    Patient {
                        id: tmp_pat_id,
                        encounter_id: tmp_enc_id,
                        legal_first_name:  tmp_legal_first_name, 
                        legal_last_name: tmp_legal_last_name,
                        legal_middle_names: tmp_legal_middle_names, 
                        phn: tmp_phn,
                        birth_date: tmp_birthdate,
                        location_id: tmp_loc_id,
                        admit_timestamp:tmp_admit_timestamp,
                        admit_notes: tmp_admit_notes,
                        discharge_timestamp: tmp_discharge_timestamp,
                        discharge_notes: tmp_discharge_notes,
                        location_short_name: tmp_location_short_name
                    }
                );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }
}