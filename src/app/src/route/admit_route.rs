//! # Patient Admit, Discharge and related routes
//!
//!  CSM500 Project (April - October 2026)
//!  Graham Parker (Student ID: 240120522)

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 
use actix_web::http::StatusCode;
use tracing;

use crate::constants;
use crate::dto::patient::*;
use crate::dao::{ patient_dao::*, common_dao::*}; 
use crate::ui::{data_forms::*, menu_fmt::*};
use crate::session::{AppSession, UserSession};
use crate::ui::common_fmt::CommonFormatter;

pub struct AdmitRoute{}

impl AdmitRoute{

    /// ### AdmitRoute::route_to_discharge_patient_save()
    ///    Route that will update the encounter to a discharged status
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * user_session (actix_session::Session): the user's session
    /// * req: web::Form<DischargeDataForm>: the user's request, encapsulated in a DischargeDataForm
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    pub async fn route_to_discharge_patient_save(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<DischargeDataForm>) -> impl Responder {
        tracing::debug!("-> Route Requested: /route_to_discharge_patient_save ");
        println!("-> Route Requested: /route_to_discharge_patient_save ");

        let req_clone0 = req.clone();
        let user_session_details: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
        let dao = PatientDAO::new( app_session.get_db_connection() ).await;

        // update_encounter_from_discharge_form always returns some sort of result wrapped in an Ok()
        let _ignore = dao.update_encounter_from_discharge_form(req_clone0, user_session_details.get_userid_as_i64()).await.unwrap();
        /*match enc_results {
            constants::INVALID_OTHER_ID => {
                tracing::debug!("..Patient not discharged"); // form_errors: e.message.unwrap().to_string(),
            },
            _ => {
                let p_id = req.patient_id.to_string();
                tracing::debug!("..Patient (id={p_id}) discharged, updated");
            }
        }*/

        // route back to home after discharge, as the patient record can not be read again.
        actix_web::web::Redirect::to("/home").using_status_code(StatusCode::SEE_OTHER)
    }

