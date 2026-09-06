//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use sqlx::postgres::{PgPool}; 
use tracing;

use crate::{constants, dao::db_query};


#[derive(Debug, Clone)]
pub struct CommonDAO {
    pub connection: PgPool,
}

impl CommonDAO {

pub const REF_TYPE_GROUP_1_INTERVENTION_TYPES: i64 = 1;
pub const REF_TYPE_GROUP_2_INTERVENTION_STATUS: i64 = 2;

    /// Creates a new AuthObjects object, with a database pool for use by other calls
    /// 
    pub async fn new(db_connection: PgPool) -> Self {
        CommonDAO {
            connection: db_connection,
        }
    }

    ///
    /// Accessor to retrieve locations from the database into a tuple. 
    ///
    /// Returns: a tuple (i64, String) containing the id of the location and an aggregated string
    ///          describing the location.
    /// 
    /*pub async fn get_locations(&self)-> Result< Option< Vec<(i64, String)> >, std::io::Error> {
        let query: String = db_query::QRY_ACTIVE_LOCATIONS.to_owned();
        tracing::debug!("get_locations()");

        let rows: Vec<( i64, String )> = sqlx::query_as(&query)
                                                .fetch_all(&self.connection) 
                                                .await
                                                .unwrap_or_default();
        if rows.is_empty() {
            tracing::error!("No Locations defined in system");
            return Ok( Some( Vec::new() ) );
        }
        else{
            let mut results: Vec<(i64, String)> = Vec::with_capacity(rows.len());
            for row in rows {
                let tmp_loc_id: i64 = row.0; // location_id
                let tmp_aggregate_name = row.1; // aggregated name

                results.push( (tmp_loc_id, tmp_aggregate_name) );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }*/

