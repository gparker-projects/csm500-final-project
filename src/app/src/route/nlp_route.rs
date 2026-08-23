//! # Natural Language prompt and related routes
//!
//!      CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 
use std::env;
use std::path::Path;

use crate::constants;
use crate::nlp::controller::CommandController;
use crate::webc::data_forms::*;//, menu_tile::*};
use crate::session::AppSession;//, UserSession};
use crate::nlp::nlp::*;

pub const COMMAND_MAPPING_FILE_NAME: &str = "command_mapping.csv";
pub const DATA_SUB_DIRECTORY: &str = "data";
pub const LANGUAGE_MODEL_FILE_NAME: &str = "all-MiniLM-L6-v2.onnx";
pub const TOKENIZER_FILE_NAME: &str = "tokenizer.json";

pub struct NLPRoute{}

impl NLPRoute{

    /// accepts a natural language prompt and processes it using the built in engine
    /// 
    pub async fn natural_language_prompt(_app_session: web::Data<AppSession>, _user_session: Session, req: web::Form<NLPromptFormData>) -> impl Responder {
        println!("-> /nlprompt Requested; prompt: \"{}\"", req.prompt);

        let mut results_sbuf = String::with_capacity(50); // Single heap allocation
        results_sbuf.push_str("<H1>natural language prompt</H1>\n");
    
        let prompt = req.prompt.clone();
       // let patient_id = req.patient_id.clone();

        // collect the cargo manifest directory at runtime, which means it might not be present
        let base_model_dir = match env::var(constants::CARGO_MANIFEST_DIR) {
            Ok(tmp_path) => {
                println!("CARGO_MANIFEST_DIR = {}", tmp_path);
                tmp_path
            }
            Err(e) => {
                println!("CARGO_MANIFEST_DIR not set: {}", e);
                "INVALID_PATH".to_string()
            }
        };
        
        // load the command controller structure, to manage proper use of the Language Engine
        let cmd: CommandController = CommandController::new(&Path::new( &base_model_dir )
                                                            .join(DATA_SUB_DIRECTORY)
                                                            .join(COMMAND_MAPPING_FILE_NAME).to_string_lossy() );
       
        
        let mut nlp = NaturalLanguageEngine::new( &Path::new( &base_model_dir )
                                                                        .join(DATA_SUB_DIRECTORY)
                                                                        .join(LANGUAGE_MODEL_FILE_NAME).to_string_lossy(),

                                                    &Path::new( &base_model_dir )
                                                                        .join(DATA_SUB_DIRECTORY)
                                                                        .join(TOKENIZER_FILE_NAME).to_string_lossy()
        ).await;

        // TODO: turn this inside out, with cmd getting a clone of the NLP, running the classifier rankings, then limiting the results to the 
        //       top three unique actions, which the user is actually allowed to perform. will need user from session
        //
        let results: Vec< (String, f32)> = nlp.get_classifier_rankings( cmd.get_all_operations_and_add_prompt( prompt.clone() ) ).await;

        results_sbuf.push_str(&format!( "<b>Prompt</b>:\n {}<br>", prompt )  );
        //results_sbuf.push_str("* ");
        for item in results.into_iter().take(3){
            let permission_id = cmd.get_permission_for_operation(item.clone().0);

            //results_sbuf.push_str( &format!("<br>'{}': {:.1}% => Command id={}", item.0, item.1 * 100., permission_id) );
            results_sbuf.push_str( &format!("<input type='button' id='action_do' name='action_do' value='") );
            results_sbuf.push_str( &format!("{}: {:.1}% => id={}' \\><br>", item.0, item.1 * 100., permission_id) );
        }
       // results_sbuf.push_str("</ul>");
        
        HttpResponse::Ok().body( results_sbuf )
    }
}