//! Intervention, Intervention Details and related routes
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 
use tracing;

use crate::dto::intervention::Intervention;

use crate::dao::{ patient_dao::*, intervention_dao::*, common_dao::*, auth_dao::*}; 
use crate::ui::{data_forms::*, menu_fmt::*};
use crate::ui::common::CommonFormatter;

use crate::session::{AppSession, UserSession};

use crate::constants;

pub struct InterventionRoute{}

impl InterventionRoute{


  ///
  /// Route that will update the intervention and then redirect back to the modify screen
  /// 
  pub async fn route_to_intervention_save(app_session: web::Data<AppSession>, user_session: Session, mut req: web::Form<InterventionDataForm>) -> impl Responder {
      tracing::debug!("-> Route Requested: /route_to_discharge_patient_save ");

      let req_clone0 = req.clone();
      let user_session_details: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
      let idao = InterventionDAO::new( app_session.get_db_connection() ).await;

      let results = idao.upsert_intervention_from_intv_form(req_clone0, user_session_details.get_userid_as_i64()).await;
      match results {
            Ok(intv_id) => {
                tracing::debug!("  >Intervention (id={intv_id})] created/updated");
                req.0.intervention_id = intv_id.to_string();
            },
            Err(e) => {
                tracing::debug!("  >Intervention not created/updated: {e}");
            }
      }

      // route back to main form again
      InterventionRoute::route_to_view_or_modify_intervention( app_session, user_session, req ).await
  }

    ///
    /// Wrapper route for the adding new, or modifying existing Interventions of a patient, without having any web form to pass data in from
    /// 
    pub async fn route_to_add_new_intervention(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<InterventionDataFormBasic>)  -> impl Responder {
        tracing::debug!("-> Route Requested: /intvnew  route_to_add_new_intervention()");

        InterventionRoute::route_to_view_or_modify_intervention(app_session, user_session,web::Form(
            InterventionDataForm {
                intervention_id: constants::INVALID_OTHER_ID.to_string(),
                intervention_type_id: req.0.intervention_type_id.clone(),
                encounter_id: req.0.encounter_id.clone(),
                patient_id: req.0.patient_id.clone(),
                ..Default::default()
            }
        )).await
    }

    ///
    /// Wrapper route for hyperlink to view/modify an Intervention without having any web form to pass data in from
    /// 
    pub async fn route_to_modify_intervention_basic(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<InterventionDataFormLink>) -> impl Responder {
        tracing::debug!("-> Route Requested: /intvlink  route_to_modify_intervention_basic()");

            InterventionRoute::route_to_view_or_modify_intervention(app_session, user_session, web::Form(
                InterventionDataForm {
                    intervention_id: req.intervention_id.clone(),
                    encounter_id: req.encounter_id.clone(),
                    patient_id: req.patient_id.clone(),
                    ..Default::default()
                }
            )).await
    }

    ///
    /// Route for adding a new Intervention for a Patient-Encounter
    /// 
    pub async fn route_to_view_or_modify_intervention(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<InterventionDataForm>)  -> impl Responder {
        tracing::debug!("-> Route Requested: /intv  (add/modify)");
        let user_session_details: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
        let wcf = &app_session.get_web_content_factory();

        let intervention_type_id: i64;
        let intervention_id: i64 = req.intervention_id.parse().unwrap(); // get the intervention id from the form that was passed in; includes for server-side validation errors
        let idao = InterventionDAO::new( app_session.get_db_connection() ).await;

        let cur_intv: Option<Intervention>
          = if intervention_id == constants::NOT_SPECIFIED_ID {
            tracing::debug!("   No Intervention specified: create a new Intervention");
            //println!("   Intervention.intervention_type_id {}", req.clone().intervention_type_id.to_string());
            intervention_type_id = req.clone().intervention_type_id.parse().unwrap(); 
            None
        }
        else{
            tracing::debug!("   Intervention exists: view existing Intervention");
            let tmp_intv = idao.get_intervention(intervention_id).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
            let tmp_intv2 = tmp_intv.clone().unwrap();
            intervention_type_id = tmp_intv2.intervention_type_id;

            tmp_intv
        };
        let cur_intv2= cur_intv.clone(); // clone of above object to avoid move below

        let intv_dtls = idao.get_all_intervention_details_for_an_intervention(intervention_id, constants::NOT_SPECIFIED_ID).await.unwrap();

        // refresh the patients in the menu (only)
        let pdao = PatientDAO::new( app_session.get_db_connection() ).await;
        let legacy_menu_results = pdao.get_patients_at_users_site_no_discharge(user_session_details.get_userid_as_i64(), false).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
        let legacy_menu = match legacy_menu_results {
            Some (patients_for_menu_lst) => {
                {MenuFormatter{}}.get_legacy_menu_with_patient(patients_for_menu_lst.clone(), req.get_patient_id_as_i64())
            }
            None => {
                tracing::debug!("No patients found for legacy menu");
                constants::LEGACY_MENU_ON_ERROR.to_string()
            }
        };

        let user_dropdown_list = {AuthDAO::new( app_session.get_db_connection() ).await}.get_user_and_departments_at_current_user_sites(user_session_details.get_userid_as_i64() ).await.unwrap();

        let cdao = CommonDAO::new( app_session.get_db_connection() ).await;
        
        let intv_type=  cdao.get_intervention_type( intervention_type_id ).await.unwrap();

        let status_dropdown_list=  cdao.get_intervention_statuses().await.unwrap();
        let location_results = cdao.get_locations_for_user(user_session_details.get_userid_as_i64()).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
        let location_menu = match location_results {
          Some (loc_list) => {
              
              CommonFormatter::get_location_dropdown(loc_list.clone(), 
                    match intervention_id == constants::NOT_SPECIFIED_ID {
                      true => constants::DEFAULT_LOCATION_REGISTRATION,
                      false => cur_intv.unwrap().location_id,
                    }
              )
          }
          None => {
              tracing::debug!("No locations found for user. [Userid:{}]", user_session_details.get_userid_as_i64());
              constants::LEGACY_MENU_ON_ERROR.to_string() // when no patient, return default error-expected menu
          }
        };

        let content = wcf.get_modify_intervention_full_tile(user_session_details.user_display_name,
                                                                         cur_intv2,
                                                                         legacy_menu,
                                                                         user_dropdown_list.unwrap(),
                                                                         status_dropdown_list.unwrap(),
                                                                         location_menu,
                                                                         intv_type.unwrap(),
                                                                         req.patient_id.clone(),
                                                                         req.intervention_type_id.clone() ,
                                                                         req.encounter_id.clone(),
                                                                         intv_dtls.clone() );

        HttpResponse::Ok().body(  content )
    }
}