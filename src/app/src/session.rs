use actix_web::cookie::Key;
use sqlx::postgres::{PgPoolOptions, PgPool};

use crate::webc::web_content::{WebContentFactory}; 
use crate::dto::{user_auth::*};
///
/// Stores application-wide state/variables
/// REF: https://actix.rs/docs/application/
/// 
#[allow(dead_code)] // low priority warning; tired of seeing it
pub struct AppSession {
    pub app_version: String,
    pub wcf: WebContentFactory,    //wcf: Mutex<WebContentFactory>,
    pub app_key: Key,
    pub connection: PgPool
}

impl AppSession {
  pub fn get_web_content_factory(&self) -> WebContentFactory{
      return self.wcf.clone();
  }

   pub fn get_db_connection(&self) -> PgPool{
      return self.connection.clone();
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