    ///
    /// Accessor to retrieve locations from the database into a tuple. 
    ///
    /// Returns: a tuple (i64, String) containing the id of the location and an aggregated string
    ///          describing the location.
    /// 
    pub async fn get_locations_for_user(&self, user_id: i64)-> Result< Option< Vec<(i64, String)> >, std::io::Error> {
        let query_level_0: String = db_query::QRY_CURRENT_USER_LOCATIONS.to_owned();
        let query = query_level_0.replace("{}", &user_id.to_string());
        tracing::debug!("get_locations_for_user()");

        let rows: Vec<( i64, String )> = sqlx::query_as(&query)
                                                .fetch_all(&self.connection) 
                                                .await
                                                .unwrap_or_default();
        if rows.is_empty() {
            tracing::error!("No Locations defined in system");
            return Ok( Some( Vec::new() ) );
        }
        else{
            let mut results: Vec<(i64, String)> = Vec::with_capacity(rows.len());
            for row in rows {
                let tmp_loc_id: i64 = row.0; // location_id
                let tmp_aggregate_name = row.1; // aggregated name

                results.push( (tmp_loc_id, tmp_aggregate_name) );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }

    ///
    /// Accessor to retrive COMMON REFERENCE TYPE entries from the database into a tuple.
    ///
    /// Returns: a tuple (i64, String, String) containing:
    ///          - id of the reference list item
    ///          - short name of the reference list item
    ///          - long name of the reference list item
    /// 
    pub async fn get_common_references(&self, group_ids: String, active_only: bool)-> Result< Option< Vec<(i64, String, String)> >, std::io::Error> {
        tracing::debug!("get_common_references()");
        println!("get_common_references()");

        let query_level_0: String = match active_only {
		    true => db_query::QRY_COMMON_REF_TYPES_FOR_GROUP.to_owned(),
		    false => db_query::QRY_COMMON_REF_TYPES_FOR_GROUP_ACTIVE_ONLY.to_owned(),
        };
        
        let query = query_level_0.replace("{group_ids}", &group_ids.to_string());
        println!("..SQL query: {}", query);

        let rows: Vec<( i64, String, String )> = sqlx::query_as(&query)
                                                .fetch_all(&self.connection) 
                                                .await
                                                .unwrap_or_default();
        if rows.is_empty() {
            tracing::error!("Reference entries not found for group_id={}", group_ids);
            return Ok( Some( Vec::new() ) );
        }
        else{
            let mut results: Vec<(i64, String, String)> = Vec::with_capacity(rows.len());
            for row in rows {
                let tmp_id: i64 = row.0; // id
                let tmp_name = row.1; //  name
                let tmp_description = row.2; // description

                results.push( (tmp_id, tmp_name, tmp_description) );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }

    ///
    /// Accessor to retreive a single COMMON REFERENCE TYPE entries from the database into a tuple.
    ///
    /// Returns: a tuple (i64, String, String) containing:
    ///          - id of the reference list item
    ///          - short name of the reference list item
    ///          - long name of the reference list item
    /// 
    pub async fn get_common_reference(&self, ref_type_id: i64)-> Result< Option< (i64, String, String) > , std::io::Error> {
        tracing::debug!("get_common_reference()");
        let query_level_0 = db_query::QRY_SINGLE_COMMON_REF_TYPE_BY_ID;
        let query = query_level_0.replace("{common_ref_id}", &ref_type_id.to_string());

        let rows: Vec<( i64, String, String )> = sqlx::query_as(&query)
                                                .fetch_all(&self.connection) 
                                                .await
                                                .unwrap_or_default();

        let mut results: (i64, String, String) = (constants::INVALID_OTHER_ID, constants::GENERAL_ERROR_NOT_FOUND.to_string(), constants::GENERAL_ERROR_NOT_FOUND.to_string());
        if rows.is_empty() {
            tracing::debug!("Reference entry not found for ref_type_id={}", ref_type_id);
            return Ok( Some(  results  ) );
        }
        else{
             // as the query only has one row, there will only ever be one result
           
            for row in rows {
                let tmp_id: i64 = row.0; // id
                let tmp_name = row.1; //  name
                let tmp_description = row.2; // description

                results = (tmp_id, tmp_name, tmp_description) ; // (id, name, description
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }

    ///
    /// Shortcut method to obtain Intervention Status group (id=2) entries from the COMMON REFERENCE TYPE table
    ///
    /// Returns: a tuple (i64, String, String) containing:
    ///          - id of the reference list item
    ///          - short name of the reference list item
    ///          - long name of the reference list item
    /// 
    pub async fn get_intervention_statuses(&self)-> Result< Option< Vec<(i64, String, String)> >, std::io::Error> {
        self.get_common_references(Self::REF_TYPE_GROUP_2_INTERVENTION_STATUS.to_string(), true).await
    }

    ///
    /// Shortcut method to obtain Intervention Type group (id=1) entries from the COMMON REFERENCE TYPE table
    ///
    /// Returns: a tuple (i64, String, String) containing:
    ///          - id of the reference list item
    ///          - short name of the reference list item
    ///          - long name of the reference list item
    /// 
    pub async fn get_intervention_types(&self)-> Result< Option< Vec<(i64, String, String)> >, std::io::Error> {
        let intv_types = "1, 3".to_string();

        self.get_common_references(intv_types, true).await
    }

    ///
    /// Shortcut method to obtain the details for a single Intervention Type group (id=1), from the COMMON REFERENCE TYPE table
    ///
    /// Returns: a tuple (i64, String, String) containing:
    ///          - id of the reference list item
    ///          - short name of the reference list item
    ///          - long name of the reference list item
    /// 
    pub async fn get_intervention_type(&self, type_id: i64)-> Result< Option< (i64, String, String) >, std::io::Error> {
        self.get_common_reference(type_id).await
    }

    ///
    /// ### CommonDAO::is_intervention_group_type()
    ///   Confirms that the Reference Type Code Id provided is in one of the two types of Intervention groups:
    ///    - Clinical or Non-Clinical
    /// 
    /// #### Parameters:
    /// * common_ref_type_group_id (i64): the id to be checked
    /// 
    /// #### Returns:
    /// * bool: true if the id matches one of the groups of Intervention Types
    /// 
    pub fn is_intervention_group_type(common_ref_type_group_id: i64) -> bool {
        let mut result: bool = common_ref_type_group_id == constants::CRT_CLINICAL_INTERVENTION_GRP_ID;
        result = result || (common_ref_type_group_id == constants::CRT_NON_CLINICAL_INTERVENTION_GRP_ID);
        return result;
    }

}