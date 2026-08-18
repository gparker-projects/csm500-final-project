//! # patient and related routes
//!
//!      CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 

//use crate::dto::patient::Patient;
use crate::dto::encounter::Encounter;

use crate::dao::{ patient_dao::*, intervention_dao::*, encounter_dao::*}; 
use crate::webc::{web_content::*, data_forms::*, menu_tile::*};

use crate::session::{AppSession, UserSession};

use crate::constants;

pub struct PatientRoute{}

impl PatientRoute{
    ///
    /// Route to View Patient details; expects a GenerialWebFormData to have been submitted to reach the route
    ///
    pub async fn route_to_patient_details(user_session: Session, app_session: web::Data<AppSession>, req: web::Form<GenericWebFormData>) -> impl Responder {
    println!("-> /patientdtls Route Requested");

    //todo: this should direct to a standard error or login screen when session is lost
    let user_session: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
    let wcf: &WebContentFactory = &app_session.get_web_content_factory(); 

    let patient_id: i64 = req.get_uid_as_i64();

    // get base patient data
    let dao = PatientDAO::new(constants::DB_CONN_STR).await;
    let idao = InterventionDAO::new(constants::DB_CONN_STR).await;
    let edao = EncounterDAO::new(constants::DB_CONN_STR).await;

    //let cur_enc_section: String; // = "No Encounters found".to_owned();
    //let cur_encounter: Encounter; 
    //let cur_intv: Option<Intervention>; 

    // pull out the current Encounter and generate summary tile for it
    let cur_enc: Encounter = edao.get_current_encounter(patient_id).await.clone();
    let cur_enc_section = wcf.get_single_encounter_summary_tile(cur_enc.clone());

    // pull out the most recent vitals (Intervention of type = "Vitals") and generate summary tile for it
    let cur_intv = idao.get_most_recent_vitals(cur_enc.id.clone()).await.expect(constants::DATABASE_ERROR_NOT_FOUND); 

    // get encounters for the patient
    let enc_results = edao.get_encounters(patient_id, false).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
    let enc_section: String = match enc_results {
        Some (encounters) => {
            //println!("Patient details obtained");
            wcf.get_encounter_list_tile(encounters)
        }
        None =>{
            //println!("No Encounters found");
            "No Encounters found".to_owned()
        } 
    };

    // get all interventions for the patient
    let intv_results = idao.get_interventions(patient_id, false).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
    let intv_section = match intv_results {
        Some (intvs) => {
            //println!("Patient details obtained");
            wcf.get_intervention_list_tile(intvs)
        }
        None =>{
            //println!("No Encounters found");
            "No Interventions found".to_owned()
        } 
    };

    // get patient encounter history

    let patient_results = dao.get_patient_details( user_session.get_userid_as_i64(), patient_id).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
    let patient_header = match patient_results {
        Some (patient_details) => {
            //println!("Patient details obtained"); //: {}", &tile_content);

            let pwrap: PatientWrapper = PatientWrapper{
                patient: patient_details,
                current_encounter: cur_enc, 
                most_recent_intervention: cur_intv
            };

            wcf.get_single_patient_summary( pwrap, -1)
        }
        None =>{
            //println!("No patients found");
            "No patients found".to_owned()
        } 
    };

    // refresh the patients in the menu (only)
    let legacy_menu_results = dao.get_patients_at_users_site_no_discharge(user_session.get_userid_as_i64(), false).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
    let legacy_menu = match legacy_menu_results {
        Some (patients_for_menu_lst) => {
            {MenuFormatter{}}.get_legacy_menu_with_patient(patients_for_menu_lst.clone(), patient_id)
        }
        None => {
            println!("No patients found for legacy menu");
            constants::LEGACY_MENU_ON_ERROR.to_string()
        }
    };

    let consolidated_content = wcf.get_patient_details_full_tile(patient_header, cur_enc_section, enc_section,
                                                    user_session.user_display_name, legacy_menu, intv_section); //, enc_history);

    HttpResponse::Ok().body( consolidated_content )
    }
}