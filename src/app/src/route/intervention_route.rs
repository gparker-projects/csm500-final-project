//! -------------------------------------------------------------------
//! Intervention, Intervention Details and related routes
//!
//!  CSM500 Project (April - October 2026)
//!  Graham Parker (Student ID: 240120522)
//! -------------------------------------------------------------------

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 
use tracing;

use crate::constants;
use crate::dto::intervention::Intervention;

use crate::dto::feature_preference::FeaturePreference;
use crate::dao::feature_preference_dao::FeaturePreferenceDAO;
use crate::dao::{patient_dao::*, intervention_dao::*, common_dao::*, auth_dao::*, encounter_dao::*}; 
use crate::session::{AppSession, UserSession};
use crate::ui::{data_forms::*, menu_fmt::*};
use crate::ui::common_fmt::CommonFormatter;
use crate::ui::feat_preference_fmt::FeaturePreferenceFormatter;

pub struct InterventionRoute{}

impl InterventionRoute{

    /// ### InterventionRoute::route_to_intervention_save()
    ///    Route that will update the intervention and then redirect back to the modify screen
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * user_session (actix_session::Session): the user's session
    /// * req: web::Form<DischargeDataForm>: the user's request, encapsulated in a InterventionDataForm
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    pub async fn route_to_intervention_save(app_session: web::Data<AppSession>, user_session: Session, mut req: web::Form<InterventionDataForm>) -> impl Responder {
        tracing::debug!("-> Route Requested: /route_to_discharge_patient_save ");
        println!("-> Route Requested: /route_to_discharge_patient_save ");

        let req_clone0 = req.clone();
        let user_session_details: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
        
        // perform server-side form validation. If not successful, send the user back with some form errors
        let frm_errors = req.validate_fields();
        match frm_errors {
            Ok( _ignore ) => { // "do nothing, because form was valid"
                tracing::debug!("..Form validation successful (InterventionDataForm), saving");
                println!("..Form validation successful (InterventionDataForm), saving");

                let idao = InterventionDAO::new( app_session.get_db_connection() ).await;
                let intv_id = idao.upsert_intervention_from_intv_form(req_clone0.clone(), user_session_details.get_userid_as_i64()).await.unwrap();
                if intv_id != constants::INVALID_OTHER_ID {
                    tracing::debug!("  >Intervention (id={intv_id})] created/updated");
                    req.0.intervention_id = intv_id.clone().to_string();

                    let type_id: i64 = req_clone0.clone().get_intervention_type_as_i64();
                    // if save successful, record a feature preference as well
                    let fpdao = FeaturePreferenceDAO::new( app_session.get_db_connection() ).await;
                    let _ignore = fpdao.upsert_feature_preference( user_session_details.clone().get_userid_as_i64(), 
                                                                                type_id).await.unwrap();
                }
                else {
                    req.form_errors = "Error occurred: Intervention not created/updated".to_string();
                    //tracing::debug!("....Error occurred: Intervention not created/updated");
                    //println!("....Error occurred: Intervention not created/updated");
                }
            },
            Err(e) => {
                tracing::error!("..!InterventionDataForm > Form errors detected, redirect to view/modify intervention");
                println!("..!InterventionDataForm > Form errors detected, redirect to view/modify intervention");
                req.form_errors = e.message.unwrap().to_string();
            },
        }
        // route back to main form again
        InterventionRoute::route_to_view_or_modify_intervention( app_session, user_session, req ).await
    }

    /// ### InterventionRoute::route_to_add_new_intervention()
    ///    Wrapper route for the adding new, or modifying existing Interventions of a patient, without having any web form to pass data in from
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * user_session (actix_session::Session): the user's session
    /// * req: web::Form<DischargeDataForm>: the user's request, encapsulated in a InterventionDataFormBasic
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    pub async fn route_to_add_new_intervention(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<InterventionDataFormBasic>)  -> impl Responder {
        tracing::debug!("-> Route Requested: /intvnew  route_to_add_new_intervention()");
        println!("-> Route Requested: /intvnew  route_to_add_new_intervention()");
        let mut tmp_encounter_id = req.encounter_id.clone();
        let tmp_patient_id = req.patient_id.clone();

        // if there was no encounter provided, we can find the current one, based on the userid
        if tmp_encounter_id == constants::INVALID_OTHER_ID.to_string() {
            let edao = EncounterDAO::new( app_session.get_db_connection() ).await;
            let enc = edao.get_current_encounter( req.get_patient_id_as_i64() ).await;
            
            tracing::debug!("..encounter_id not provided. Found: id={}", tmp_encounter_id);
            tmp_encounter_id = enc.id.to_string();
        }
        else{
            tmp_encounter_id = req.encounter_id.clone();
        }

        InterventionRoute::route_to_view_or_modify_intervention(app_session, user_session,web::Form(
            InterventionDataForm {
                intervention_id: constants::INVALID_OTHER_ID.to_string(),
                intervention_type_id: req.0.intervention_type_id.clone(),
                encounter_id: tmp_encounter_id,
                patient_id: tmp_patient_id,
                ..Default::default()
            }
        )).await
    }

