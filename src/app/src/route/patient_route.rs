//! Patient and related routes
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 
use tracing;

use crate::constants;
use crate::dao::feature_preference_dao::FeaturePreferenceDAO;
use crate::dto::encounter::Encounter;
use crate::dto::feature_preference::FeaturePreference;

use crate::ui::feat_preference_fmt::FeaturePreferenceFormatter;
use crate::dao::{common_dao::*, encounter_dao::*, intervention_dao::*, patient_dao::*};
use crate::ui::{data_forms::*, menu_fmt::*, simple_fmt::*};
use crate::session::{AppSession, UserSession};

pub struct PatientRoute{}

impl PatientRoute{

    /// ### PatientRoute::route_to_patient_details()
    ///   Route to View Patient details; expects a GenerialWebFormData to have been submitted to reach the route
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * req: web::Form<GenericWebFormData>: the request data to obtain the patient's data using a GenericWebFormData struct
    /// * user_session (actix_session::Session): the user's session
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    pub async fn route_to_patient_details(user_session: Session, app_session: web::Data<AppSession>, req: web::Form<GenericWebFormData>) -> impl Responder {
    tracing::debug!("-> /patientdtls Route Requested");
    println!("-> /patientdtls Route Requested");

    //todo: this should direct to a standard error or login screen when session is lost
    let user_session_details: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
    let userid = user_session_details.get_userid_as_i64();
    let patient_id: i64 = req.get_uid_as_i64();

    // get base patient data
    let pdao = PatientDAO::new( app_session.get_db_connection() ).await;
    let idao = InterventionDAO::new( app_session.get_db_connection() ).await;
    let edao = EncounterDAO::new( app_session.get_db_connection() ).await;

    // pull out the current Encounter and generate summary tile for it
    let cur_enc: Encounter = edao.get_current_encounter(patient_id).await.clone();
    let cur_enc_section = SimpleFormatter::get_single_encounter_summary_tile(cur_enc.clone());
    let cur_enc_id = cur_enc.clone().id.to_string(); // must be copied here before it moves below

    // pull out the most recent vitals (Intervention of type = "Vitals") and generate summary tile for it
    let cur_intv = idao.get_most_recent_vitals(cur_enc.id).await.expect(constants::DATABASE_ERROR_NOT_FOUND); 

    // get encounters for the patient
    let enc_results = edao.get_encounters(patient_id, false).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
    let enc_section: String = match enc_results {
        Some (encounters) => SimpleFormatter::get_encounter_list_tile(encounters),
        None => "No Encounters found".to_owned()
    };

    // get all interventions for the patient
    let intv_results = idao.get_interventions(cur_enc.id, false).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
    let intv_section = match intv_results {
        Some (intvs) =>  SimpleFormatter::get_intervention_list_for_patient_details_tile(intvs),
        None => "No Interventions found".to_owned(),
    };

    // get patient encounter history
    let patient_results = pdao.get_patient_details( userid, patient_id).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
    let patient_header = match patient_results {
        Some (patient_details) => {
            //println!("Patient details obtained"); //: {}", &tile_content);

            let pwrap: PatientWrapper = PatientWrapper{
                patient: patient_details,
                current_encounter: cur_enc, 
                most_recent_intervention: cur_intv
            };

            SimpleFormatter::get_single_patient_summary( pwrap, -1)
        }
        None => "No patients found".to_owned()
    };

    // refresh the patients in the menu (only)
    let legacy_menu_results = pdao.get_patients_at_users_site_no_discharge(userid).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
    let legacy_menu = match legacy_menu_results {
        Some (patients_for_menu_lst) => {
            {MenuFormatter{}}.get_legacy_menu_with_patient(patients_for_menu_lst.clone(), patient_id, user_session_details.clone())
        }
        None => {
            tracing::debug!("No patients found for legacy menu");
            constants::LEGACY_MENU_ON_ERROR.to_string()
        }
    };
    
    // no user should be able to get into the system without a location assigned, so we will not worry about an exception here
    let item_list = {CommonDAO::new( app_session.get_db_connection() ).await}.get_intervention_types().await.unwrap();
    // limit the list to intervention types that the user is allowed to use (clinical, non clinical or none)

    

    let fast_actions_upper_limit = app_session.clone().system_config.get_max_general_fastactions();
    let pref_list: Option<Vec<FeaturePreference>> = {FeaturePreferenceDAO::new( app_session.get_db_connection() ).await}.get_active_feature_preferences_of_interventions_for_user(userid, fast_actions_upper_limit).await.unwrap();

    let consolidated_content = app_session.get_web_content_factory().get_patient_details_full_tile(patient_header,
                                                                                                           cur_enc_section,
                                                                                                           enc_section,
                                                                                                           user_session_details,
                                                                                                           legacy_menu,
                                                                                                           intv_section,
                                                                                                           item_list.unwrap(), 
                                                                                                           FeaturePreferenceFormatter::get_feature_preference_section(pref_list),
                                                                                                           patient_id.to_string(),
                                                                                                           cur_enc_id);

    HttpResponse::Ok().body( consolidated_content )
    }
}