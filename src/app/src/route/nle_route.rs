//! Natural Language prompt and related routes
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 
use std::path::Path;
use std::collections::{HashSet};
use tracing;

use crate::constants;
use crate::nle::controller::CommandController;
use crate::ui::data_forms::*;
use crate::session::{AppSession, UserSession};
use crate::nle::nle::*;

//pub const COMMAND_MAPPING_FILE_NAME: &str = "command_mapping.csv";
//pub const DATA_SUB_DIRECTORY: &str = "data";
//pub const LANGUAGE_MODEL_FILE_NAME: &str = "all-MiniLM-L6-v2.onnx";
//pub const TOKENIZER_FILE_NAME: &str = "tokenizer.json";

pub struct NLERoute{}

impl NLERoute{


   /*   pub async fn natural_language_prompt_test2(&self, _app_session: web::Data<AppSession>, _user_session: Session, req: web::Form<NLPromptFormData>) -> impl Responder {
        println!("-> /nlprompt Requested;  natural_language_prompt_test2();  prompt: \"{}\"", req.prompt);


        
        
        HttpResponse::Ok().body( "SUCCESS" )
    }*/

    /// accepts a natural language prompt and processes it using the built in engine
    /// 
    pub async fn natural_language_prompt(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<NLPromptFormData>) -> impl Responder {
        tracing::info!("-> /nlprompt Requested;  natural_language_prompt();  prompt: \"{}\"", req.prompt);

        let mut results_sbuf = String::with_capacity(500); // Single heap allocation
        
        let prompt = req.prompt.clone();
       // let patient_id = req.patient_id.clone();

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
        let mut unique_ids: HashSet<i64> = HashSet::new();
        let mut user_options: Vec<(String, f32, i64)> = Vec::new();

        results_sbuf.push_str( "<div id=\"MapleEMR::NLPCanvas\">" );
        results_sbuf.push_str(&format!( "<!-- Prompt :\n {} -->", prompt )  );
        
        results_sbuf.push_str("<div class='data'>Here are some options, based on your prompt:<p>");
        results_sbuf.push_str("<form action=\"/nlprompt\" method=\"post\" id=\"nlpActionCmdForm\" name=\"nlpActionCmdForm\" onSubmit=\"event.preventDefault(); return performNLAction(0)\" align=\"right\" class=\"nlpCommandAreaCls\">");

        let option_limit = 3;

        for item in items.into_iter(){
            
            let permission = cmd.get_permission_and_label_for_operation(item.clone().0);
            //let permission_id = cmd.get_permission_for_operation(item.clone().0);
            
            // check if we already have the option captured, up to the upper limit
            if (unique_ids.len() < option_limit) && !unique_ids.contains( &permission.0 ){
                unique_ids.insert(permission.0);
                user_options.push( (permission.1, item.1, permission.0) );
            }
        }

        for item in user_options.into_iter(){
            results_sbuf.push_str( "<input type='button' id='nlp_action_");
            results_sbuf.push_str( &item.2.to_string() ); 
            results_sbuf.push_str( "' name='nlp_action_" );
            results_sbuf.push_str( &item.2.to_string() ); 
            results_sbuf.push_str( "' value='" );
            results_sbuf.push_str( &item.0);
            //results_sbuf.push_str( &format!("{}: {:.1}% => id={}' \\><p>", item.0, item.1 * 100., permission_id) );

            results_sbuf.push_str("' onclick=\"performNLAction(");
            results_sbuf.push_str( &item.2.to_string() ); 
            results_sbuf.push_str( "); return false;\" \\>" );
        }
        results_sbuf.push_str("</div></div></form><p>");
        results_sbuf
    }
}