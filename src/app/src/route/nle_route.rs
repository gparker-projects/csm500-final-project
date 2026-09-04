//! Natural Language prompt and related routes
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 

use rand::rand_core::UnwrapErr;
//use std::collections::{HashSet};
use tracing;
use std::path::Path;
use std::process::Command;

use crate::constants;
use crate::nle::controller::CommandController;
use crate::ui::data_forms::*;
use crate::session::{AppSession, UserSession};
use crate::nle::nle::*;

use crate::ui::nle_command_fmt::NLECommandFormatter;
use crate::dao::patient_dao::*; //common_dao::*, encounter_dao::*, intervention_dao::*,
use crate::dto::patient::*;

pub struct NLERoute{}

impl NLERoute{
       pub async fn natural_language_prompt(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<NLPromptFormData>) -> impl Responder {
        tracing::info!("-> /nlprompt Requested;  natural_language_prompt();  prompt: \"{}\"", req.prompt);

        let mut results_sbuf = String::with_capacity(500); // Single heap allocation
        let prompt = req.prompt.clone();

        let cur_session: Option<UserSession> = user_session.get(constants::USER_SESSION).unwrap();

        let userid = cur_session.unwrap().clone().get_userid_as_i64();
                
        let nle = NaturalLanguageEngine::new( &app_session.get_full_path_language_model_file(),
                                                                 &app_session.get_full_path_tokenizer_file()
        ).await;
        
        // load the command controller structure, to manage proper use of the Language Engine
        let mut cmd: CommandController = CommandController::new(&app_session.get_full_path_command_mapping_file(), nle );

        let cur_session: UserSession = user_session.get(constants::USER_SESSION).unwrap().unwrap();
        let pdao = PatientDAO::new( app_session.get_db_connection() ).await;
        let referenced_patient = CommandController::get_referenced_patient(pdao, userid, prompt.clone()).await;


        let prompt_final = match referenced_patient.0 {
            CommandController::KNOWN_PATIENT_FOUND =>{
                let p: Patient = referenced_patient.clone().1.unwrap(); // pull out the patient's name and adjust the prompt prior to matching
                prompt.clone().replace(&p.legal_first_name, "patient").replace(&p.legal_last_name, "patient")
            },
            _ => prompt.clone(),
        };

        let classifer_results_final: Vec< (String, f32)> = cmd.get_classifier_rankings_filtered_for_permissions( prompt_final, cur_session.clone().user_authorizations ).await;
        let patient_id = match referenced_patient.clone().0 {
            CommandController::NO_PATIENT_FOUND => constants::INVALID_PATIENT_ID,
            _ => { // otherwise reduce the list of results.
                match referenced_patient.1 {
                    Some(rp) => rp.id,
                    None => constants::INVALID_PATIENT_ID,
                }
            },
        };

        tracing::debug!("...NLERoute evaluation: patient_id={} ", patient_id);
        tracing::debug!("...                     prompt={} ", prompt.clone());

        //results_sbuf.push_str("<H1>natural language prompt</H1>\n");
        results_sbuf.push_str( &NLECommandFormatter::get_nle_options_content(classifer_results_final, prompt, cmd, patient_id) );
        
        HttpResponse::Ok().body( results_sbuf )
    }



}