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

use crate::constants;
use crate::nle::controller::CommandController;
use crate::ui::data_forms::*;
use crate::session::{AppSession, UserSession};
use crate::nle::nle::*;

use crate::ui::nle_command_fmt::NLECommandFormatter;
use crate::dao::patient_dao::*; //common_dao::*, encounter_dao::*, intervention_dao::*,


pub struct NLERoute{}

impl NLERoute{

    /* Reference Command mapping
        admit patient, 1, Admit New Patient
        admit add new open patient, 1, Admit New Patient
        add patient, 1, Admit New Patient
        new open patient, 1, Admit New Patient
        create patient, 1, Admit New Patient
        add information, 2, Add to Patient Chart
        add medication, 2, Add Medication
        prescribe medication, 2, Add Medication
        move patient, 2, Transfer Patient
        update contact information, 2, Update Patient Information
        discharge patient, 3, Discharge Patient
    */

   /*   pub async fn natural_language_prompt_test2(&self, _app_session: web::Data<AppSession>, _user_session: Session, req: web::Form<NLPromptFormData>) -> impl Responder {
        println!("-> /nlprompt Requested;  natural_language_prompt_test2();  prompt: \"{}\"", req.prompt);

        HttpResponse::Ok().body( "SUCCESS" )
    }*/

       pub async fn natural_language_prompt(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<NLPromptFormData>) -> impl Responder {
        tracing::info!("-> /nlprompt Requested;  natural_language_prompt();  prompt: \"{}\"", req.prompt);

        let mut results_sbuf = String::with_capacity(500); // Single heap allocation
        let prompt = req.prompt.clone();

        let cur_session: Option<UserSession> = user_session.get(constants::USER_SESSION).unwrap();

        let userid = cur_session.unwrap().clone().get_userid_as_i64();
        //let patient_id = req.patient_id.clone();

        let base_model_dir = app_session.system_config.cargo_manifest_dir.clone();
        let data_dir = app_session.system_config.data_sub_dir.clone();
        let language_model_file = app_session.system_config.language_model_file.clone();
        let tokenizer_file = app_session.system_config.tokenizer_file.clone();
        let command_mapping_file = app_session.system_config.command_mapping_file.clone();
                
        let nle = NaturalLanguageEngine::new( &Path::new( &base_model_dir.clone() )
                                                                        .join(data_dir.clone())
                                                                        .join(language_model_file).to_string_lossy(),

                                                    &Path::new( &base_model_dir.clone()  )
                                                                        .join(data_dir.clone())
                                                                        .join(tokenizer_file).to_string_lossy()
        ).await;
        
        // load the command controller structure, to manage proper use of the Language Engine
        let mut cmd: CommandController = CommandController::new(&Path::new( &base_model_dir.clone() )
                                                            .join(data_dir.clone())
                                                            .join(command_mapping_file).to_string_lossy(), nle );

        let cur_session: UserSession = user_session.get(constants::USER_SESSION).unwrap().unwrap();
        let pdao = PatientDAO::new( app_session.get_db_connection() ).await;
        let referenced_patient = cmd.get_referenced_patient(pdao, userid, prompt.clone());

       // if cur_session.user_authorizations.has_permission(p_id){
       // }

        //let classifer_results: Vec< (String, f32)> = cmd.get_classifier_rankings( prompt.clone() ).await;
        //let classifer_results: Vec< (String, f32)> = cmd.get_classifier_rankings_filtered_for_permissions( prompt.clone(), cur_session.user_authorizations ).await;

        //results_sbuf.push_str("<H1>natural language prompt</H1>\n");
        //results_sbuf.push_str( &NLECommandFormatter::get_nle_options_content(classifer_results, prompt, cmd) );
        
        HttpResponse::Ok().body( results_sbuf )
    }


    

}