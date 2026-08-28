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
use std::sync::Arc;
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
    pub data_sub_dir: String 
}

///
/// Stores application-wide state/variables
/// 
#[allow(dead_code)] // low priority warning; tired of seeing messages
pub struct AppSession {
    pub wcf: WebContentFactory,
    pub app_key: Key,
    pub connection: PgPool,
    pub system_config: SysConfig,
    pub nle_session: Arc<ort::session::Session>
}

impl AppSession {

  ///
  /// Returns a clone of the current web content factory
  /// 
  pub fn get_web_content_factory(&self) -> WebContentFactory{
      return self.wcf.clone();
  }

  ///
  /// Returns a clone of the current connection to the datbase
  /// 
  pub fn get_db_connection(&self) -> PgPool{
      return self.connection.clone();
  }

  ///
  /// Returns a cloned Arc thread of the NL engine session (from ort)
  /// 
  pub fn get_nle_session(&self) -> Arc<ort::session::Session>{
      return Arc::clone(&self.nle_session);
  }
}

///
/// Stores user session variables
/// 
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserSession {
    pub user_id: String,
    pub user_display_name: String,
    pub email: String,
    pub user_authorizations: UserAuthorization,   // for permissions and departments
    // current patients
    // preferences
}

impl UserSession {
  pub fn get_userid_as_i64(&self) -> i64{
      let result: i64 = self.user_id.parse().unwrap();
      return result;
  }
}