    /// ### AdmitRoute::route_to_admit_save()
    ///    Route that will save patient data, from an Admit form submission
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * user_session (actix_session::Session): the user's session
    /// * req: web::Form<DischargeDataForm>: the user's request, encapsulated in a AdmitDataForm
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    pub async fn route_to_admit_save(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<AdmitDataForm>) -> impl Responder {
            tracing::debug!("-> Route Requested: /route_to_admit_SAVE ");
            println!("-> Route Requested: /route_to_admit_SAVE ");
            //println!("..patient_id:{}", req.patient_id.clone());

            let req_clone0 = req.clone();
            let user_session_details: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info

            // perform server-side form validation. If not successful, send the user back with some form errors
            let frm_errors = req.validate_fields();
            match frm_errors {
                Err(e) => {
                    tracing::error!("!AdmitDataForm > Form errors detected");
                    println!("!AdmitDataForm > Form errors detected. Redirect out: {}", e);

                    let req_clone = req.clone(); // local clone to avoid borrowing issues
        
                    Self::route_to_admit_discharge(app_session, user_session, web::Form(
                        AdmitDataForm {
                            patient_id: constants::INVALID_PATIENT_ID.to_string(), // no patient_id in this variation
                            encounter_id: req_clone.encounter_id,
                            patient_first_name: req_clone.patient_first_name,
                            patient_last_name: req_clone.patient_last_name,
                            patient_middle_name: req_clone.patient_middle_name,
                            phn: req_clone.phn,
                            birthdate: req_clone.birthdate,
                            location_id: req_clone.location_id,
                            admit_notes: req_clone.admit_notes,
                            action_flag: req_clone.action_flag,
                            form_errors: e.message.unwrap().to_string(),
                            ..Default::default() 
                        }
                    )).await
                },
                _ => {
                    tracing::debug!("..Form validation successful, Saving Patient");
                    println!("..Form validation successful, Saving Patient");

                    let dao = PatientDAO::new( app_session.get_db_connection() ).await;
                    let p_id = dao.upsert_patient_from_admit_form(req_clone0, user_session_details.clone().get_userid_as_i64()).await.unwrap();

                    if p_id != constants::INVALID_OTHER_ID {
                        tracing::debug!("  >(Step 1/2): Patient saved successfully, patient (id={p_id}) added/updated");
                        println!("  >(Step 1/2): Patient saved successfully, patient (id={p_id}) added/updated");
                        let mut req_clone2 = req.clone();
                        req_clone2.patient_id = p_id.to_string();

                        // if patient was successful, we need the Encounter as well
                        let e_id = dao.upsert_encounter_from_admit_form(req_clone2, user_session_details.clone().get_userid_as_i64()).await.unwrap();
                        if e_id != constants::INVALID_OTHER_ID {
                            tracing::debug!("  >(Step 2/2): Encounter saved successfully, patient (id={p_id}) and encounter (id={e_id}) added/updated");
                            println!("  >(Step 2/2): Encounter saved successfully, patient (id={p_id}) and encounter (id={e_id}) added/updated");

                            tracing::debug!("<--- Redirect back to : /route_to_admit_discharge (001)");
                            println!("  <--- Redirect back to : /route_to_admit_discharge (001)");
                            let req_clone = req.clone(); // local clone to avoid borrowing issues

                            // should redirect to the patient details
                            Self::route_to_admit_discharge(app_session, user_session, web::Form(
                                AdmitDataForm {
                                    patient_id: p_id.to_string(),
                                    encounter_id: e_id.to_string(),
                                    patient_first_name: req_clone.patient_first_name,
                                    patient_last_name: req_clone.patient_last_name,
                                    patient_middle_name: req_clone.patient_middle_name,
                                    phn: req_clone.phn,
                                    birthdate: req_clone.birthdate,
                                    location_id: req_clone.location_id,
                                    admit_notes: req_clone.admit_notes,
                                    action_flag: req_clone.action_flag,
                                    ..Default::default() // no form errors in this variation
                                }
                            )).await
                        }
                        else {
                            tracing::debug!("  >(Step 1/2): FAILED Admit form did not save");
                            tracing::debug!("<--- Redirect back to : /route_to_admit_discharge (003)");
                            println!("  >(Step 1/2): FAILED Admit form did not save");
                            println!("<--- Redirect back to : /route_to_admit_discharge (003)");

                            let req_clone = req.clone(); // local clone to avoid borrowing issues

                            Self::route_to_admit_discharge(app_session, user_session, web::Form(
                                AdmitDataForm {
                                    patient_id: constants::INVALID_PATIENT_ID.to_string(), // no patient_id in this variation
                                    encounter_id: req_clone.encounter_id,
                                    patient_first_name: req_clone.patient_first_name,
                                    patient_last_name: req_clone.patient_last_name,
                                    patient_middle_name: req_clone.patient_middle_name,
                                    phn: req_clone.phn,
                                    birthdate: req_clone.birthdate,
                                    location_id: req_clone.location_id,
                                    admit_notes: req_clone.admit_notes,
                                    action_flag: req_clone.action_flag,
                                    form_errors: "An error occurred, please try again".to_string(),
                                    ..Default::default() 
                                }
                            )).await
                        }
                    }
                    else {
                        tracing::debug!("  >(Step 1/2): FAILED Admit form did not save");
                        tracing::debug!("<--- Redirect back to : /route_to_admit_discharge (003)");
                        println!("  >(Step 1/2): FAILED Admit form did not save");
                        println!("<--- Redirect back to : /route_to_admit_discharge (003)");

                        let req_clone = req.clone(); // local clone to avoid borrowing issues

                        Self::route_to_admit_discharge(app_session, user_session, web::Form(
                            AdmitDataForm {
                                patient_id: constants::INVALID_PATIENT_ID.to_string(), // no patient_id in this variation
                                encounter_id: req_clone.encounter_id,
                                patient_first_name: req_clone.patient_first_name,
                                patient_last_name: req_clone.patient_last_name,
                                patient_middle_name: req_clone.patient_middle_name,
                                phn: req_clone.phn,
                                birthdate: req_clone.birthdate,
                                location_id: req_clone.location_id,
                                admit_notes: req_clone.admit_notes,
                                action_flag: req_clone.action_flag,
                                form_errors: "An error occurred, please try again".to_string(),
                                ..Default::default() 
                            }
                        )).await
                    }
                }, // "do nothing, because form was valid"
            }
    }

    /// ### AdmitRoute::route_to_admit_new_no_patient()
    ///    Wrapper route for the menu option to admit a patient without having any web form to pass data in from
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * user_session (actix_session::Session): the user's session
    /// * req: web::Form<DischargeDataForm>: the user's request, encapsulated in a AdmitFormBasic
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    pub async fn route_to_admit_new_no_patient(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<AdmitFormBasic>) -> impl Responder {
        tracing::debug!("-> Route Requested: /route_to_admit_new_no_patient");
        //tracing::debug!("..user_prompt:{}", req.user_prompt.clone());
        println!("-> Route Requested: /route_to_admit_new_no_patient");
        //println!("..user_prompt:{}", req.user_prompt.clone());
        //println!("..patient_id:{}", req.patient_id.clone());


            AdmitRoute::route_to_admit_discharge(app_session, user_session, web::Form(
                AdmitDataForm {
                    patient_id: req.patient_id.clone(),
                    action_flag: "admit".to_string(),
                    user_prompt: req.user_prompt.clone(),
                    admit_notes: req.user_prompt.clone(),
                    ..Default::default()
                }
            )).await
    }