    /// ### InterventionRoute::route_to_modify_intervention_basic()
    ///    Wrapper route for hyperlink to view/modify an Intervention without having any web form to pass data in from
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * user_session (actix_session::Session): the user's session
    /// * req: web::Form<DischargeDataForm>: the user's request, encapsulated in a InterventionDataFormLink
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
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

    /// ### InterventionRoute::route_to_view_or_modify_intervention()
    ///    Route for adding a new Intervention for a Patient-Encounter
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * user_session (actix_session::Session): the user's session
    /// * req: web::Form<DischargeDataForm>: the user's request, encapsulated in a InterventionDataForm
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    pub async fn route_to_view_or_modify_intervention(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<InterventionDataForm>)  -> impl Responder {
        tracing::debug!("-> Route Requested: /intv  (add/modify) route_to_view_or_modify_intervention()");
        println!("-> Route Requested: /intv  (add/modify) route_to_view_or_modify_intervention()");
        let user_session_details: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
        let wcf = &app_session.get_web_content_factory();
        let userid = user_session_details.clone().get_userid_as_i64();

        let intervention_type_id: i64;
        let intervention_id: i64 = req.intervention_id.parse().unwrap(); // get the intervention id from the form that was passed in; includes for server-side validation errors
        let idao = InterventionDAO::new( app_session.get_db_connection() ).await;

        let cur_intv: Option<Intervention>
          = if intervention_id == constants::NOT_SPECIFIED_ID {
            tracing::debug!("   No Intervention specified: create a new Intervention");
            println!("   Intervention.intervention_type_id {}", req.clone().intervention_type_id.to_string());
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
        let mut legacy_menu = constants::LEGACY_MENU_ON_ERROR.to_string();

        let patients_for_menu_lst = pdao.get_patients_at_users_site_no_discharge(userid).await.expect( constants::DATABASE_ERROR_NOT_FOUND ).unwrap();
        if patients_for_menu_lst.len() > 0 {
             legacy_menu = {MenuFormatter{}}.get_legacy_menu_with_patient(patients_for_menu_lst.clone(), req.get_patient_id_as_i64(), user_session_details.clone());
        }

        let user_dropdown_list = {AuthDAO::new( app_session.get_db_connection() ).await}.get_user_and_departments_at_current_user_sites(userid ).await.unwrap();

        let cdao = CommonDAO::new( app_session.get_db_connection() ).await;
        
        let intv_type=  cdao.get_intervention_type( intervention_type_id ).await.unwrap();

        // obtain the dropdown of measures
        //println!("..get measures (common ref type) for group_id={}", intervention_type_id);

        let intv_details_type=  cdao.get_common_references_by_id(intervention_type_id, true).await.unwrap();
        let status_dropdown_list=  cdao.get_intervention_statuses().await.unwrap();

        let mut location_results = cdao.get_locations_for_user(userid).await.expect( constants::DATABASE_ERROR_NOT_FOUND ).unwrap();
        if location_results.len() == 0 {             
            location_results = vec![(constants::DEFAULT_LOCATION_REGISTRATION, "Registration".to_string())];
            tracing::debug!("No locations found for user");
        }
        let location_menu = CommonFormatter::get_location_dropdown(location_results.clone(), 
            match intervention_id == constants::NOT_SPECIFIED_ID {
                true => constants::DEFAULT_LOCATION_REGISTRATION,
                false => cur_intv.unwrap().location_id,
            }
        );

        let fast_actions_upper_limit = app_session.clone().system_config.get_max_general_fastactions();
        let max_age_feature_preferences = app_session.clone().system_config.get_max_age_feature_preferences();
        let pref_list: Option<Vec<FeaturePreference>> = {FeaturePreferenceDAO::new( app_session.get_db_connection() ).await}
                                                                                .get_active_feature_preferences_of_intervention_details_for_user(userid,
                                                                                                                                                        intervention_type_id,
                                                                                                                                                        max_age_feature_preferences,
                                                                                                                                                        fast_actions_upper_limit).await.unwrap();
        let content = wcf.get_modify_intervention_full_tile(user_session_details.user_display_name,
                                                                         cur_intv2,
                                                                         legacy_menu,
                                                                         user_dropdown_list.unwrap(),
                                                                         status_dropdown_list.unwrap(),
                                                                         location_menu,
                                                                         intv_type.unwrap(),
                                                                         req.clone(),
                                                                         intv_dtls.clone(),
                                                                         intv_details_type.unwrap(),
                                                                         FeaturePreferenceFormatter::get_feature_preference_section( pref_list ),
                                                                         );
        HttpResponse::Ok().body(  content )
    }
}