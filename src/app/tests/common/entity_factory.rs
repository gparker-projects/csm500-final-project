/// ---------------------------------------------------------------------------------
/// Provides a means to create quick/easy entities for testing, for use with test case execution
/// 
/// NOTE: Comments are intentionally brief, as this is not primary system code
///  CSM500 Project (April - October 2026)
///  Graham Parker (Student ID: 240120522)
/// -------------------------------------------------------------------

use maple_hms::constants;
use maple_hms::dao::patient_dao::PatientWrapper;
use maple_hms::dto::encounter::Encounter;
use maple_hms::dto::feature_preference::FeaturePreference;
use maple_hms::dto::intervention::Intervention;
use maple_hms::dto::intervention_detail::InterventionDetail;
use maple_hms::dto::patient::Patient;
use maple_hms::dto::user_auth::{Permission, UserAuthorization};
use maple_hms::session::{UserSession};

use maple_hms::ui::tile_factory::WebContentFactory;
use maple_hms::session::*;
use actix_web::cookie::Key;
use sqlx::postgres::PgPoolOptions;
//use maple_hms::dto::user_auth::*;
use actix_web::web;

pub const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

#[cfg(test)]

pub struct EntityFactory;

#[cfg(test)]
impl EntityFactory{

    ///
    /// Returns a dto::Patient entity, with -1 as the Id. All other fields are defaulted.
    ///
    #[allow(dead_code)]
    pub fn create_patient() -> Patient {
        use maple_hms::constants;

        return Patient {
            id: constants::INVALID_OTHER_ID,
            ..Default::default()
        }
    }

    ///
    /// Returns a dto::Encounter entity, with -1 as the Id. All other fields are defaulted.
    ///
    #[allow(dead_code)]
    pub fn create_encounter() -> Encounter {
        return Encounter {
            id: constants::INVALID_OTHER_ID,
            ..Default::default()
        }
    }

    ///
    /// Returns a dto::Intervention entity, with -1 as the Id. All other fields are defaulted.
    ///
    #[allow(dead_code)]
    pub fn create_intervention() -> Intervention {
        return Intervention {
            id: constants::INVALID_OTHER_ID,
            ..Default::default()
        }
    }

    ///
    /// Returns a dto::Intervention Detail entity, with -1 as the Id. All other fields are defaulted.
    ///
    #[allow(dead_code)]
    pub fn create_intervention_detail() -> InterventionDetail {
        return InterventionDetail {
            id: constants::INVALID_OTHER_ID,
            ..Default::default()
        }
    }

    ///
    /// Returns a vector of 10 dao::Patient entities
    ///
    #[allow(dead_code)]
    pub fn create_vector_of_patients() -> Vec<Patient> {
        let mut results = Vec::new();
        //let pid = 10101; // common patient id for all records

        for i in 1..11 {
            let mut p = EntityFactory::create_patient(); 
            p.id = i;
            results.push(p);
        }
        return results; 
    }

    ///
    /// Returns a vector of 10 dao::PatientWrapper entities, linking the entities using a common patient id (10101)
    ///
    #[allow(dead_code)]
    pub fn create_vector_of_patient_wrappers() -> Vec<PatientWrapper> {
        let mut results = Vec::new();
        let pid = 10101; // common patient id for all records

        for i in 1..11 {
            results.push(EntityFactory::create_patient_wrapper_with_ids(pid, i*1000, i*1000+i));
        }
        return results; 
    }

    ///
    /// Returns a vector of 10 dao::Encounter entities, linking the entities using a common patient id (20202)
    ///
    #[allow(dead_code)]
    pub fn create_vector_of_encounters() -> Vec<Encounter> {
        let mut results = Vec::new();
        let pid = 20202; // common patient id for all records

        for i in 1..11 {
            let mut e = EntityFactory::create_encounter();
            e.patient_id = pid;
            e.id = i;
            results.push(e);
        }
        return results; 
    }

    ///
    /// Returns a vector of 10 dao::Intervention entities, linking the entities using a common encounter id (30303)
    ///
    #[allow(dead_code)]
    pub fn create_vector_of_interventions() -> Vec<Intervention> {
        let mut results = Vec::new();
        let enc_id = 30303; // common Encounter id for all records

        for i in 1..11 {
            let mut obj = EntityFactory::create_intervention();
            obj.encounter_id = enc_id;
            obj.id = i*1000;
            results.push(obj);
        }
        return results; 
    }

