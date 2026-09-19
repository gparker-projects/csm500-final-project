//! # Defines a Data Access Object (DAO) for a (Patient) Encounter entity, which enables retrieval
//!   and assembly of an Encounter object, based on data in the database.
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use sqlx::postgres::{PgPool}; 
use chrono::NaiveDateTime;
use tracing;

use crate::dto::encounter::*;
use crate::dao::db_query;
use crate::constants;

#[derive(Debug, Clone)]
pub struct EncounterDAO {
    pub connection: PgPool,
}

impl EncounterDAO {
    
    /// ### EncounterDAO::new()
    ///    Creates a new Encounter Data Access Object, with a database pool for use by other calls
    /// 
    /// #### Parameters:
    /// * db_connection (PgPool): a PgPool for establishing a database connection
    /// 
    /// #### Returns:
    /// * EncounterDAO: the EncounterDAO object that was created
    /// 
    pub async fn new(db_connection: PgPool) -> Self {
        EncounterDAO {
            connection: db_connection,
        }
    }

    /// ### get_encounters()
    ///  Wrapper method that only requests the current encounter for the patient instead of all of them. This is to improve code clarity.
    /// 
    /// #### Parameters:
    ///  * patient_id: i64 - the id of the patient for which to retrieve the Encounter
    /// 
    /// #### Returns:
    ///  * Encounter - the Encounter(s) that was found, if any. If none, a shell Encounter is assembled and returned
    /// 
    pub async fn get_current_encounter(&self, patient_id: i64) -> Encounter {
        if patient_id == constants::INVALID_PATIENT_ID {
            tracing::error!("EncounterDao::get_encounters(): Invalid Patient Id = -1 provided");
        }

        let tmp : Vec<Encounter> = self.get_encounters(patient_id, true).await.unwrap().expect(constants::DATABASE_ERROR_NOT_FOUND);

        return match tmp.first() {
            Some( e ) =>{
                e.clone()
            },
            None => {
                tracing::error!("EncounterDao::get_encounters(): No Encounter returned for patient_id = {}", patient_id);
                Encounter {
                    patient_id: patient_id,
                    ..Default::default() 
                }
            },
        }
    }

    /// ### get_encounters()
    ///  Finds and returns all encounters based on an encounter
    /// 
    /// #### Parameters:
    ///  * patient_id: i64 - the id of the patient for which to retirieve the Encounters
    ///  * current_only: bool - boolean flag for if the encounter result set should include only the most current Encounter
    /// 
    /// #### Returns:
    ///  * Option< Vec<Encounter> > - a vector of the Encounter(s) that were found, if any. If none, an empty Vector is assembled and returned
    ///  * std::io::Error - Any error that occured
    /// 
    pub async fn get_encounters(&self, patient_id: i64, current_only: bool) -> Result< Option< Vec<Encounter> >, std::io::Error> {
        tracing::info!("EncounterDao::get_encounters({}, {})", patient_id, current_only);
        let tmp = match current_only {
            true => db_query::QRY_CURRENT_ENCOUNTER,
            false => db_query::QRY_ALL_ENCOUNTERS
        };

        let query = tmp.replace("{}", &patient_id.to_string());
        let rows: Vec<(i64, // encounter_id
                       NaiveDateTime, // admit_timestamp
                       Option<chrono::NaiveDateTime>, // discharge_timestamp
                       String, String, // admit and discharge notes
                       String, // encounter_site_name
                       String,  // current_encounter
                       String // room_identifier
                      )> = sqlx::query_as(&query)
        .fetch_all(&self.connection) 
        .await
        .unwrap_or_default();

        if rows.is_empty() {
            tracing::debug!(">get_encounters() Query: {}", query);
            tracing::debug!("No encounters found for patient_id: {} [{}]", patient_id, rows.len());
            return Ok( Some( Vec::new() ) );
        }
        else{
            //println!("get_encounters() -> Loading {} encounters",  rows.len());
            let mut results: Vec<Encounter> = Vec::with_capacity(rows.len());
            for row in rows {
                let tmp_enc_id: i64 = row.0; // encounter_id
                let tmp_admit_timestamp: NaiveDateTime = row.1;// admit_timestamp
                let tmp_discharge_timestamp: Option<NaiveDateTime> = row.2; //discharge_timestamp
                let tmp_admit_notes = row.3; // admission_notes
                let tmp_discharge_notes = row.4;  // discharge_notes
                let encounter_site_name = row.5; // encounter_site_name
                let tmp_room_identifier = row.6; // room_identifier
                let is_current_encounter = row.7; // is_current_encounter

                results.push(
                    Encounter {
                        id: tmp_enc_id,
                        admit_notes: tmp_admit_notes,
                        admit_timestamp: tmp_admit_timestamp, 
                        discharge_notes: tmp_discharge_notes,
                        discharge_timestamp: tmp_discharge_timestamp,
                        patient_id: patient_id,
                        encounter_site_name: encounter_site_name,
                        room_identifier: tmp_room_identifier,
                        is_current_encounter: is_current_encounter,
                    }
                );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }
}