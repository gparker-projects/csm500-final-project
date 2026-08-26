//! # Home and related routes
//!
//!      CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 

use crate::dto::encounter::Encounter;
use crate::dao::{patient_dao::*, intervention_dao::*, encounter_dao::*}; 
use crate::webc::menu_fmt::*;
use crate::session;
use crate::constants;

pub struct HomeRoute{}

impl HomeRoute{

    ///
    /// Main workspace page of the application, to be supplemented with lots of Javascript, CSS and API calls
    /// 
    pub async fn route_to_home(app_session: web::Data<session::AppSession>, user_session: Session) -> impl Responder {
        println!("-> /home Route Requested");

        let user_session: session::UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info

        let wcf = &app_session.get_web_content_factory(); // https://actix.rs/docs/application/
        let mut content = wcf.get_home_tile(); // retrieve the page base content

        // get patients at the user's facility, for display
        let dao = PatientDAO::new( app_session.get_db_connection() ).await;
        let idao = InterventionDAO::new( app_session.get_db_connection() ).await;
        let edao = EncounterDAO::new( app_session.get_db_connection() ).await;

        let qry_results = dao.get_patients_at_users_site_no_discharge(user_session.get_userid_as_i64(), false).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
        match qry_results {
            Some (patient_list) => {
            //println!("Retrieved {} patients:", patient_list.len());
            let mut pwrap: Vec<PatientWrapper> = Vec::new();

            for p in patient_list.clone(){
                let cur_enc: Encounter = edao.get_current_encounter(p.id.clone()).await.clone(); //get the current encounter for each patient
                let cur_intv = idao.get_most_recent_vitals(cur_enc.id.clone()).await.expect(constants::DATABASE_ERROR_NOT_FOUND);

                pwrap.push( PatientWrapper{
                        patient: p.clone(),
                        current_encounter: cur_enc.clone(),
                        most_recent_intervention: cur_intv
                    }
                );
                //print!(">> DEBUG Added pid={} e={} i={}", tmp_p, tmp_e, tmp_i);
            }
            
            let patient_list_html = wcf.get_home_route_summary_of_patients_tile_using_wrapper(pwrap.clone()); 
            content = content.replace(constants::BODY_TILE_CONTENT_TAG, &patient_list_html);  // replace default string

            let std_menu_html = {MenuFormatter{}}.get_legacy_menu(patient_list.clone()); 
            content = content.replace(constants::LEGACY_MENU_TILE_TAG, &std_menu_html);  // replace default string       
            }
            None => {
            println!("No patients found");
            }
        }
        // and adjust the menu

        // add the user's identity
        content = content.replace(constants::USER_IDENTITY_TILE_TAG, &&user_session.user_display_name); 

        //change to get_home_tile_with_user_identity(&&user_session.user_display_name);

        HttpResponse::Ok().body( content )
    }

}