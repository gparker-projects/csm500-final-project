//! # Defines a Data Access Object (DAO) for a (Patient) Intervention entity, which enables retrieval
//!   and assembly of an Intervention object, based on data in the database. Intervetions represent
//!   orders, procedures, medication, etc for a real-world Patient.
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use sqlx::postgres::{PgPool}; 
use sqlx::Row;
use chrono::NaiveDateTime;
use tracing;

use crate::dto::{intervention::*, intervention_detail::*};
use crate::constants;
use crate::dao::db_query;
use crate::ui::{data_forms::*};

#[derive(Debug, Clone)]
pub struct InterventionDAO {
    pub connection: PgPool,
}

impl InterventionDAO {

    /// ### InterventionDAO::new()
    ///   Creates a new Intervention Data Access Object, with a database pool for use by other calls
    /// 
    /// #### Parameters:
    /// * db_connection (PgPool): a PgPool for establishing a database connection
    /// 
    /// #### Returns:
    /// * InterventionDAO: the InterventionDAO object that was created
    /// 
    pub async fn new(db_connection: PgPool) -> Self {
        InterventionDAO {
            connection: db_connection,
        }
    }

    /// ### get_intervention()
    ///   Finds and returns an intervention based on an intervention/id => should never be more than one. 
    ///   For simplicity with the code, we'll still use fetch_all.
    /// 
    /// #### Parameters:
    /// * intervention_id (i64): the id of the intervention to be retrieved
    /// 
    /// #### Returns:
    /// * Option< Intervention >: the (Option-wrapped) Intervention, if found
    /// * std::io::Error: the error that occured, if applicable
    /// 
    pub async fn get_intervention(&self, intervention_id: i64) -> Result< Option< Intervention >, std::io::Error> {
        let query = db_query::QRY_INTERVENTION_FOR_ID.replace("{}", &intervention_id.to_string());

        let rows: Vec<(i64, i64, String, String,
                       i64, i64, i64, i64,
                       String, String, String,
                       Option<chrono::NaiveDateTime>,
                       Option<chrono::NaiveDateTime>
                    )> = sqlx::query_as(&query)
        .fetch_all(&self.connection) 
        .await
        .unwrap_or_default();

        if rows.is_empty() {
            tracing::debug!(">get_interventions() Query: {}", query);
            tracing::debug!("No Interventions found for encounter_id: {} [count={}]", intervention_id, rows.len());
            return Ok( None );
        }
        else{
            //println!("Loading {} Interventions",  rows.len());
            let mut results: Vec<Intervention> = Vec::with_capacity(rows.len());
            for row in rows { // should only ever iterate once
                let tmp_intv_id: i64 = row.0; // intervention_id
                let tmp_enc_id: i64 = row.1; // encounter_id
                let tmp_description = row.2; // description
                let tmp_notes = row.3; // notes
                let tmp_location_id: i64 = row.4; // location_id
                let tmp_users_id: i64 = row.5; // users_id
                let tmp_intervention_type_id: i64 = row.6; //intervention_type_id
                let tmp_status_id: i64 = row.7; // users_id
                let tmp_room_identifier = row.8; // tmp_room_identifier

                let tmp_intervention_type: String = row.9; //intervention_type_id
                let tmp_status = row.10; //status_code
                let tmp_scheduled_timestamp = row.11; // i.scheduled_timestamp
                let tmp_performed_timestamp = row.12; //i.performed_Timestamp

                results.push(
                    Intervention {
                        id: tmp_intv_id,
                        encounter_id: tmp_enc_id,
                        description: tmp_description,
                        notes: tmp_notes,
                        location_id: tmp_location_id,
                        users_id: tmp_users_id,
                        intervention_type_id: tmp_intervention_type_id,
                        status_id: tmp_status_id,
                        room_identifier: tmp_room_identifier,
                        intervention_type: tmp_intervention_type,
                        status_code: tmp_status,
                        scheduled_timestamp: tmp_scheduled_timestamp,
                        performed_timestamp: tmp_performed_timestamp
                    }
                );
            }
            return Ok( results.first().cloned() ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }

    /// ### get_interventions()
    ///   Finds and returns all interventions based on an encounter
    /// 
    /// #### Parameters:
    /// * encounter_id (i64): the id of the intervention to be retrieved
    /// * current_only (bool): boolean flag indicating if only the most current intervention should be included
    /// 
    /// #### Returns:
    /// * Option< Vec<Intervention> >: the (Option-wrapped) Intervention vector, if found. If none, an empty vector is returned.
    /// * std::io::Error: the error that occured, if applicable
    /// 
    pub async fn get_interventions(&self, encounter_id: i64, current_only: bool) -> Result< Option< Vec<Intervention> >, std::io::Error> {

        let tmp = match current_only {
            true => db_query::QRY_CURRENT_VITALS_FOR_ENC_ID,
            false => db_query::QRY_INTERVENTIONS_FOR_ENC_ID
        };
        let query = tmp.replace("{}", &encounter_id.to_string());

        let rows: Vec<(i64, i64, String, String,
                       i64, i64, i64, i64,
                       String, String, String,
                       Option<chrono::NaiveDateTime>,
                       Option<chrono::NaiveDateTime>
                    )> = sqlx::query_as(&query)
        .fetch_all(&self.connection) 
        .await
        .unwrap_or_default();

        if rows.is_empty() {
            //println!(">get_interventions() Query: {}", query);
            tracing::debug!("No Interventions found for encounter_id: {} [count={}]", encounter_id, rows.len());
            return Ok( Some( Vec::new() ) );
        }
        else{
            //println!("Loading {} Interventions",  rows.len());
            let mut results: Vec<Intervention> = Vec::with_capacity(rows.len());
            for row in rows {
                let tmp_intv_id: i64 = row.0; // intervention_id
                let tmp_enc_id: i64 = row.1; // encounter_id
                let tmp_description = row.2; // description
                let tmp_notes = row.3; // notes
                let tmp_location_id: i64 = row.4; // location_id
                let tmp_users_id: i64 = row.5; // users_id
                let tmp_intervention_type_id: i64 = row.6; //intervention_type_id
                let tmp_status_id: i64 = row.7; // users_id
                let tmp_room_identifier = row.8; // tmp_room_identifier

                let tmp_intervention_type: String = row.9; //intervention_type_id
                let tmp_status = row.10; //status_code
                let tmp_scheduled_timestamp = row.11; // i.scheduled_timestamp
                let tmp_performed_timestamp = row.12; //i.performed_Timestamp

                results.push(
                    Intervention {
                        id: tmp_intv_id,
                        encounter_id: tmp_enc_id,
                        description: tmp_description,
                        notes: tmp_notes,
                        location_id: tmp_location_id,
                        users_id: tmp_users_id,
                        intervention_type_id: tmp_intervention_type_id,
                        status_id: tmp_status_id,
                        room_identifier: tmp_room_identifier,
                        intervention_type: tmp_intervention_type,
                        status_code: tmp_status,
                        scheduled_timestamp: tmp_scheduled_timestamp,
                        performed_timestamp: tmp_performed_timestamp
                    }
                );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }

    
    /// ### get_all_intervention_details_for_an_intervention()
    ///   Finds and returns all Intervention Details, based on an Intervention
    /// 
    /// #### Parameters:
    /// * intervention_id (i64): the id of the intervention for which Intervention Details are to be retrieved
    /// * type_id: i64: type of the interventions to be retreived
    /// 
    /// #### Returns:
    /// * Option< Vec<InterventionDetail> >: the (Option-wrapped) InterventionDetail vector, if found. If none, an empty vector is returned.
    /// * std::io::Error: the error that occured, if applicable
    /// 
    pub async fn get_all_intervention_details_for_an_intervention(&self, intervention_id: i64, type_id: i64) -> Result< Option< Vec<InterventionDetail> >, std::io::Error> {
        println!("get_all_intervention_details_for_an_intervention()");
        let query =  match type_id == constants::NOT_SPECIFIED_ID {
            true => {
                let tmp = db_query::QRY_ALL_INTERVENTION_DETAILS_FOR_AN_INTERVENTION;
                tmp.replace("{}", &intervention_id.to_string())
            }
            false => {
                let tmp = db_query::QRY_ALL_INTERVENTION_DETAILS_FOR_INTV_AND_TYPE;
                let tmp2 = tmp.replace("{1}", &intervention_id.to_string());
                tmp2.replace("{2}", &type_id.to_string())
            }
        };
        //println!("..Query={}", query);

        let rows: Vec<(i64, String, String, NaiveDateTime, String,i64, i64 )> = sqlx::query_as(&query)
        .fetch_all(&self.connection) 
        .await
        .unwrap_or_default();

        if rows.is_empty() {
            tracing::debug!("..No intervention details found for intervention_id: {} [{}]", intervention_id, rows.len());
            tracing::debug!("..Query: {}", query);
            return Ok( Some( Vec::new() ) );
        }
        else{
            //println!("get_all_intervention_details_for_an_intervention() -> Loading {} Intervention Details",  rows.len());
            let mut results: Vec<InterventionDetail> = Vec::with_capacity(rows.len());
            for row in rows {
                let tmp_id: i64 = row.0; // id
                let tmp_value = row.1; // value
                let tmp_notes = row.2; // notes
                let tmp_entry_timestamp: NaiveDateTime = row.3; // entry_timestamp
                let tmp_intervention_type: String = row.4; //intervention_type
                let tmp_intv_id: i64 = row.5; // intervention_id
                let tmp_type_id: i64 = row.6; // type_id

                results.push(
                    InterventionDetail {
                        id: tmp_id,
                        intervention_id: tmp_intv_id,
                        type_id: tmp_type_id,
                        value: tmp_value,
                        notes: tmp_notes,
                        entry_timestamp: tmp_entry_timestamp,
                        intervention_type: tmp_intervention_type
                    }
                );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }

    /// ### get_most_recent_vitals()
    ///   get the most recent intervention for the Encounter that is of a vitals type
    /// 
    /// #### Parameters:
    /// * encounter_id (i64): the id of the Encounter for which most recent vital (Intervention) is to be retrieved
    /// 
    /// #### Returns:
    /// * Option< Intervention >: the (Option-wrapped) Intervention, if found
    /// * std::io::Error: the error that occured, if applicable
    /// 
    pub async fn get_most_recent_vitals(&self, encounter_id: i64) -> Result< Option< Intervention >, std::io::Error> {
        let results = self.get_interventions(encounter_id, true).await.expect(constants::DATABASE_ERROR_NOT_FOUND);

        Ok( results.expect(constants::DATABASE_ERROR_NOT_FOUND).first().cloned() )
    }

    /// ### upsert_intervention_from_intv_form()
    ///   Given an InterventionDataForm, creates a new Intervention record, or updates an existing one
    /// 
    /// #### Parameters:
    /// * form: InterventionDataForm - the form that represents the Intervention to be inserted/updated
    /// * _audit_user_id: i64 - id of the user which is to be audited (potential feature only)
    /// 
    /// #### Returns:
    /// * i64: the id of the Intervention record that is created, if applicable
    /// * std::io::Error: the error that occured, if applicable
    ///
    pub async fn upsert_intervention_from_intv_form(&self, form: InterventionDataForm, _audit_user_id: i64)-> Result<i64, sqlx::Error> {
        tracing::debug!("  > upsert_intervention_from_intv_form (Intervention id={})", &form.intervention_id);

        let mut query_level_0 = db_query::UPDATE_INTERVENTION.to_string();

        // if id is not specified, we INSERT
        if &form.intervention_id == &constants::NOT_SPECIFIED_ID.to_string() {
            query_level_0 = db_query::INSERT_INTERVENTION.to_string();
        }

        let query_level_1 = &query_level_0.replace("{description}", &form.description.trim());
        let query_level_2 = &query_level_1.replace("{notes}", &form.notes.trim());
        let query_level_3 = &query_level_2.replace("{location_id}", &form.location_id);
        let query_level_4 = &query_level_3.replace("{users_id}", &form.users_id);
        let query_level_5 = &query_level_4.replace("{encounter_id}", &form.encounter_id);
        let query_level_6 = &query_level_5.replace("{intervention_type_id}", &form.intervention_type_id);
        let query_level_7 = &query_level_6.replace("{scheduled_timestamp}", &form.scheduled_timestamp);
        let query_level_8 = &query_level_7.replace("{performed_timestamp}", &form.performed_timestamp);
        let query_level_9 = &query_level_8.replace("{intervention_id}", &form.intervention_id); // INSERT does not include this field, only the UPDATE

        let query = &query_level_9.replace("{status_id}", &form.status_id);

        tracing::debug!(" >> Intervention Upsert query: {}", query);

        let result = sqlx::query(&query)
                                                        .fetch_one(&self.connection)
                                                        .await
                                                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
        // return the patient id that was created or updated
        let inserted_id: i64 = result.get("id");

        Ok(inserted_id)
    }


    /// ### upsert_intervention_details_from_intv_form()
    ///   Given an InterventionDetailsDataForm, creates a new Intervention Details record, or updates an existing one
    /// 
    /// #### Parameters:
    /// * form: InterventionDetailsDataForm - the form that represents the Intervention Details to be inserted/updated
    /// * _audit_user_id: i64 - id of the user which is to be audited (potential feature only)
    /// 
    /// #### Returns:
    /// * i64: the id of the Intervention Details record that is created, if applicable
    /// * std::io::Error: the error that occured, if applicable
    ///
    pub async fn upsert_intervention_details_from_intv_form(&self, form: InterventionDetailsDataForm, _audit_user_id: i64)-> Result<i64, sqlx::Error> {
        tracing::debug!("upsert_intervention_details_from_intv_form (Intv Dtls id={})", &form.intervention_details_id);
       // println!("upsert_intervention_details_from_intv_form (Intv Dtls id={})", &form.intervention_details_id);

        let mut query_level_0 = db_query::UPDATE_INTERVENTION_DETAILS.to_string();
    
        // if id is not specified, we INSERT
        if &form.intervention_details_id == &constants::NOT_SPECIFIED_ID.to_string() {
            query_level_0 = db_query::INSERT_INTERVENTION_DETAILS.to_string();
        }

        let query_level_1 = &query_level_0.replace("{value}", &form.value.trim());
        let query_level_2 = &query_level_1.replace("{notes}", &form.notes.trim());
        let query_level_3 = &query_level_2.replace("{intervention_details_id}", &form.intervention_details_id);
        let query_level_4 = &query_level_3.replace("{type_id}", &form.type_id);
        let query_level_5 = &query_level_4.replace("{entry_timestamp}", &form.entry_timestamp);
        let query = &query_level_5.replace("{intervention_id}", &form.intervention_id); // INSERT does not include this field, only the UPDATE

        tracing::debug!("..UPSERT query: {}", query);
       // println!("..UPSERT query: {}", query);

        let result = sqlx::query(&query)
                                                        .fetch_one(&self.connection)
                                                        .await
                                                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
        // return the patient id that was created or updated
        let inserted_id: i64 = result.get("id"); // NOTE: MUST be lowercase, per Postgresql



        Ok(inserted_id)
    }
}