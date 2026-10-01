//! Default route (" / ")
//!
//!  CSM500 Project (April - October 2026)
//!  Graham Parker (Student ID: 240120522)

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 
use tracing;

use crate::ui::tile_factory::*;
use crate::session;
use crate::constants;

pub struct DefaultRoute{}

impl DefaultRoute{

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
                tracing::debug!("Ok(Some()) Validation errors present in session: {}", &validation_errors);
                // if the login form had validation errors, then we need to show them in the regenerated page.

                let mut content = wcf.get_tile(WebContentItem::WCTypeLoginTile); // retrieve the page base content

                // construct alternate content for the page
                let alt_content = "<label id=\"errLabel\" style=\"color: red\"><b>".to_owned() + &validation_errors + "</b>"; //.expect("User session invalid")
                content = content.replace("<label id=\"errLabel\">", &alt_content);   // retrieve validation errors; they are just raw text for now
                user_session.purge(); // minimize attack vectors by purging the session 

                HttpResponse::Ok().body( content )
            },
            Ok( None )=> {
                tracing::debug!("Ok( None ) No active session");
                HttpResponse::Ok().body( wcf.get_tile(WebContentItem::WCTypeLoginTile) )
            },
            Err(e)=> {
                tracing::error!("User session does not exist: {}", e);
                HttpResponse::Ok().body( wcf.get_tile(WebContentItem::WCTypeLoginTile) )
            },
        }
    }

}