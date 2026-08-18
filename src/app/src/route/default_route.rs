//! # Home and related routes
//!
//!      CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 

use crate::webc::web_content::*;
use crate::session;
use crate::constants;

pub struct DefaultRoute{}

impl DefaultRoute{

    ///
    /// default route when nothing else is specified by the user
    ///
    pub async fn default_route(app_session: web::Data<session::AppSession>, user_session: Session) -> impl Responder {
        println!("-> /default_route Requested");
        
        let wcf = &app_session.get_web_content_factory(); 
        println!("Checking session for Validation errors");
        
        match user_session.get::<String>(constants::VALIDATION_ERRORS){
            Ok(Some(validation_errors))=> {
            println!("Ok(Some()) Validation errors present in session: {}", &validation_errors);
            // if the login form had validation errors, then we need to show them in the regenerated page.

            let mut content = wcf.get_tile(WebContentItem::WCTypeLoginTile); // retrieve the page base content

            // construct alternate content for the page
            let alt_content = "<label id=\"errLabel\" style=\"color: red\"><b>".to_owned() + &validation_errors + "</b>"; //.expect("User session invalid")
            content = content.replace("<label id=\"errLabel\">", &alt_content);   // retrieve validation errors; they are just raw text for now

            user_session.purge(); // minimize attack vectors by purging the session 

            HttpResponse::Ok().body( content )
            },
            Ok( None )=> {
            //println!("Ok( None ) No errors present in session");
            println!("Ok( None ) No active session");
            HttpResponse::Ok().body( wcf.get_tile(WebContentItem::WCTypeLoginTile) )
            },
            Err(_)=> {
            //println!("Ok( None ) No errors present in session");
            println!("User session does not exist");
            HttpResponse::Ok().body( wcf.get_tile(WebContentItem::WCTypeLoginTile) )
            },
        }
    }

}