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
use crate::nle::controller::CommandController;
use crate::webc::data_forms::*;//, menu_tile::*};
use crate::session::AppSession;
use crate::session::UserSession;
use crate::nle::nle::*;

pub const COMMAND_MAPPING_FILE_NAME: &str = "command_mapping.csv";
pub const DATA_SUB_DIRECTORY: &str = "data";
pub const LANGUAGE_MODEL_FILE_NAME: &str = "all-MiniLM-L6-v2.onnx";
pub const TOKENIZER_FILE_NAME: &str = "tokenizer.json";


pub struct NLERoute{}

impl NLERoute{


   /*   pub async fn natural_language_prompt_test2(&self, _app_session: web::Data<AppSession>, _user_session: Session, req: web::Form<NLPromptFormData>) -> impl Responder {
        println!("-> /nlprompt Requested;  natural_language_prompt_test2();  prompt: \"{}\"", req.prompt);


        
        
        HttpResponse::Ok().body( "SUCCESS" )
    }*/

    /// accepts a natural language prompt and processes it using the built in engine
    /// 
    pub async fn natural_language_prompt(_app_session: web::Data<AppSession>, user_session: Session, req: web::Form<NLPromptFormData>) -> impl Responder {
        println!("-> /nlprompt Requested;  natural_language_prompt();  prompt: \"{}\"", req.prompt);

        let mut results_sbuf = String::with_capacity(500); // Single heap allocation
        
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

                
        let nle = NaturalLanguageEngine::new( &Path::new( &base_model_dir )
                                                                        .join(DATA_SUB_DIRECTORY)
                                                                        .join(LANGUAGE_MODEL_FILE_NAME).to_string_lossy(),

                                                    &Path::new( &base_model_dir )
                                                                        .join(DATA_SUB_DIRECTORY)
                                                                        .join(TOKENIZER_FILE_NAME).to_string_lossy()
        ).await;
        
        // load the command controller structure, to manage proper use of the Language Engine
        let mut cmd: CommandController = CommandController::new(&Path::new( &base_model_dir )
                                                            .join(DATA_SUB_DIRECTORY)
                                                            .join(COMMAND_MAPPING_FILE_NAME).to_string_lossy(), nle );
       

        let cur_session: Option<UserSession> = user_session.get(constants::USER_SESSION).unwrap();


        //let classifer_results: Vec< (String, f32)> = cmd.get_classifier_rankings( prompt.clone() ).await;
        let classifer_results: Vec< (String, f32)> = cmd.get_classifier_rankings_filtered_for_permissions( prompt.clone(), cur_session.unwrap().user_authorizations ).await;
        

        //results_sbuf.push_str("<H1>natural language prompt</H1>\n");
        results_sbuf.push_str( &NLERoute::get_nle_options_content(classifer_results, prompt, cmd) );
        
        HttpResponse::Ok().body( results_sbuf )
    }

    ///
    /// 
    /// 
    pub fn get_nle_options_content( items: Vec< (String, f32)>, prompt: String, cmd: CommandController) -> String{
        let mut results_sbuf = String::with_capacity(500); 
        results_sbuf.push_str( "<div id=\"MapleEMR::NLPCanvas\">" );
        results_sbuf.push_str(&format!( "<!-- Prompt :\n {} -->", prompt )  );
        
        results_sbuf.push_str("<div class='data'>Here are some options, based on your prompt:<p>");

        results_sbuf.push_str("<form action=\"/nlprompt\" method=\"post\" id=\"nlpCommandForm\" onSubmit=\"event.preventDefault(); return performNLPrompt(0)\" align=\"right\" class=\"nlpCommandAreaCls\">");
        for item in items.into_iter().take(3){
            let permission_id = cmd.get_permission_for_operation(item.clone().0);

            results_sbuf.push_str( "<input type='button' id='nlp_action_");
            results_sbuf.push_str( &permission_id.to_string() ); 
            results_sbuf.push_str( "' name='nlp_action_" );
            results_sbuf.push_str( &permission_id.to_string() ); 
            results_sbuf.push_str( "' value='" );
            results_sbuf.push_str( &item.clone().0 );
            //results_sbuf.push_str( &format!("{}: {:.1}% => id={}' \\><p>", item.0, item.1 * 100., permission_id) );

            results_sbuf.push_str( "'\\>" );
            results_sbuf.push_str("<td><a href=\"#\" onclick=\"performNLPrompt("  ); 
            results_sbuf.push_str( &permission_id.to_string() ); 
            results_sbuf.push_str("); return false;\">Click</a>"); 
        }
        results_sbuf.push_str("</div></div></form>");

        results_sbuf
    }
}