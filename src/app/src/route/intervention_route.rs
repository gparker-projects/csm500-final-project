//! # Intervention, Intervention Details and related routes
//!
//!      CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 

use crate::dto::encounter::Encounter;

use crate::dao::{ patient_dao::*, intervention_dao::*, encounter_dao::*}; 
use crate::webc::{web_content::*, data_forms::*, menu_tile::*};

use crate::session;

use crate::constants;

pub struct InterventionRoute{}

impl InterventionRoute{

    ///
    /// Route for adding new, or modifying existing Interventions of a patient
    /// 
    pub async fn route_to_modify_intervention(app_session: web::Data<session::AppSession>, user_session: Session) -> impl Responder {
        println!("-> /modify_intervention Route Requested");

        let user_session: session::UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
        let wcf = &app_session.get_web_content_factory(); 
        //let mut content = wcf.get_modify_intervention_tile(); // retrieve the page base content

        //pub fn get_modify_intervention_tile(&self, current_intervention: Intervention) -> String {



        HttpResponse::Ok().body( "CONTENT TODO: route_to_modify_intervention()" )  //content )
    }
}