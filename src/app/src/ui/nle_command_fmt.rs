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

pub struct NLECommandFormatter{}

impl NLECommandFormatter {
    ///
    /// 
    /// 
    pub fn get_nle_options_content( items: Vec< (String, f32)>, prompt: String, cmd: CommandController, patient_id: i64) -> String{
        tracing::debug!("get_nle_options_content()");
        
        let mut results_sbuf = String::with_capacity(500); 
        let mut unique_ids: HashSet<i64> = HashSet::new();
        let mut user_options: Vec<(String, f32, i64)> = Vec::new();

        results_sbuf.push_str( "<div id=\"MapleEMR::NLPCanvas\">" );
        results_sbuf.push_str(&format!( "<!-- Prompt :\n {} -->", prompt )  );
        
        results_sbuf.push_str("<div class='data'>Here are some options, based on your prompt:<p>");
        results_sbuf.push_str("<form action=\"/nlprompt\" method=\"post\" id=\"nlpActionCmdForm\" name=\"nlpActionCmdForm\" onSubmit=\"event.preventDefault(); return performNLAction(0)\" align=\"right\" class=\"nlpCommandAreaCls\">");

        let option_limit = 3;

        for item in items.into_iter(){
            tracing::debug!("..obtain label, permission for: {} @ {}", item.0, item.1);
            //println!("..obtain label, permission for: {} @ {}", item.0, item.1);
            
            let permission = cmd.get_permission_and_label_for_operation(item.clone().0);
            tracing::debug!("..label, permission obtained: {} @ {}", permission.1, permission.0);
            //println!("..label, permission obtained: {} @ {}", permission.1, permission.0);
            
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
            results_sbuf.push_str( &item.0.trim()); // button name
            //results_sbuf.push_str( &format!("{}: {:.1}% => id={}' \\><p>", item.0, item.1 * 100., permission_id) );

            results_sbuf.push_str("' onclick=\"performNLAction(");
            results_sbuf.push_str( &item.2.to_string() ); // command action id (also the permission)
            results_sbuf.push_str( ","); 
            results_sbuf.push_str( &patient_id.to_string() ); // patient id
            results_sbuf.push_str( "); return false;\" \\>" );
        }
        results_sbuf.push_str("</div></div></form><p>");
        results_sbuf
    }
}