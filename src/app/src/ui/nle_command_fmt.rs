//! Natural Language prompt and related routes
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use std::collections::{HashSet};
use tracing;

use crate::constants;
use crate::dto::patient::*;
use crate::nle::controller::CommandController;

pub struct NLECommandFormatter{}

impl NLECommandFormatter {
    ///
    /// 
    /// 
    pub fn get_nle_options_content( items: Vec< (String, f32)>, cmd: CommandController, patient_id: i64, referred_patient: Option<Patient>) -> String{
        tracing::debug!("get_nle_options_content()");
        
        let mut results_sbuf = String::with_capacity(500); 
        let mut unique_ids: HashSet<i64> = HashSet::new();
        let mut user_options: Vec<(String, f32, i64)> = Vec::new();
        let mut alt_patient_id: i64 = constants::INVALID_OTHER_ID;

        results_sbuf.push_str( "<div id=\"MapleEMR::NLPCanvas\">" );
  
        let patient_name: String; // if the prompt did not infer a patient id, we will use the forced one
        match referred_patient {
            Some(p) =>{
                patient_name = p.legal_first_name.to_owned()  + " " + &p.legal_last_name;
                alt_patient_id = p.id;
            },
            None =>{
                patient_name = String::new();
                if patient_id != constants::INVALID_OTHER_ID {
                    alt_patient_id = patient_id;
                } 
            },
        }; 

        if items.len() > 0 {
            results_sbuf.push_str("<div class='data'>Here are some options, based on your prompt:<p>\n");
            results_sbuf.push_str("<form action=\"/nlprompt\" method=\"post\" id=\"nlpActionCmdForm\" name=\"nlpActionCmdForm\" onSubmit=\"event.preventDefault(); return performNLAction();\" align=\"right\" class=\"nlpCommandAreaCls\">\n");

            let option_limit = 3;

            // iterate the list of options to ensure we are not duplicating any (names may be different, but actions should not be)
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


                if patient_name == String::new() {// button name
                    results_sbuf.push_str( &item.0.trim()); 
                }
                else{
                    results_sbuf.push_str( &item.0.trim().replace("Patient", &patient_name.clone()) ); 
                }
                //results_sbuf.push_str( &item.0.trim()); // button name

                results_sbuf.push_str("' onclick=\"performNLAction(");
                results_sbuf.push_str( &item.2.to_string() ); // command action id (also the permission)
                results_sbuf.push_str( ","); 
                results_sbuf.push_str( &alt_patient_id.to_string() ); // patient id
                results_sbuf.push_str( "); return false;\" \\>\n" );
            }
        }
        else{
            results_sbuf.push_str("<div class='data'>Your prompt did not result in any options. Please rephrase and try again.<p>\n");
        }
       
        results_sbuf.push_str("</div></div></form><p>\n");
        results_sbuf
    }
}