//! # Defines a Data Access Object (DAO) for a Feature Preference entity, which enables retrieval
//!   and assembly of an Feature Preference object, based on data in the database.
//!
//!  CSM500 Project (April - October 2026)
//!  Graham Parker (Student ID: 240120522)

use chrono::NaiveDateTime;
use std::collections::HashSet;
use sqlx::postgres::{PgPool}; 
use sqlx::Row;
use tracing;

use crate::{constants, dao::db_query};
use crate::dto::feature_preference::FeaturePreference;
//use crate::dao::common_dao::CommonDAO;

#[derive(Debug, Clone)]
pub struct FeaturePreferenceDAO {
    pub connection: PgPool,
}

impl FeaturePreferenceDAO {

    /// ### FeaturePreferenceDAO::new()
    ///    Creates a new Feature Preference Data Access Object, with a database pool for use by other calls
    /// 
    /// #### Parameters:
    /// * db_connection (PgPool): a PgPool for establishing a database connection
    /// 
    /// #### Returns:
    /// * FeaturePreferenceDAO: the FeaturePreferenceDAO object that was created
    /// 
    pub async fn new(db_connection: PgPool) -> Self {
        FeaturePreferenceDAO {
            connection: db_connection,
        }
    }

    /// ### get_active_feature_preferences_for_user_intervention_level()
    ///    Obtains all active features preferences for a user. Disregards department, only includes active preferences and active common_reference_types.
    /// 
    /// #### Parameters:
    /// * user_id (i64): the id of the user for which the feature preferences are to be obtains
    /// * intervention_type_id (i64): type of intervention that will be used to limit the set of feature preferences retrieved
    /// * upper_limit (usize): number of preferences (upper limit) to be returned
    /// * intervention_level_only (bool): when true, only retrives intervention-level preferences
    /// 
    /// #### Returns:
    /// * Option< Vec<FeaturePreference> >: the Feature Preferences for the user, if found
    /// * std::io::Error: An error, if applicable
    /// 
    async fn get_active_feature_preferences_for_user_intervention_level(&self, user_id: i64,
                                                                               upper_limit_age_days: usize,
                                                                               upper_limit_num_prefs: usize)-> Result< Option< Vec<FeaturePreference> >, std::io::Error> {
        tracing::debug!("get_active_feature_preferences_for_user_intervention_level()");
        //println!("get_active_feature_preferences_for_user_intervention_level()");
         
        let query_level_0 =  db_query::QRY_ACTIVE_FEATURE_PREFERENCES_FOR_USER_INTERVENTION_LEVEL_ONLY;

        let query_level_1 = query_level_0.replace("{users_id}", &user_id.to_string());
        let query_level_2 = query_level_1.replace("{limit_days}", &upper_limit_age_days.to_string());
        let query_level_3 = query_level_2.replace("{feature_ids}", &constants::CRT_TOP_LEVEL_INTERVENTION_GRP_IDS.to_string());
        let query = query_level_3.replace("{limit_rows}", &upper_limit_num_prefs.to_string());

        //tracing::debug!("..SELECT sql: {}", query);
        //println!("..SELECT sql: {}", query);

        let rows: Vec<( i64, i32, i32,
                        NaiveDateTime, Option<i64>, i64, i64, String
         )> = sqlx::query_as(&query)
                                                .fetch_all(&self.connection) 
                                                .await
                                                .unwrap_or_default();
        if rows.is_empty() {  // this is an acceptable, known situation. The first time a user uses an FP, they will not already have it.
            return Ok( None );
        }
        else{
            let mut results: Vec<FeaturePreference> = Vec::with_capacity(rows.len());
            let mut lookup: HashSet<String> = HashSet::new();

            for row in rows {
                let tmp_feature_id: i64 = row.5;  //feature_id
                println!("....+ validating tmp_ref_group_id={} for addition to user={}",tmp_feature_id, user_id ); 

                let tmp_calculation_date: NaiveDateTime = row.3; // calculation_date
                //let tmp_department_id: i64 = match row.4 {  // NOTE 1: department_id usage will not be available for the final implementation 
                //    None => constants::INVALID_OTHER_ID,    // due to remaining timeline
                //    Some(dept_id) => dept_id
                //};

                let tmp_fp = FeaturePreference {
                    id: row.0,
                    display_order: i64::from(row.1),
                    weight: i64::from(row.2),
                    calculation_date: tmp_calculation_date,
                    department_id: constants::INVALID_OTHER_ID,  // tmp_department_id, SEE NOTE 1 ABOVE
                    users_id: user_id, // spelling in DTO matches DB
                    feature_id: row.5, 
                    ref_group_id: i64::from(row.6),
                    ref_name: row.7
                };

                //println!("...Adding ID={}", &tmp_fp.get_unique_key());
                results.push( tmp_fp.clone() );
                lookup.insert( tmp_fp.get_unique_key() );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }


    /// ### get_active_feature_preferences_for_user_intervention_details_level()
    ///    Obtains all active features preferences for a user. Disregards department, only includes active preferences and active common_reference_types.
    /// 
    /// #### Parameters:
    /// * user_id (i64): the id of the user for which the feature preferences are to be obtains
    /// * intervention_type_id (i64): type of intervention that will be used to limit the set of feature preferences retrieved
    /// * limits_days (i64): the maximum number of days to allow a FeaturePreference to be considered in the list of active entries
    /// * upper_limit (usize): number of preferences (upper limit) to be returned
    /// 
    /// * intervention_level_only (bool): when true, only retrives intervention-level preferences
    /// 
    /// #### Returns:
    /// * Option< Vec<FeaturePreference> >: the Feature Preferences for the user, if found
    /// * std::io::Error: An error, if applicable
    /// 
    pub async fn get_active_feature_preferences_of_intervention_details_for_user(&self, user_id: i64,
                                                                                       intervention_type_id: i64,
                                                                                       limits_days: i64,
                                                                                       upper_limit_num_prefs: usize)-> Result< Option< Vec<FeaturePreference> >, std::io::Error> {
        tracing::debug!("get_active_feature_preferences_for_user_intervention_details_level()");
     //   println!("get_active_feature_preferences_for_user_intervention_details_level()");
         
        let query_level_0 = db_query::QRY_ACTIVE_FEATURE_PREFERENCES_FOR_USER_INTERVENTION_DETAILS_LEVEL;
        let query_level_1 = query_level_0.replace("{users_id}", &user_id.to_string());
        let query_level_2 = query_level_1.replace("{limit_days}", &limits_days.to_string());
        let query_level_3 = query_level_2.replace("{feature_ids}", &intervention_type_id.to_string());
        let query = query_level_3.replace("{limit_rows}", &upper_limit_num_prefs.to_string());

        //tracing::debug!("..SELECT sql: {}", query);
        println!("..SELECT sql: {}", query);

        //id, display_order, weight,
        //  calculation_date, department_id, feature_id, ref_group_id, ref_name
        let rows: Vec<( i64, i32, i32,
                        NaiveDateTime, Option<i64>, i64, i64, String
         )> = sqlx::query_as(&query)
                                                .fetch_all(&self.connection) 
                                                .await
                                                .unwrap_or_default();
        if rows.is_empty() {
            //tracing::debug!("..Feature Prefs not found: user_id={} group_ids={}", user_id, &intervention_type_id.to_string());
            //println!("..Feature Prefs not found: user_id={} group_ids={}", user_id, &tmp_intv_types); 

            // this is an acceptable, known situation. The first time a user uses an FP, they will not already have it.
            return Ok( None );
        }
        else{
            let mut results: Vec<FeaturePreference> = Vec::with_capacity(rows.len());
            let mut lookup: HashSet<String> = HashSet::new();

            println!("..{} rows were returned", rows.len());

            for row in rows {
                println!("....+ validating tmp_ref_group_id={} for addition to user={}",&intervention_type_id.to_string(), user_id ); 

                let tmp_calculation_date: NaiveDateTime = row.3; // calculation_date
               // let tmp_department_id: i64 = match row.4 {
               //     None => constants::INVALID_OTHER_ID, // feature not implemented
               //     Some(dept_id) => dept_id
              //  };       

                let tmp_fp = FeaturePreference {
                    id: row.0,
                    display_order: i64::from(row.1),
                    weight: i64::from(row.2),
                    calculation_date: tmp_calculation_date,
                    department_id: constants::INVALID_OTHER_ID, // tmp_department_id, feature not implemented
                    users_id: user_id, // spelling in DTO matches DB
                    feature_id: row.5, 
                    ref_group_id: i64::from(row.6),
                    ref_name: row.7
                };

                results.push( tmp_fp.clone() );
                lookup.insert( tmp_fp.get_unique_key() );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }

    /// ### get_active_feature_preferences_of_interventions_for_user()
    ///
    /// Specialized wrapper for get_active_feature_preferences_for_user(user, TRUE)
    ///   Obtains all active features preferences for a user, that are at the intervention level only (common_reference_type.group_id=1).
    ///   Disregards department, only includes active preferences and active common_reference_types.
    /// 
    /// #### Parameters:
    /// * user_id (i64): the id of the user making the data request, for audit purposes
    /// * upper_limit (usize): number of preferences (upper limit) to be returned
    /// 
    /// #### Returns:
    /// * Option< Vec<FeaturePreference> >: the Feature Preferences for the user, if found
    /// * std::io::Error: An error, if applicable
    /// 
    pub async fn get_active_feature_preferences_of_interventions_for_user(&self, user_id: i64, upper_limit_num_prefs: usize)-> Result< Option< Vec<FeaturePreference> >, std::io::Error> {
        self.get_active_feature_preferences_for_user_intervention_level(user_id,constants::FEATURE_PREFERENCE_UPPER_AGE_LIMIT_DAYS, upper_limit_num_prefs).await // constants::CRT_ANY_INTERVENTION_GROUP,
    }

    /// ### upsert_feature_preference()
    ///
    /// Given an user_id and feature_id, create a new Feature Preference record, or update an existing one
    /// 
    /// #### Parameters:
    /// * user_id (i64): the id of the user for which the feature preference is being updated
    /// * feature_id (i64): id of the feature preference to be updated
    /// 
    /// #### Returns:
    /// * Option< i64 >: the id of the Feature Preferences that was updated, if found
    /// * std::io::Error: An error, if applicable
    /// 
    pub async fn upsert_feature_preference(&self, user_id: i64, feature_id: i64) ->  Result< Option< i64 >, std::io::Error> {
        tracing::debug!("upsert_feature_preference()");
        println!("upsert_feature_preference()");

        let query_level_0 = db_query::UPDATE_FEATURE_PREFERENCE.to_string();
    
        let query_level_1 = &query_level_0.replace("{users_id}", &user_id.to_string());
        let query = &query_level_1.replace("{feature_id}", &feature_id.to_string());

        // It is inconsequential if the user has a feature preference or not. As a result, we will assume they have a record first.
        //   If the ininital UPDATE fails, we perform an INSERT, which *should* succeed.
        //   However if it fails as well, we just carry on and do not interrupt the user with an error.
        //
        // tracing::debug!("..UPDATE sql: {}", query);
        //println!("..UPDATE sql: {}", query);  // tracing does not preserve formatting, making copy/paste useless
        let result = sqlx::query(&query)
                                                        .fetch_optional(&self.connection)
                                                        .await;
        match result.unwrap() {
            Some( outer_row  ) => Ok( outer_row.get("id") ),
            None => {
                let query_level_0 = db_query::INSERT_FEATURE_PREFERENCE.to_string();
                    
                let query_level_1 = &query_level_0.replace("{users_id}", &user_id.to_string());
                let query = &query_level_1.replace("{feature_id}", &feature_id.to_string());

                tracing::debug!("..INSERT sql: {}", query);
                println!("..INSERT sql: {}", query); // tracing does not preserve formatting, making copy/paste useless
                let qry_result = sqlx::query(&query)
                                                .fetch_optional(&self.connection)
                                                .await
                                                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
                match qry_result{
                    Ok(inner_result) => {
                        println!("....reached line 289");
                        let item = inner_result.unwrap();
                        Ok( item.get("id") )
                    },
                    Err(_) => {
                        println!("....reached line 294");
                        Ok( None ) 
                    },
                }
            }
        }
    }
}