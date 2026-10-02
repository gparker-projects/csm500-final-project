/// #Unit & Integration tests for the Convert Utils module
/// 
/// ###Includes:
/// * to_i64()
/// * is_aged()
///  CSM500 Project (April - October 2026)
///  Graham Parker (Student ID: 240120522)
/// -------------------------------------------------------------------

mod common;

#[cfg(test)]
mod convert_utils_tests {
    use maple_hms::constants;
    use maple_hms::{dto::user_auth::*};
    use maple_hms::session::*;
    use maple_hms::ui::tile_factory::{WebContentFactory};
    
    use actix_web::cookie::Key;
    use sqlx::postgres::{PgPoolOptions}; 

    const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

    /// ### test_convert_utils()
    /// 
    /// Tests the is_aged() function of the ConvertUtils strut
    /// 
    #[test]
    fn test_convert_utils() {
        assert!(maple_hms::dto::convert_utils::ConvertUtils::to_i64("not a number".to_string()) == constants::INVALID_OTHER_ID, "Not a number did not convert to -1");
        
        let earliest_birth_date = chrono::NaiveDate::from_ymd_opt(1880, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(); // very much aged
        let future = chrono::NaiveDate::from_ymd_opt(2030, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(); // very much aged

        assert!(maple_hms::dto::convert_utils::ConvertUtils::is_aged(chrono::Utc::now().naive_utc(), 0), "Timestamp was not aged, asserted as aged");
        assert!(maple_hms::dto::convert_utils::ConvertUtils::is_aged(earliest_birth_date, 4), "Timestamp was very aged, asserted as not aged");
        assert!(!maple_hms::dto::convert_utils::ConvertUtils::is_aged(future, 4), "Timestamp is in future, asserted as aged (past)");
    }

    /// ### test_sessions()
    /// 
    /// Tests the ability for a Session to be created/populated
    /// 
    ///   Specifically tests: session::UserSession::
    ///      new()
    ///      get_userid_as_i64()
    ///      get_user_display_name()
    ///      has_permission()
    #[test]
    fn test_sessions() {
        // create a couple of permission and put them in a Vector, then add to the UserAuthorization
        // the UserAuthorization gets put into the Session
        let perm: Permission = Permission::new(1, 1);
        let perm2: Permission = Permission::new(1, 2);

        let perms: Vec<Permission> = vec![perm, perm2];

        let ua = UserAuthorization {
            granted_permissions: perms
        };

        let sess: UserSession = UserSession {
            user_id: "1".to_string(), // 
            user_display_name: "TEST, UNIT".to_string(), //
            email: "test@gmail.com".to_string(), //
            user_authorizations: ua //
        };

        assert_eq!(sess.get_userid_as_i64(), 1);
        assert_eq!(sess.get_user_display_name(), "TEST, UNIT".to_string());
        assert_eq!(sess.has_permission(1), true);
        assert_eq!(sess.has_permission(9999), false);

        assert!(maple_hms::dto::convert_utils::ConvertUtils::to_i64("not a number".to_string()) == constants::INVALID_OTHER_ID, "Not a number did not convert to -1");
        
    }

    /// ### test_sys_config()
    /// 
    /// Tests the ability for a SysConfig to be created/populated
    /// 
    ///   Specifically tests: session::SysConfig::new() and ::get_max_general_fastactions()
    ///
    #[test]
    fn test_sys_config() {
        let cfg = SysConfig{
            max_general_fastactions: "1".to_string(),
            ..Default::default()
        };

        let val = cfg.get_max_general_fastactions();
        if val <= 0 {
            assert!(false);
        }
        else{
            assert!(true);
        }
    }

    /// ### test_app_session()
    /// 
    /// Tests the ability for an AppSession to be created/populated
    /// 
    ///   Specifically tests: session::AppSession::
    ///     new() 
    ///     get_db_connection()
    ///     get_full_path_language_model_file()
    ///     get_full_path_tokenizer_file()
    ///     get_full_path_command_mapping_file()
    ///     get_web_content_factory()
    ///
    #[tokio::test]
    async fn test_app_session() {
        // required to set up the WebContentFactory
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

        let tmp_cargo_manifest_dir = "\\data\\manifest_dir".to_string();
        let tmp_command_mapping_file = "command_mapping.csv".to_string();
        let tmp_language_model_file = "all-MiniLM-L6-v2.onnx".to_string();
        let tmp_tokenizer_file = "tokenizer.json".to_string();
        let tmp_model_data_dir = "\\data\\".to_string();
        let tmp_data_sub_dir = "\\data\\".to_string();

        // set up SysConfig
        let cfg = SysConfig{
            app_version: "v1.0Unit_test".to_string(),
            db_conn_str: DB_CONN_STR.to_string(),
            cargo_manifest_dir: tmp_cargo_manifest_dir, 
            model_data_dir: tmp_model_data_dir,
            command_mapping_file: tmp_command_mapping_file,
            language_model_file: tmp_language_model_file, 
            tokenizer_file: tmp_tokenizer_file, 
            data_sub_dir: tmp_data_sub_dir, 
            max_general_fastactions: "3".to_string(),
            max_nle_fastactions: "3".to_string(),
            website_bind_address: "10.10.10.10:8080".to_string(),
            session_key: "thisIsAVeryinauthenticSessionKeyOnlyToBeused_forunit_testing".to_string()
        };

        let tmp_key = Key::generate();

        // finally: set up AppSession
        let appsess = AppSession {
            wcf: tmp_wcf, 
            app_key: tmp_key,
            connection: db_pool,
            system_config: cfg
        };

        assert!( Some(appsess.get_db_connection()).is_some(), "AppSession did not have DBConnection established");

        assert_eq!( appsess.get_full_path_language_model_file(),  "\\data\\all-MiniLM-L6-v2.onnx".to_string(), "AppSession did not have language_model_file");
        assert_eq!( appsess.get_full_path_tokenizer_file(),       "\\data\\tokenizer.json".to_string(),        "AppSession did not have tokenizer_file");
        assert_eq!( appsess.get_full_path_command_mapping_file(), "\\data\\command_mapping.csv".to_string(),  "AppSession did not have command_mapping_file");

        assert!( Some(appsess.get_web_content_factory()).is_some(),  "AppSession did not properly store WCF");
    }
}