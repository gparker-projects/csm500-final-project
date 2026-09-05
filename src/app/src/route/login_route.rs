//! Login and related routes
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use actix_web::{web, Responder};
use actix_session::{Session}; 
use actix_web::http::StatusCode;
use tracing;

use crate::dao::{ auth_dao::*}; 
use crate::ui::{ data_forms::*};

use crate::session;

use crate::constants;

pub struct LoginRoute{}

impl LoginRoute{

    /// ### LoginRoute::login()
    ///   Route used to log a user into the application, authenticating and authorizing them, then 
    ///   redirecting to the home page.
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * req: web::Form<LoginFormData>: the request data for the user's login (userid and password)
    /// * user_session (actix_session::Session): the user's session
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    /// #### Refs
    /// * https://stackoverflow.com/questions/75369137/rust-actix-web-how-to-change-method-when-using-actix-webwebredirecttou
    /// * https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Redirections#temporary_redirections
    /// 
    pub async fn login(user_session: Session, req: web::Form<LoginFormData>, app_session: web::Data<session::AppSession>, ) -> impl Responder { // Box<dyn Responder<>> { //
        tracing::debug!("-> /login Requested");

        let cur_db_conn = AuthDAO::new( app_session.get_db_connection() ).await;
        let user_can_login = cur_db_conn.can_user_login(req.username.clone(), req.password.clone()).await.expect( constants::DATABASE_ERROR_NOT_FOUND );

        match user_can_login {
            Some (current_user) => {
            tracing::debug!("User can login: {} redirect to /home", req.username.clone());

            let uid: i64 = current_user.id;
            let user_perms = cur_db_conn.get_user_permissions( uid ).await.expect( constants::DATABASE_ERROR_NOT_FOUND ).unwrap();

            // initialize user session (this is the only location it can occur), for an authenticated user
            //  ref: https://docs.rs/actix-admin/latest/actix_admin/prelude/struct.Session.html
            // copy values from the db into the session; will use a different type of object than the DTO.user
            user_session.insert(constants::USER_SESSION, session::UserSession {
                user_id: current_user.id.to_string(), 
                user_display_name: current_user.name + " (" + &current_user.user_name + ")",
                email: current_user.email,
                user_authorizations: user_perms,
            }).expect("User Session could not be constructed");
            
            actix_web::web::Redirect::to("/home").using_status_code(StatusCode::SEE_OTHER) 
            }
            None => {
                tracing::debug!("Login denied for {} redirect back to /<default route>", req.username.clone()); // must use the user from the session as DB was not successful

                // do not PURGE before this; it will trash the session including this new key
                let _ignore = user_session.insert(constants::VALIDATION_ERRORS, "Invalid user or password. Please try again.");
                actix_web::web::Redirect::to("/").using_status_code(StatusCode::SEE_OTHER)
            }
        }
    }

    /// ### LoginRoute::logout()
    ///   Route provides a means for the user to exit the system, discarding the user session and redirecting to the login page.
    /// 
    /// #### Parameters:
    /// * user_session (actix_session::Session): the user's session
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    /// #### Refs
    /// * https://stackoverflow.com/questions/75369137/rust-actix-web-how-to-change-method-when-using-actix-webwebredirecttou
    /// * https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Redirections#temporary_redirections
    /// 
    pub async fn logout(user_session: Session ) -> impl Responder { // Box<dyn Responder<>> { //
        let _ignore = user_session.insert(constants::VALIDATION_ERRORS, "Invalid user or password. Please try again.");
        actix_web::web::Redirect::to("/").using_status_code(StatusCode::SEE_OTHER)
    }
}