//! ---------------------------------------------------------------------------------
//! Application and user session structs to hold persistent information and 
//! connections that will be shared across the application to all users, or for
//! only a specific user, across all their interactions.
//!
//! CSM500 Project (April - October 2026)
//! Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//!  For application-wide state/variables: 
//!  https://actix.rs/docs/application/
//! 
//!  For Stores user session variables:
//!  https://docs.rs/actix-session/latest/actix_session/struct.SessionMiddleware.html
//! 
//! ---------------------------------------------------------------------------------

use actix_web::cookie::Key;
use serde::Deserialize;
//use std::sync::Arc;
use std::path::Path;
use sqlx::postgres::{PgPool};

use crate::ui::tile_factory::{WebContentFactory}; 
use crate::dto::{user_auth::*};
use std::fs::read_to_string;

use std::env::VarError;

use crate::constants;

///
/// Stores application-wide configuration loaded at startup
/// 
#[derive(Deserialize, Default, Clone)]
pub struct SysConfig {
    pub app_version: String,
    pub db_conn_str: String,
    pub cargo_manifest_dir: String, // provided by the application after load; do not actually set in the TOML
    pub model_data_dir: String,     //   same as prior
    pub command_mapping_file: String,
    pub language_model_file: String,
    pub tokenizer_file: String,
    pub data_sub_dir: String,
    pub max_general_fastactions: String,
    pub max_nle_fastactions: String, // not implemented for use at this time
    pub website_bind_address: String,
    pub session_key: String,
    pub max_age_feature_preferences: String,
}

impl SysConfig{
    /// ### get_max_general_fastactions()
    ///  Basic accessor that converts the max_general_fastactions to a usize (generic integer)
    ///  by unwrapping and parsing the attribute from a String type.
    /// 
    /// #### Returns:
    /// * usize: max_general_fastactions as usize (generic integer)
    /// 
    pub fn get_max_general_fastactions(&self) -> usize{
        self.max_general_fastactions.parse().unwrap()
    }

    /// ### get_max_age_feature_preferences()
    ///  Basic accessor that converts the max_age_feature_preferences to a usize (generic integer)
    ///  by unwrapping and parsing the attribute from a String type.
    /// 
    /// #### Returns:
    /// * usize: max_age_feature_preferences as usize (generic integer)
    /// 
    pub fn get_max_age_feature_preferences(&self) -> i64{
        self.max_age_feature_preferences.parse().unwrap()
    }

    /// ### session::SysConfig::new()
    ///   Reads the system configuration file from a static path... so it is the only one we need to do this from
    ///  the rest of the config settings are in this config file, eliminating many constants otherwise requird by the application
    /// 
    /// #### Parameters: None
    /// 
    /// #### Returns:
    /// * SysConfig: an initialized SysConfig instance
    /// 
    pub fn new(config_path: Result<String, VarError>) -> Self {
        // collect the cargo manifest directory at runtime, which means it might not be present
        let cargo_manifest_dir = match config_path {
            Ok(tmp_path) => {
                tracing::info!("CARGO_MANIFEST_DIR = {}", tmp_path);
                tmp_path
            }
            Err(e) => {
                tracing::error!("CARGO_MANIFEST_DIR not set: {}", e);
                "INVALID_PATH".to_string()
            }
        };

        let base_model_data_dir = cargo_manifest_dir.clone()  + constants::DATA_SUB_DIRECTORY;
        let toml_config_file = cargo_manifest_dir.clone()  + constants::SYSTEM_CONFIGURATION_FILE;

        let toml_config_str = read_to_string(toml_config_file.clone()); 
        let mut final_config: SysConfig = Default::default();

        match toml_config_str {
            Ok(results) => {

                let tmp_config = toml::from_str::<SysConfig>( &results );
                match tmp_config {
                    Ok(ok_config) => {
                        tracing::info!("Configuration loaded: {}", toml_config_file.clone());
                        final_config = ok_config;
                    }
                    Err(e) => {
                        tracing::error!("Error reading from TOML ({}): {}", toml_config_file.clone(), e);
                        eprintln!("Error reading from TOML ({}): {}", toml_config_file.clone(), e);
                    },
                }
            }
            Err(e) => {
                tracing::error!("Error reading from TOML ({}): {}", toml_config_file.clone(), e);
                eprintln!("Error reading from TOML ({}): {}", toml_config_file.clone(), e);
            },
        }
        final_config.cargo_manifest_dir = cargo_manifest_dir; // override some of the values, with setting obtained elsewhere in by the system
        final_config.model_data_dir = base_model_data_dir;

        final_config
    }

