use actix_web::cookie::Key;
use sqlx::postgres::{PgPool};

use crate::webc::web_content::{WebContentFactory}; 
use crate::dto::{user_auth::*};
use std::sync::Arc;

///
/// Stores application-wide state/variables
/// REF: https://actix.rs/docs/application/
/// 
#[allow(dead_code)] // low priority warning; tired of seeing messages
pub struct AppSession {
    pub app_version: String,
    pub wcf: WebContentFactory,
    pub app_key: Key,
    pub connection: PgPool,
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
/// REF: https://docs.rs/actix-session/latest/actix_session/struct.SessionMiddleware.html
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