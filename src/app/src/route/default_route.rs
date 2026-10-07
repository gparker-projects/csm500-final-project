//! ---------------------------------------------------------------------------------
//! Default route (" / "), 404 Not found and a server monitoring (is it up?) route.
//!
//!  CSM500 Project (April - October 2026)
//!  Graham Parker (Student ID: 240120522)
//! 
//! ---------------------------------------------------------------------------------
use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 
use actix_web::http::StatusCode;
use tracing;

use crate::ui::tile_factory::*;
use crate::session;
use crate::constants;

pub struct BasicRoute{}

impl BasicRoute{

    /// ### is_it_up_route()
    /// 
    /// Allows a monitoring services to perform a basic "is the application up?" check
    /// 
    /// #### Parameters: None
    /// 
    /// #### Returns: 
    /// * Responder: the general responder that allows the system to report system is up using an Ok() response
    /// 
    pub async fn is_it_up_route() -> impl Responder {
        tracing::info!("-> /isItUp Requested");
        HttpResponse::Ok().body("MapleHMS is Up")
    }

    /// ### error_route()
    /// 
    /// Allows a monitoring services to perform a basic "is the application up?" check
    /// 
    /// #### Parameters: None
    /// 
    /// #### Returns: 
    /// * Responder: the general responder that allows the system to report system is up using an Ok() response
    /// 
    pub async fn error_route() -> impl Responder {
        tracing::info!("MapleHMS is down");
        actix_web::web::Redirect::to("/home").using_status_code(StatusCode::SERVICE_UNAVAILABLE)
    }

    /// ### not_found_route()
    ///   Route for processing resource not found / 404 errors
    /// 
    /// #### Parameters: None
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    pub async fn not_found_route() -> impl Responder {
        tracing::info!("-> Error 404 route");
        actix_web::web::Redirect::to("/home").using_status_code(StatusCode::SEE_OTHER)
    }

    /// ### DefaultRoute::default_route()
    ///   The default route when nothing else is specified by the user
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * user_session (actix_session::Session): the user's session
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    /// #### Refs
    ///  https://actix.rs/docs/application/
    /// 
    pub async fn default_route(app_session: web::Data<session::AppSession>, user_session: Session) -> impl Responder {
        tracing::debug!("-> /default_route Requested");
        
        let wcf = &app_session.get_web_content_factory(); 
        tracing::debug!("Checking session for Validation errors");
        
        match user_session.get::<String>(constants::VALIDATION_ERRORS){
            Ok(Some(validation_errors))=> {
                tracing::debug!("..Ok(Some()) Validation errors present in session");
                //println!("Ok(Some()) Validation errors present in session: {}", &validation_errors);
                // if the login form had validation errors, then we need to show them in the regenerated page.

                let mut content = wcf.get_tile(WebContentItem::WCTypeLoginTile); // retrieve the page base content

                // construct alternate content for the page
                let alt_content = "<label id=\"errLabel\" style=\"color: red\"><b>".to_owned() + &validation_errors + "</b>"; //.expect("User session invalid")
                content = content.replace("<label id=\"errLabel\">", &alt_content);   // retrieve validation errors; they are just raw text for now
                user_session.purge(); // minimize attack vectors by purging the session 

                HttpResponse::Ok().body( content )
            },
            _ => {
                tracing::debug!("..Ok( None ) No active session");
                HttpResponse::Ok().body( wcf.get_tile(WebContentItem::WCTypeLoginTile) )
            },
        }
    }

}