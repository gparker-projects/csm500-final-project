//! ---------------------------------------------------------------------------------
//! Application and user session structs to hold persistent information and 
//!   connections that will be shared across the application to all users, or for
//!   only a specific user, across all their interactions.
//!
//!      CSM500 Project (April - October 2026)
//!         Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//!   For application-wide state/variables: 
//!     https://actix.rs/docs/application/
//! 
//!   For Stores user session variables:
//!     https://docs.rs/actix-session/latest/actix_session/struct.SessionMiddleware.html
//! 
//! ---------------------------------------------------------------------------------

use actix_web::cookie::Key;
use serde::Deserialize;
//use std::sync::Arc;
use std::path::Path;
use sqlx::postgres::{PgPool};

use crate::ui::tile_factory::{WebContentFactory}; 
use crate::dto::{user_auth::*};

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
}

impl SysConfig{
    ///
    /// Basic accessor that converts the max_general_fastactions to a usize (generic integer)
    /// 
    pub fn get_max_general_fastactions(&self) -> usize{
        self.max_general_fastactions.parse().unwrap()
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