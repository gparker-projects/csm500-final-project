use sqlx::postgres::{PgPoolOptions, PgPool}; 
//use sqlx::Row;
//use std::io::{Error, ErrorKind};
use chrono::NaiveDateTime;
use crate::dto::{intervention::*, intervention_detail::*};
use crate::constants;
use crate::dao::db_query;

#[derive(Debug, Clone)]
pub struct InterventionDAO {
    pub connection: PgPool,
}

impl InterventionDAO {
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

        InterventionDAO {
            connection: db_pool,
        }
    }

    /// Finds and returns all interventions based on an encounter
    /// 
    pub async fn get_interventions(&self, encounter_id: i64, current_only: bool) -> Result< Option< Vec<Intervention> >, std::io::Error> {

        let tmp = match current_only {
            true => db_query::QRY_CURRENT_VITALS_FOR_ENC_ID,
            false => db_query::QRY_INTERVENTIONS_FOR_ENC_ID
        };
        let query = tmp.replace("{}", &encounter_id.to_string());

        //println!("get_interventions Query: {}", query);

        let rows: Vec<(i64, i64, String, String,
                       i64, i64, i64, i64,
                       String, String, String )> = sqlx::query_as(&query)
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
                let tmp_description = row.2; // description
                let tmp_notes = row.3; // notes
                let tmp_location_id: i64 = row.4; // location_id
                let tmp_users_id: i64 = row.5; // users_id
                let tmp_intervention_type_id: i64 = row.6; //intervention_type_id
                let tmp_status_id: i64 = row.7; // users_id
                let tmp_room_identifier = row.8; // tmp_room_identifier

                let tmp_intervention_type: String = row.9; //intervention_type_id
                let tmp_status = row.10; //status_code

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
                        status_code: tmp_status
                    }
                );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }

    /// Finds and returns all Intervention Details, based on an Intervention
    /// 
    pub async fn get_all_intervention_details(&self, intervention_id: i64, type_id: i64) -> Result< Option< Vec<InterventionDetail> >, std::io::Error> {

        let query =  match type_id == constants::NOT_SPECIFIED_ID {
            true => {
                let tmp = db_query::QRY_ALL_INTERVENTION_DETAILS;
                tmp.replace("{}", &intervention_id.to_string())
            }
            false => {
                let tmp =db_query::QRY_ALL_INTERVENTION_DETAILS_FOR_TYPE;
                let tmp2 = tmp.replace("{1}", &intervention_id.to_string());
                tmp2.replace("{2}", &type_id.to_string())
            }
        };

        //println!("get_interventions_details() Query: {}", query);

        let rows: Vec<(i64, String, String, NaiveDateTime, String,i64, i64 )> = sqlx::query_as(&query)
        .fetch_all(&self.connection) 
        .await
        .unwrap_or_default();

        if rows.is_empty() {
            println!("No intervention details found for intervention_id: {} [{}]", intervention_id, rows.len());
            return Ok( Some( Vec::new() ) );
        }
        else{
            println!("Loading {} Intervention Details",  rows.len());
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


    ///
    /// get the most recent intervention for the Encounter that is of a vitals type
    /// 
    pub async fn get_most_recent_vitals(&self, encounter_id: i64) ->  Intervention {
        let tmp : Vec<Intervention> = self.get_interventions(encounter_id, true).await.unwrap().expect(constants::DATABASE_ERROR_NOT_FOUND);

        return tmp.first().unwrap().clone();
    }
}