    /// ### AdmitRoute::route_to_discharge_patient()
    ///    Wrapper route for the menu option to discharge a patient without having any web form to pass data in from
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * user_session (actix_session::Session): the user's session
    /// * req: web::Form<DischargeDataForm>: the user's request, encapsulated in a AdmitFormBasic
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    pub async fn route_to_discharge_patient(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<AdmitFormBasic>) -> impl Responder {
            tracing::debug!("-> Route Requested: /route_to_discharge_patient");
            //tracing::debug!("..user_prompt:{}", req.user_prompt.clone());
            println!("-> Route Requested: /route_to_discharge_patient");
            //println!("..user_prompt:{}", req.user_prompt.clone());

            AdmitRoute::route_to_admit_discharge(app_session, user_session, web::Form(
                AdmitDataForm {
                    patient_id: req.patient_id.clone(),
                    action_flag: "discharge".to_string(),
                    user_prompt: req.user_prompt.clone(),
                    ..Default::default()
                }
            )).await
    }

    /// ### AdmitRoute::route_to_admit_discharge()
    ///    Wrapper route for the menu option to discharge a patient without having any web form to pass data in from
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * user_session (actix_session::Session): the user's session
    /// * req: web::Form<DischargeDataForm>: the user's request, encapsulated in a AdmitDataForm
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    pub async fn route_to_admit_discharge(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<AdmitDataForm>) -> impl Responder {
        tracing::debug!("-> Route Requested: /admit_discharge  route_to_admit_discharge()");
        println!("-> Route Requested: /admit_discharge  route_to_admit_discharge()");

        let user_session_details: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
        let userid = user_session_details.clone().get_userid_as_i64();
        let user_display_name = user_session_details.clone().user_display_name;

        let wcf = &app_session.get_web_content_factory();

        let patient_id: i64 = req.patient_id.parse().unwrap(); // get the patient id from the form that was passed in; includes for server-side validation errors
        let dao = PatientDAO::new( app_session.get_db_connection() ).await;

        let discharge: bool = req.action_flag.eq("discharge");

        let existing_patient: Option<Patient>
            = if patient_id == constants::NOT_SPECIFIED_ID {
                tracing::debug!("..No Patient specified: create a new Patient and Encounter");
                println!("..No Patient specified: create a new Patient and Encounter");

                Some( Patient::to_patient(req.clone()) )
            }
            else{
                tracing::debug!("..Patient exists: view existing Patient and Encounter");
                println!("..Patient exists: view existing Patient and Encounter");
                let tmp_patient = dao.get_patient_details_not_discharged( userid.clone(), patient_id).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
                
                match tmp_patient {
                    Some(mut p) => {
                        if req.clone().user_prompt.len() > 0 {
                            if discharge {
                                p.discharge_notes = p.discharge_notes + &req.clone().user_prompt; // if discharging and arriving via prompt prompt will be added to the discharge notes
                            }
                            else {
                                p.admit_notes = p.admit_notes + &req.clone().user_prompt; // if admitting and arriving via prompt, prompt will be added to the admit notes
                            }
                        }                    
                        Some(p)
                    }, 
                    None => None,
                }
            };

        // construct the legacy menu based on the user's patients and site
        let legacy_menu_results = dao.get_patients_at_users_site_no_discharge(userid.clone()).await.expect( constants::DATABASE_ERROR_NOT_FOUND ).unwrap();
        let mut legacy_menu = constants::LEGACY_MENU_ON_ERROR.to_string();
        if legacy_menu_results.len() > 0 {
            legacy_menu = {MenuFormatter{}}.get_legacy_menu_with_patient(legacy_menu_results.clone(), patient_id, user_session_details.clone());
        }

        let location_results = {CommonDAO::new( app_session.get_db_connection() ).await}.get_locations_for_user(userid.clone())
                                                                                                            .await
                                                                                                            .expect( constants::DATABASE_ERROR_NOT_FOUND )
                                                                                                            .unwrap();
        let mut location_menu = constants::LEGACY_MENU_ON_ERROR.to_string();
        if location_results.len() > 0 {
            location_menu = CommonFormatter::get_location_dropdown(location_results.clone(), constants::DEFAULT_LOCATION_REGISTRATION)
        }

        // construct the tile based on session, patient data and the legacy menu
        let content = wcf.get_admit_discharge_full_tile(user_display_name.clone(),
                                            existing_patient,
                                            legacy_menu,
                                            location_menu,
                                            discharge,
                                            req.clone().user_prompt); // retrieve the page base content

        HttpResponse::Ok().body( content )
    }
}