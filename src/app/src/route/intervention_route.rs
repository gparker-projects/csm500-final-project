//! # Intervention, Intervention Details and related routes
//!
//!      CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 

use crate::dto::intervention::Intervention;

use crate::dao::{ patient_dao::*, intervention_dao::*, common_dao::*, auth_dao::*}; 
use crate::webc::{web_content::*, data_forms::*, menu_tile::*};
use crate::webc::data_forms::InterventionDataForm;

use crate::session;
use crate::constants;

pub struct InterventionRoute{}

impl InterventionRoute{
    ///
    /// Route for adding a new Intervention for a Patient-Encounter
    /// 
    pub async fn route_to_add_new_intervention(app_session: web::Data<session::AppSession>, user_session: Session, req: web::Form<InterventionDataForm>)  -> impl Responder {
        println!("-> /intvnew Route Requested");
        let user_session: session::UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
        let wcf = &app_session.get_web_content_factory(); 

        let intervention_id: i64 = req.intervention_id.parse().unwrap(); // get the intervention id from the form that was passed in; includes for server-side validation errors
        let idao = InterventionDAO::new(constants::DB_CONN_STR).await;
        

        let cur_intv: Option<Intervention>
          = if intervention_id == constants::NOT_SPECIFIED_ID {
              println!("   No Intervention specified: create a new Intervention");
              None
          }
          else{
              println!("   Intervention exists: view existing Intervention");
              idao.get_intervention(intervention_id).await.expect( constants::DATABASE_ERROR_NOT_FOUND ) 
         };

        // refresh the patients in the menu (only)
        let pdao = PatientDAO::new(constants::DB_CONN_STR).await;
        let legacy_menu_results = pdao.get_patients_at_users_site_no_discharge(user_session.get_userid_as_i64(), false).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
        let legacy_menu = match legacy_menu_results {
            Some (patients_for_menu_lst) => {
                {MenuFormatter{}}.get_legacy_menu_with_patient(patients_for_menu_lst.clone(), req.get_patient_id_as_i64())
            }
            None => {
                println!("No patients found for legacy menu");
                constants::LEGACY_MENU_ON_ERROR.to_string()
            }
        };

        let user_dropdown_list = {AuthDAO::new(constants::DB_CONN_STR).await}.get_user_and_departments_at_current_user_sites(user_session.get_userid_as_i64() ).await.unwrap();

        let status_dropdown_list=  {CommonDAO::new(constants::DB_CONN_STR).await}.get_intervention_statuses().await.unwrap();


        let content = wcf.get_modify_intervention_full_page_tile(user_session.user_display_name,
                                                                         cur_intv,
                                                                         legacy_menu,
                                                                         user_dropdown_list.unwrap(),
                                                                         status_dropdown_list.unwrap() );

        HttpResponse::Ok().body(  content )
    }

    ///
    /// Route for adding new, or modifying existing Interventions of a patient
    /// 
    pub async fn route_to_view_or_modify_intervention(app_session: web::Data<session::AppSession>, user_session: Session) -> impl Responder {
        println!("-> /intv Route Requested");

        let user_session: session::UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
        let wcf = &app_session.get_web_content_factory(); 
        //let mut content = wcf.get_modify_intervention_tile(); // retrieve the page base content

        //pub fn get_modify_intervention_tile(&self, current_intervention: Intervention) -> String {



        HttpResponse::Ok().body( "route_to_view_or_modify_intervention()" )  //content )
    }

}