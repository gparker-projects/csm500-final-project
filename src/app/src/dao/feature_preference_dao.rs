//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use chrono::NaiveDateTime;
use std::collections::HashSet;
use sqlx::postgres::{PgPool}; 
use sqlx::Row;
use tracing;

use crate::{constants, dao::db_query};
use crate::dto::feature_preference::FeaturePreference;
use crate::dao::common_dao::CommonDAO;

#[derive(Debug, Clone)]
pub struct FeaturePreferenceDAO {
    pub connection: PgPool,
}

impl FeaturePreferenceDAO {
    /// Creates a new AuthObjects object, with a database pool for use by other calls
    /// 
    pub async fn new(db_connection: PgPool) -> Self {
        FeaturePreferenceDAO {
            connection: db_connection,
        }
    }

    ///
    /// Obtains all active features preferences for a user. Disregards department, only includes active preferences and active common_reference_types.
    /// 
    pub async fn get_active_feature_preferences_for_user(&self, user_id: i64,
                                                                intervention_type_id: i64,
                                                                upper_limit: usize,
                                                                intervention_level_only: bool)-> Result< Option< Vec<FeaturePreference> >, std::io::Error> {
        tracing::debug!("get_active_feature_preferences_for_user()");
        println!("get_active_feature_preferences_for_user()");
        let query_level_0 = db_query::QRY_GET_ALL_ACTIVE_FEATURE_PREFERENCE_FOR_USER;
        let query_level_1 = query_level_0.replace("{users_id}", &user_id.to_string());
        let query_level_2 = query_level_1.replace("{limit_days}", &"14".to_string());


        // TODO

        let tmp_intv_types = match intervention_type_id {
            constants::CRT_ANY_INTERVENTION_GROUP => "1, 3".to_string(), // groups 1 and 3 are Clinical, non-Clinical intervention types
            _ => intervention_type_id.to_string()
        };
        let query_level_3 = query_level_2.replace("{feature_ids}", &tmp_intv_types);
        let query = query_level_3.replace("{limit_rows}", &"3".to_string());

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
            tracing::debug!("..Feature Preferences not found for user_id={}", user_id);
            //println!("..Feature Preference entries not found for user_id={}", user_id);
            return Ok( None );
        }
        else{
            let mut results: Vec<FeaturePreference> = Vec::with_capacity(rows.len());
            let mut lookup: HashSet<String> = HashSet::new();
            let mut counter: usize = 0;

            for row in rows {
                let tmp_feature_id: i64 = row.5;  //feature_id
                let tmp_ref_group_id: i64 = i64::from(row.6); // ref_group_id

                // only collect items that are a) an intervention, when only interventions are requested
                //  or b) everything other than intervention-level, when no interventions are wanted
                let is_intv = CommonDAO::is_intervention_group_type(tmp_ref_group_id);
                if (!intervention_level_only && !is_intv) || (intervention_level_only && is_intv){

                    //println!("....> Adding" ); 
                    let tmp_id: i64 = row.0; // id
                    let tmp_display_order: i64 = i64::from(row.1); //  display_order
                    let tmp_weight: i64 = i64::from(row.2);  //weight
                    let tmp_calculation_date: NaiveDateTime = row.3; // calculation_date
                    let tmp_department_id: i64 = match row.4 {
                        None => constants::INVALID_OTHER_ID,
                        Some(dept_id) => dept_id
                    };       

                    let tmp_ref_name: String = row.7; // ref_name

                    let tmp_fp = FeaturePreference {
                        id: tmp_id,
                        display_order: tmp_display_order,
                        weight: tmp_weight,
                        calculation_date: tmp_calculation_date,
                        department_id: tmp_department_id,
                        users_id: user_id, // spelling in DTO matches DB
                        feature_id: tmp_feature_id, 
                        ref_group_id: tmp_ref_group_id,
                        ref_name: tmp_ref_name
                    };
                    //tracing::warn!("..Evaluating Pref ID={}", &tmp_fp.get_unique_key());

                    if !lookup.contains( &tmp_fp.get_unique_key() ) && counter < upper_limit{
                        //tracing::debug!("...Adding ID={}", &tmp_fp.get_unique_key());
                        results.push( tmp_fp.clone() );
                        lookup.insert( tmp_fp.get_unique_key() );
                        counter = counter + 1;
                    }
                    else{
                        tracing::debug!("..Not Adding ID={}", &tmp_fp.get_unique_key());
                    }
                }
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }


    ///
    /// Specialized wrapper for get_active_feature_preferences_for_user(user, TRUE)
    /// 
    /// 
    /// Obtains all active features preferences for a user, that are at the intervention level only (common_reference_type.group_id=1).
    /// Disregards department, only includes active preferences and active common_reference_types.
    /// 
    pub async fn get_active_feature_preferences_of_interventions_for_user(&self, user_id: i64, upper_limit: usize)-> Result< Option< Vec<FeaturePreference> >, std::io::Error> {
        self.get_active_feature_preferences_for_user(user_id, constants::CRT_ANY_INTERVENTION_GROUP, upper_limit, true).await
    }

    ///
    /// Specialized wrapper for get_active_feature_preferences_for_user(user, FALSE)
    /// 
    /// Obtains all active features preferences for a user, that are not at the intervention level only (common_reference_type.group_id <> 1).
    /// Disregards department, only includes active preferences and active common_reference_types.
    /// 
    pub async fn get_active_feature_preferences_of_intervention_details_for_user(&self, user_id: i64, intervention_type_id: i64, upper_limit: usize)-> Result< Option< Vec<FeaturePreference> >, std::io::Error> {
        self.get_active_feature_preferences_for_user(user_id, intervention_type_id, upper_limit, false).await
    }

    //https://users.rust-lang.org/t/calling-stored-procedures-setting-parameters-and-returning-parameters-on-postgresql/91508/4

    ///
    /// Given an user_id and feature_id, create a new Feature Preference record, or update an existing one
    /// RETURNS: i64: the id of the Intervention record that is created, if applicable
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
        tracing::debug!("..UPDATE sql: {}", query);
        println!("..UPDATE sql: {}", query);  // tracing does not preserve formatting, making copy/paste useless
        let result = sqlx::query(&query)
                                                        .fetch_optional(&self.connection)
                                                        .await
                                                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        match result {
            Some( outer_row  ) => {
               Ok( outer_row.get("id") )
            },
            None => {
                let query_level_0 = db_query::INSERT_FEATURE_PREFERENCE.to_string();
                    
                let query_level_1 = &query_level_0.replace("{users_id}", &user_id.to_string());
                let query = &query_level_1.replace("{feature_id}", &feature_id.to_string());

                tracing::debug!("..INSERT sql: {}", query);
                println!("..INSERT sql: {}", query); // tracing does not preserve formatting, making copy/paste useless

                let inner_result = sqlx::query(&query)
                                                .fetch_optional(&self.connection)
                                                .await
                                                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                match inner_result {
                    Some( inner_row  ) => {
                        Ok( inner_row.get("id") )
                    },
                    None => {
                        tracing::debug!(" >> Feature Preference insert failed user_id={} feature_id={}", user_id, feature_id);
                        Ok( Some(constants::INVALID_OTHER_ID) ) 
                    }
                }
            }
        }

    }
}