    /// ### fn get_application_secret_key()
    ///   Provides the secret key for the application, usually from a config file (TODO)
    /// 
    /// #### Referencees:
    ///  https://docs.rs/actix-web/latest/actix_web/cookie/struct.Key.html
    /// 
    /// #### Parameters: None
    /// 
    /// #### Returns: 
    /// * Key: the Key obtained from the actix_web::cookie::Key class
    /// 
   pub fn get_application_secret_key(&self) -> Key {
        tracing::info!(">get_application_secret_key()");

        actix_web::cookie::Key::from(
        std::env::var("SESSION_KEY")
            .unwrap_or_else(|_| self.clone().session_key )
            .as_bytes()
        )
    }
}

///
/// Stores application-wide state/variables
/// 
#[allow(dead_code)] // low priority warning; tired of seeing messages
pub struct AppSession {
    pub wcf: WebContentFactory,
    pub app_key: Key,
    pub connection: PgPool,
    pub system_config: SysConfig//,
    //pub nle_session: Arc<ort::session::Session>
}

impl AppSession {

  /// ### get_db_connection()
  ///    Accessor returns a clone of the current web content factory
  /// 
  /// #### Returns:
  /// * WebContentFactory: the current web content factory instance
  /// 
  pub fn get_web_content_factory(&self) -> WebContentFactory{
      return self.wcf.clone();
  }

  /// ### get_db_connection()
  ///    Accessor rReturns a clone of the current connection to the datbase
  /// 
  /// #### Returns:
  /// * PgPool: the current database connection as a PgPool
  /// 
  pub fn get_db_connection(&self) -> PgPool{
      return self.connection.clone();
  }

  /// ### get_full_path_language_model_file()
  ///    Accessor returns a full path to the language model file 
  /// 
  /// #### Returns:
  /// * String: the fill path to the language model file
  /// 
  pub fn get_full_path_language_model_file(&self) -> String {
      Path::new( &self.system_config.cargo_manifest_dir.clone()  )
                .join(self.system_config.data_sub_dir.clone())
                 .join(self.system_config.language_model_file.clone()).to_string_lossy().to_string()
  }

  /// ### get_full_path_tokenizer_file()
  ///    Accessor returns a full path to the tokenizer file 
  /// 
  /// #### Returns:
  /// * String: the fill path to the tokenizer file
  /// 
  pub fn get_full_path_tokenizer_file(&self) -> String {
      Path::new(&self.system_config.cargo_manifest_dir.clone()  )
                .join(self.system_config.data_sub_dir.clone())
                 .join(self.system_config.tokenizer_file.clone()).to_string_lossy() .to_string()
  }

  /// ### get_full_path_command_mapping_file()
  ///    Accessor returns a full path to the command mapping file 
  /// 
  /// #### Returns:
  /// * String: the fill path to the mapping file
  /// 
  pub fn get_full_path_command_mapping_file(&self) -> String {
      Path::new( &self.system_config.cargo_manifest_dir.clone()  )
                .join(self.system_config.data_sub_dir.clone())
                .join(self.system_config.command_mapping_file.clone()).to_string_lossy().to_string()
  }

  /*
  ///
  /// Returns a cloned Arc thread of the NL engine session (from ort)
  /// 
  pub fn get_nle_session(&self) -> Arc<ort::session::Session>{
      return Arc::clone(&self.nle_session);
  }*/
}

///
/// Stores user session variables for the application
/// 
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct UserSession {
    pub user_id: String,
    pub user_display_name: String,
    pub email: String,
    pub user_authorizations: UserAuthorization,   // for permissions and departments
}

impl UserSession {

    /// ### get_userid_as_i64()
    ///    Accessor helper method returns the user_id of the UserSession, as an i64 (default is string). 
    ///    This comes up a lot in the code, as we are using ids as keys to reference our entities.
    /// 
    /// #### Returns:
    /// * i64: the id of the user (from the self.user_id attribute), as an i64
    /// 
    pub fn get_userid_as_i64(&self) -> i64{
        let result: i64 = self.user_id.parse().unwrap();
        return result;
    }

    /// ### get_user_display_name()
    ///    Accessor returns the user_display_name field as a cloned String. This saves other methods
    ///    from having to later clone the session.
    /// 
    /// #### Returns:
    /// * String: the content of the user_display_name attribute
    /// 
    pub fn get_user_display_name(&self) -> String{
        return self.user_display_name.clone();
    }

    /// ### has_permission()
    ///    Shortcut extended accessor method gives more direct access to the has_permission() method
    ///    of the (UserAuthoriation) struct being held.
    /// 
    /// #### Parameters:
    /// * permission_id (i64): the id of the Permission which were are checking for the user to have
    /// 
    /// #### Returns:
    /// * bool: true/false for if the user does/does not have the permission
    /// 
    pub fn has_permission(&self, permission_id: i64) -> bool{
        let ua = self.user_authorizations.clone();

        return ua.has_permission( permission_id );
    }
}