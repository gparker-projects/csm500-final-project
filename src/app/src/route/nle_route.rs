//! Natural Language prompt and related routes
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
use crate::nle::controller::CommandController;
use crate::ui::data_forms::*;
use crate::session::{AppSession, UserSession};
use crate::nle::nle::*;

use crate::ui::nle_command_fmt::NLECommandFormatter;
use crate::dao::patient_dao::*;
use crate::dto::patient::*;

pub struct NLERoute{}

impl NLERoute{

    /// ### NLERoute::natural_language_prompt()
    ///   Route to process and perform a natural language engine (NLE) query
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * req: web::Form<GenericWebFormData>: the request data to obtain the patient's data using a GenericWebFormData struct
    /// * user_session (actix_session::Session): the user's session
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    pub async fn natural_language_prompt(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<NLPromptFormData>) -> impl Responder {
        tracing::info!("-> /nlprompt Requested;  natural_language_prompt();  prompt: \"{}\"", req.prompt);
        println!("-> /nlprompt Requested; natural_language_prompt();  prompt: \"{}\"", req.prompt);

        let mut results_sbuf = String::with_capacity(500); // Single heap allocation
        let mut default_patient = constants::INVALID_PATIENT_ID; // default patient, may be provided by the caller either as part of context of prompt, or in prompt itself

        // strip and set aside the patient id, if provided
        let prompt_level_0: String = match req.prompt.clone().strip_prefix("{patient_id=-1") {
            Some(result) => result.to_string(), // default_patient remains as -1
            None => {                          
                if let Some(remaining_prompt_body) = req.prompt.clone().strip_prefix("{patient_id=") {
                    if let Some((raw_patient_id, remaining_prompt_body_1)) = remaining_prompt_body.split_once('}') {
                        default_patient = raw_patient_id.parse().unwrap();
                        remaining_prompt_body_1.to_string().replace("}", "") //we are left with a "}" prefix, which we don't really care about. It will not affect the classification algorithm (currently).
                    }
                    else{
                        remaining_prompt_body.to_string() // if we must, we just proceed with the prompt, stripped of most of its prefix.
                    }
                }
                else{
                    req.prompt.clone() // if we must, we just proceed with the prompt as-is
                }
            }
        };

        let context_level: i8;
        let prompt_level_1: String;
        let prompt: String;
        // next extract the context level: either no patient (0), within a patient/encounter (1) or within a patient's intervention (2)
        if prompt_level_0.contains("{ctxtlvl=1}"){ // within a patient/encounter (1)
            prompt_level_1 = prompt_level_0.replace("{ctxtlvl=1}", "");
            context_level = CommandController::CONTEXT_LEVEL_REQUIRES_PATIENT;
        }
        else if prompt_level_0.contains("{ctxtlvl=2}"){ //within a patient's intervention (2)
            prompt_level_1 = prompt_level_0.replace("{ctxtlvl=2}", "");
            context_level = CommandController::CONTEXT_LEVEL_REQUIRES_PATIENT_INTERVENTION;
        }
        else{ // otherwise assume level 0, no params
            prompt_level_1 = prompt_level_0.replace("{ctxtlvl=0}", "");
            context_level = CommandController::CONTEXT_LEVEL_NO_PATIENT_REQUIRED; // redundant, but better for code clarity - conversely "patient unknown, may or may not be present"
        }
        prompt = prompt_level_1;

        println!("Revised prompt and context level: '{}', ({})", prompt.clone(), context_level);
        println!("Default_patient: id={}", default_patient.clone());        

        let cur_session: Option<UserSession> = user_session.get(constants::USER_SESSION).unwrap();

        let userid = cur_session.unwrap().clone().get_userid_as_i64();
                
        let nle = NaturalLanguageEngine::new( &app_session.get_full_path_language_model_file(),
                                                                 &app_session.get_full_path_tokenizer_file()
        ).await;
        
        // load the command controller structure, to manage proper use of the Language Engine
        let mut cmd: CommandController = CommandController::new(&app_session.get_full_path_command_mapping_file(), nle );

        let cur_session: UserSession = user_session.get(constants::USER_SESSION).unwrap().unwrap();
        let pdao = PatientDAO::new( app_session.get_db_connection() ).await;
        let referenced_patient = CommandController::get_referenced_patient(pdao, userid, prompt.clone()).await; // perform a basic search within the prompt for any of the current patients

        let mut prompt_final = prompt.clone();

        let patient_id = match referenced_patient.clone().0 {
            CommandController::NO_PATIENT_FOUND => {
                if default_patient != constants::INVALID_PATIENT_ID {
                    default_patient // if the patient is still invalid, but we have a value patient from the context, provide that instead
                }
                else{
                    constants::INVALID_PATIENT_ID
                }
            },
            CommandController::KNOWN_PATIENT_FOUND =>{
                let p: Patient = referenced_patient.clone().1.unwrap(); // pull out the patient's name and adjust the prompt prior to matching
                prompt_final = prompt.clone().replace(&p.legal_first_name, "patient").replace(&p.legal_last_name, "patient");
                p.id
            },
            _ => { // otherwise reduce the list of results.
                match referenced_patient.clone().1 {
                    Some(rp) => rp.id,
                    None => default_patient,// if there was a default patient in the prompt, we return it. Otherwse it gets constants::INVALID_PATIENT_ID by default
                }
            },
        };

        tracing::debug!("..revised prompt: {}", prompt_final.clone());
        tracing::debug!("..revised patient_id: {}", patient_id);
        tracing::debug!("..context_level: {}", context_level);
        println!("..revised prompt: {}", prompt_final.clone());
        println!("..revised patient_id: {}", patient_id);
        println!("..context_level: {}", context_level);

        let classifer_results_final: Vec< (String, f32)> = cmd.get_filtered_classifier_rankings( prompt_final, cur_session.clone().user_authorizations, context_level).await;

        results_sbuf.push_str( &NLECommandFormatter::get_nle_options_content(classifer_results_final, cmd, patient_id, referenced_patient.1 ));
        
        HttpResponse::Ok().body( results_sbuf )
    }
}