    ///
    /// Returns a vector of 10 dao::Intervention Details entities, linking the entities using a common encounter id (40404)
    ///
    #[allow(dead_code)]
    pub fn create_vector_of_intervention_details() -> Vec<InterventionDetail> {
        let mut results = Vec::new();
        let intv_id = 40404; // common Intervention Details id for all records

        for i in 1..11 {
            let mut obj = EntityFactory::create_intervention_detail();
            obj.intervention_id = intv_id;
            obj.id = i*1000;
            results.push(obj);
        }
        return results; 
    }    
    
    ///
    /// Returns a dao::PatientWrapper entity, linking the entities using a standard Id (1000)
    ///
    #[allow(dead_code)]
    pub fn create_patient_wrapper() -> PatientWrapper {
        return EntityFactory::create_patient_wrapper_with_ids(1000, 1000, 1000); // link all of the records together
    }

    ///
    /// Returns a dao::PatientWrapper entity, using ids that have been provided for subordinate entities
    ///
    #[allow(dead_code)]
    pub fn create_patient_wrapper_with_ids(p_id: i64, e_id: i64, i_id: i64) -> PatientWrapper {
        let mut p = EntityFactory::create_patient();
        let mut e = EntityFactory::create_encounter();
        let mut i = EntityFactory::create_intervention();
        p.id = p_id;
        e.id = e_id;
        i.id = i_id;

        return PatientWrapper {
            patient: p,
            current_encounter: e,
            most_recent_intervention: Some(i),
        }
    }


    ///
    /// Create a couple of Permissions and put them in a vector, then add to the UserAuthorization.
    /// The UserAuthorization gets put into the UserSession
    /// 
    #[allow(dead_code)]
    pub fn create_user_session() -> UserSession {
        let perm: Permission = Permission::new(1, 1);
        let perm2: Permission = Permission::new(1, 2);
        let perm3: Permission = Permission::new(1, Permission::ALLOW_CREATE_UPDATE_ADMIT);

        let ua = UserAuthorization {
            granted_permissions: vec![perm.clone(), perm2.clone(), perm3]
        };

        return UserSession {
            user_id: "5".to_string(), // 
            user_display_name: "TEST, UNIT".to_string(), //
            email: "test@gmail.com".to_string(), //
            user_authorizations: ua //
        }
    }

    ///
    /// Creates a vector of tuples (i64, String, String) representing interventions, for testing.
    /// 
    #[allow(dead_code)]
    pub fn create_intervention_type_list(short_identifier: String, unique_index: i64) -> Vec<(i64, String, String)>{
        let mut items = Vec::new();
        
        for i in 1..11 {
            let str: String; 
            if i == unique_index {
                str = "UNIT TEST-".to_owned() + &short_identifier + "-" + &i.to_string();            
            }
            else{
                str = "UNIT TEST ".to_owned() + &short_identifier + " " + &i.to_string();
            }
            items.push( (i, str.clone(), str.clone()) );
        }
        items
    }

    ///
    /// Creates a vector of tuples (i64, String) representing locations, for testing.
    /// 
    #[allow(dead_code)]
    pub fn create_location_list(unique_index: i64) -> Vec<(i64,  String)>{
        let mut items = Vec::new();
        for i in 1..11 {
            let str: String; 
            if i == unique_index {
                str = "UNIT TEST-Location-".to_owned() + &i.to_string();          
            }
            else{
                str = "UNIT TEST Location ".to_owned() + &i.to_string();
            }
            items.push( (i, str.clone()) );
        }
        items
    }

    ///
    /// Creates a vector of FeaturePreferences for testing.
    /// 
    #[allow(dead_code)]
    pub fn create_vector_of_feature_preferences() -> Vec<FeaturePreference>{
        let mut items = Vec::new();
        
        for i in 1..11 {
            let fp = FeaturePreference{
               id: i,
               display_order: 1,
               weight: 1,
               calculation_date: chrono::Utc::now().naive_utc(),
               users_id: 1,
               department_id: 1,
               feature_id: 1,
               ref_group_id: 1,
               ref_name: "UNIT TEST 1".to_string(),
            };
            items.push( fp );
        }
        items
    }

