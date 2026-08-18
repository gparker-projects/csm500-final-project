//! # admit and related routes
//!
//!      CSM500 Project (April - October 2026)
//!         Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

//!
use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 

use crate::dto::patient;

use crate::dao::{ patient_dao::*, common_dao::*}; 
use crate::webc::{data_forms::*, menu_tile::*};
use crate::session::{AppSession, UserSession};

use crate::constants;

pub struct AdmitRoute{}

impl AdmitRoute{
  ///
  /// Route that will save patient data, from an Admit form submission
  /// 
  pub async fn route_to_admit_save(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<AdmitDataForm>) -> impl Responder {
      println!("-> Route Requested: /route_to_admit_SAVE ");

      let req_clone0 = req.clone();
      let user_session_details: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
      let dao = PatientDAO::new(constants::DB_CONN_STR).await;

      let results = dao.upsert_patient_from_admit_form(req_clone0, user_session_details.get_userid_as_i64()).await;
      match results {
          Ok(p_id) => {
            println!("  >(Step 1/2): Patient saved successfully, patient (id={p_id}) added/updated");
            let mut req_clone2 = req.clone();
            req_clone2.patient_id = p_id.to_string();

            // if patient was successful, we need the Encounter as well
            let enc_results = dao.upsert_encounter_from_admit_form(req_clone2, user_session_details.get_userid_as_i64()).await;
            match enc_results {
                Ok(e_id) => {
                  println!("  >(Step 2/2): Encounter saved successfully, patient (id={p_id}) and encounter (id={e_id}) added/updated");

                  println!("<--- Redirect back to : /route_to_admit_discharge (001)");
                  let req_clone = req.clone(); // local clone to avoid borrowing issues

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
                              ..Default::default() // no form errors in this variation
                          }
                  )).await
                },
                Err(e) => {
                    println!("  >(Step 2/2): FAILED - Admit form did not save: {e}");
                    println!("<--- Redirect back to : /route_to_admit_discharge (002)");
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
                                form_errors: "An error occurred, please try again".to_string()
                              // ..Default::default() 
                            }
                    )).await
                }
            }
          },
          Err(e) => {
            println!("  >(Step 1/2): FAILED Admit form did not save: {e}");
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
                        form_errors: "An error occurred, please try again".to_string()
                      // ..Default::default() 
                    }
            )).await
          }
      }
  }

  ///
  /// Route for New patient admit, or existing patient discharge page
  /// 
  pub async fn route_to_admit_discharge(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<AdmitDataForm>) -> impl Responder {
      println!("-> Route Requested: /admit_discharge");

      let user_session_details: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
      let wcf = &app_session.get_web_content_factory();

      let patient_id: i64 = req.patient_id.parse().unwrap(); // get the patient id from the form that was passed in; includes for server-side validation errors
      let dao = PatientDAO::new(constants::DB_CONN_STR).await;

      println!(">> route_to_admit_discharge() called");

      let existing_patient: Option<patient::Patient>
        = if patient_id == constants::NOT_SPECIFIED_ID {
              println!("   No Patient specified: create a new Patient and Encounter");
              None
          }
          else{
              println!("   Patient exists: view existing Patient and Encounter");
              dao.get_patient_details( user_session_details.get_userid_as_i64(), patient_id).await.expect( constants::DATABASE_ERROR_NOT_FOUND ) 
          };


      // construct the legacy menu based on the user's patients and site
      let legacy_menu_results = dao.get_patients_at_users_site_no_discharge(user_session_details.get_userid_as_i64(), false).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
      let legacy_menu = match legacy_menu_results {
          Some (patients_for_menu_lst) => {
            {MenuFormatter{}}.get_legacy_menu_with_patient(patients_for_menu_lst.clone(), patient_id)
          }
          None => {
              println!("No patients found for legacy menu. [Userid:{}]", user_session_details.get_userid_as_i64());
              constants::LEGACY_MENU_ON_ERROR.to_string() // when no patient, return default error-expected menu
          }
      };

      let cdao = CommonDAO::new(constants::DB_CONN_STR).await;
      let location_results = cdao.get_locations_for_user(user_session_details.get_userid_as_i64()).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
      let location_menu = match location_results {
          Some (loc_list) => {
              wcf.get_location_dropdown(loc_list.clone(), constants::DEFAULT_LOCATION_REGISTRATION)
          }
          None => {
              println!("No locations found for user. [Userid:{}]", user_session_details.get_userid_as_i64());
              constants::LEGACY_MENU_ON_ERROR.to_string() // when no patient, return default error-expected menu
          }
      };

      // construct the tile based on session, patient data and the legacy menu
      let content = wcf.get_admit_discharge_tile(user_session_details.user_display_name, existing_patient, legacy_menu, location_menu); // retrieve the page base content

      HttpResponse::Ok().body( content ) //"TODO : route_to_admit_discharge()" ) 
  }

}