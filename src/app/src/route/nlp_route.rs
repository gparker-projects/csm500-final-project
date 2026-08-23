//! # Natural Language prompt and related routes
//!
//!      CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 

use crate::constants;
use crate::nlp::controller::CommandController;
use crate::webc::data_forms::*;//, menu_tile::*};
use crate::session::AppSession;//, UserSession};
use crate::nlp::nlp::*;

pub struct NLPRoute{}

impl NLPRoute{

    /// accepts a natural language prompt and processes it using the built in engine
    /// 
    pub async fn natural_language_prompt(_app_session: web::Data<AppSession>, _user_session: Session, req: web::Form<NLPromptFormData>) -> impl Responder {
        println!("-> /nlprompt Requested; prompt: \"{}\"", req.prompt);

        let mut results_sbuf = String::with_capacity(50); // Single heap allocation
        results_sbuf.push_str("<H1>NL Prompt test</H1>\n");
    
        let prompt = req.prompt.clone();
       // let patient_id = req.patient_id.clone();

       

        /*let inputs: Vec<String> = vec![prompt.clone(), // first index is the item we're matching against. All the others are matched against it
                                       "admit patient".to_string(),
                                       "discharge patient".to_string(),
                                       "add information".to_string(),
                                       "add medication".to_string(),
                                       "prescribe medication".to_string(),
                                       "move patient".to_string(),
                                       "update contact information".to_string(),
                                       ];*/

        let base_model_dir = env!("CARGO_MANIFEST_DIR");

        let cmd: CommandController = CommandController::new(&base_model_dir);
        let inputs: Vec<String> = cmd.get_all_operations_and_add_prompt( prompt.clone() );

        println!("# Inputs loaded: {}",inputs.clone().len());

        for item in inputs.clone(){
            println!("Input loaded: '{}'",item);
        }
        
        
        let mut nlp = NaturalLanguageEngine::new( base_model_dir ).await;

        results_sbuf.push_str(&format!( "<b>Prompt</b>:\n {}", prompt )  );

        println!("NaturalLanguageEngine::get_classification_rankings()");
        let results: Vec< (String, f32)> = nlp.get_classification_rankings( inputs.clone() ).await;

        results_sbuf.push_str("<ul>");
        for item in results{
            results_sbuf.push_str( &format!("<li>\t'{}': {:.1}% </li>", item.0, item.1 * 100.) );
        }
        results_sbuf.push_str("</ul>");
        
        HttpResponse::Ok().body( results_sbuf )
    }
}