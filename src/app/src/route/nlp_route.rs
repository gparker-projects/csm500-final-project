//! # Natural Language prompt and related routes
//!
//!      CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 

//use crate::dao::{patient_dao::*, intervention_dao::*, encounter_dao::*}; 
use crate::webc::data_forms::*;//, menu_tile::*};
use crate::session::AppSession;//, UserSession};

//use crate::constants;

pub struct NLPRoute{}

impl NLPRoute{

    /// performs a natural language prompt using the built in engine
    /// 
    /// check by going to: http://127.0.0.1:8000/db
    /// 
    //async fn natural_language_prompt(req: web::Form<NLPromptFormData>) -> impl Responder {
    pub async fn natural_language_prompt(_app_session: web::Data<AppSession>, _user_session: Session, req: web::Form<NLPromptFormData>) -> impl Responder {
    
        println!("-> /nlprompt Requested; prompt: \"{}\"", req.prompt);

        let mut results_sbuf = String::with_capacity(50); // Single heap allocation
        results_sbuf.push_str("<b>PLACEHOLDER CONTENT/b>\n");

        if req.prompt.contains("discharge") {
            //actix_web::web::Redirect::to("/admdis").using_status_code(StatusCode::SEE_OTHER)
            HttpResponse::Ok().body(format!( r##"{{"action": "discharge", "prompt": "{}",}}"##, req.prompt)) 
        }
        else if req.prompt.contains("admit")  {
            HttpResponse::Ok().body(format!( r##"{{"action": "admit", "prompt": "{}",}}"##, req.prompt))
        }
        else {
            HttpResponse::Ok().body(format!(r##"{{"action": "other", "prompt": "{}",}}"##, req.prompt)) 
        }

        //actix_web::web::Redirect::to("/home").using_status_code(StatusCode::SEE_OTHER)
    
        // these are the ACTUAL execution from the POC
        //let results = nlp::NLP{}.execute();
        //HttpResponse::Ok().body(format!("<b>machine_learn_test {}</b>", results.await.to_string())) 

        //HttpResponse::Ok().body(format!("{}", results_sbuf)) 
    }
}