    /// ### get_mock_user_session() 
    /// 
    /// Supporting method that sets up an initial mock user session for use by the main tests
    /// 
    pub fn get_mock_user_session(user_id: String) -> UserSession{
        let perm: Permission = Permission::new(1, 1);   // create some base permissions
        let perm2: Permission = Permission::new(1, Permission::ALLOW_CREATE_CLINICAL_INTERVENTION);
        let perms: Vec<Permission> = vec![perm, perm2]; // put them in a vector
        let ua = UserAuthorization { // the vector gets put into the UserAuthorization
            granted_permissions: perms
        };

        UserSession { // the UserAuthorization gets  put into the UserSession
            user_id: user_id, 
            user_display_name: "TEST, UNIT".to_string(), 
            email: "test@gmail.com".to_string(), 
            user_authorizations: ua 
        }
    }

    /// ### get_mock_app_session() 
    /// 
    /// Supporting method that sets up an initial mock application session for use by the main tests
    /// 
    pub async fn get_mock_app_session() -> web::Data<maple_hms::session::AppSession> {
    // a lot of set up to mimic a live system session
        let path = std::env::current_dir().expect("Base path to executable could not be found");
        let newpath = path.display().to_string() + "\\webc\\static\\";
        let tmp_wcf = WebContentFactory::new(&newpath, "UNIT TEST".to_string());

        // set up PgPool
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(DB_CONN_STR)
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                tracing::debug!("{}", e);
                assert!(false);
                panic!("{}", e)
            },
        };

        // set up SysConfig
        let cfg = SysConfig{
            app_version: "v1.0Unit_test".to_string(),
            db_conn_str: DB_CONN_STR.to_string(),
            cargo_manifest_dir: "\\data\\manifest_dir".to_string(), 
            model_data_dir: "\\data\\".to_string(),
            command_mapping_file: "command_mapping.csv".to_string(),
            language_model_file: "all-MiniLM-L6-v2.onnx".to_string(), 
            tokenizer_file: "tokenizer.json".to_string(), 
            data_sub_dir: "\\data\\".to_string(), 
            max_general_fastactions: "3".to_string(),
            max_nle_fastactions: "3".to_string(),
            website_bind_address: "10.10.10.10:8080".to_string(),
            session_key: "thisIsAVeryinauthenticSessionKeyOnlyToBeused_forunit_testing".to_string(),
            max_age_feature_preferences: "30".to_string()
        };

        // finally: return the initialized AppSession
        web::Data::new(AppSession {
            wcf: tmp_wcf, 
            app_key: Key::generate(),
            connection: db_pool,
            system_config: cfg
        })
    }

     /// ### get_mock_app_session() 
    /// 
    /// Supporting method that sets up an initial mock application session for use by the main tests
    /// 
    pub async fn get_mock_nle_app_session(base_path: String) -> web::Data<maple_hms::session::AppSession> {
        // a lot of set up to mimic a live system session
        let wcf_path = std::env::current_dir().expect("Base path to executable could not be found").display().to_string() + "\\webc\\static\\";
        let tmp_wcf = WebContentFactory::new(&wcf_path, "UNIT TEST".to_string());

        // set up PgPool
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(DB_CONN_STR)
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                tracing::debug!("{}", e);
                assert!(false);
                panic!("{}", e)
            },
        };

        // set up SysConfig
        let cfg = SysConfig{
            app_version: "v1.0Unit_test".to_string(),
            db_conn_str: DB_CONN_STR.to_string(),
            cargo_manifest_dir: base_path.clone() + &"\\data\\manifest_dir".to_string(), 
            model_data_dir: base_path.clone() + &"\\data\\".to_string(),
            command_mapping_file: base_path.clone() + &"command_mapping.csv".to_string(),
            language_model_file: base_path.clone() + &"all-MiniLM-L6-v2.onnx".to_string(), 
            tokenizer_file: base_path.clone() + &"tokenizer.json".to_string(), 
            data_sub_dir: "\\data\\".to_string(), 
            max_general_fastactions: "3".to_string(),
            max_nle_fastactions: "3".to_string(),
            website_bind_address: "10.10.10.10:8080".to_string(),
            session_key: "thisIsAVeryinauthenticSessionKeyOnlyToBeused_forunit_testing".to_string(),
            max_age_feature_preferences: "30".to_string()
        };

        // finally: return the initialized AppSession
        web::Data::new(AppSession {
            wcf: tmp_wcf, 
            app_key: Key::generate(),
            connection: db_pool,
            system_config: cfg
        })
    }

}
