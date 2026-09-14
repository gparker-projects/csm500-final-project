//! Natural Language Engline (NLE) controller module
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use sqlx::types::uuid::timestamp::context;
use tracing;

use crate::constants;
use crate::dto::user_auth::*;
use crate::nle::nle::*;

use crate::dao::patient_dao::*;
use crate::dto::patient::*;

///
/// # CommandController
/// ### Provides logic and constraints around commands being executed by the NL model
/// 
pub struct CommandController{
    command_hashset: Vec<(String, i64)>, 
    full_command_hashset: Vec<(String, String, i64)>,
    nl_engine: NaturalLanguageEngine
}

impl CommandController{

    pub const NO_PATIENT_FOUND: i8 = 0;
    //pub const UNKNOWN_PATIENT_FOUND: i8 = 1;
    pub const KNOWN_PATIENT_FOUND: i8 = 2;

    pub const CONTEXT_LEVEL_NO_PATIENT_REQUIRED: i8 = 0; //context level: either no patient (0), within a patient/encounter (1) or within a patient's intervention (2)
    pub const CONTEXT_LEVEL_REQUIRES_PATIENT: i8 = 1;
    pub const CONTEXT_LEVEL_REQUIRES_PATIENT_INTERVENTION: i8 = 2;


    ///
    /// Public constructor for the CommandController
    /// 
    pub fn new(mapping_file_path: &str, nle: NaturalLanguageEngine) -> Self {
        tracing::debug!( "CommandController::New() {}", mapping_file_path );
        let mut tmp_command_hashset = Vec::<(String, i64)>::with_capacity(10);
        let mut tmp_full_command_hashset = Vec::<(String, String, i64)>::with_capacity(10);

        // read in the command mapping config .CSV
        if let Ok(lines) = CommandController::read_lines( mapping_file_path ) {
            for line in lines.map_while(Result::ok) {
                if !line.starts_with("#") { //ignore comment lines, identified as starting with #
                    let parts: Vec<&str> = line.split(',').collect();

                    // convert to an i64, matching all other system data structures
                    // be sure to trim() first, otherwise the parse fails!
                    let tmp_index: i64 = parts[1].trim().parse::<i64>().unwrap_or(constants::INVALID_OTHER_ID); 
                    tmp_command_hashset.push( ( parts[0].to_string(), tmp_index ) ); // phrase, id
                    tmp_full_command_hashset.push( ( parts[0].to_string(), parts[2].to_string(), tmp_index ) ); // phrase, user control label, id
                }
            }
        }

        CommandController{
            command_hashset: tmp_command_hashset,
            full_command_hashset: tmp_full_command_hashset,
            nl_engine: nle
        }
    }

    
    // given the id of a permission (command), return the context it is allowed to operate in
    pub fn is_command_allowed_at_context_level (permission_id: i64, current_context_level: i8) -> bool{
        let required_level: i8;

        if permission_id >= 100000 { // the permissions above 100000 are currently undefined, but reserved for intervention-level calls
            required_level = CommandController::CONTEXT_LEVEL_REQUIRES_PATIENT_INTERVENTION ;
        }
        else if permission_id == 4 { // // admit new patient
            required_level = CommandController::CONTEXT_LEVEL_NO_PATIENT_REQUIRED;
        }
        else if permission_id <= 12 { // we'll ignore Login as there is no Login (id=1) available when you're already in the system
            required_level = CommandController::CONTEXT_LEVEL_REQUIRES_PATIENT;
        }
        else { // the permissions between 13 - 99999 are currently undefined
            required_level = CommandController::CONTEXT_LEVEL_NO_PATIENT_REQUIRED; 
        }

        println!("command_allowed_at_context_level({})", permission_id);
        println!("..current_context_level {} == {} required_level ", current_context_level, required_level);

        // either meets the required level, or is "no patient context required"
        if required_level >= current_context_level || required_level == CommandController::CONTEXT_LEVEL_NO_PATIENT_REQUIRED {
            println!("....allowed");
            return true;
        }
        return false;
    }

    ///
    /// # Obtains classifier rankings, only including items that the user has a permission for, and appropriate for the context level.
    ///   E.g. Some users can not access certain permissions AND the system should not allow some functions to be performed at different
    ///        context levels (e.g. Add a request for Bloodwork, when there is no Patient selected)
    /// 
    /// ## Parameters:
    /// 
    /// * prompt (String): the user provided prompt
    /// * user_auths (UserAuthorization): user authorization wrapper of permissions the user is allowed to execute
    /// 
    /// ## Returns: Vec<(String, f32)>, which is a list of classifier rankings
    /// 
    /// * String: Sentence that was evaluated
    /// * f32: Resulting percentage of success of the comparison against the prompt
    /// 
    pub async fn get_filtered_classifier_rankings(&mut self, prompt: String, user_auths: UserAuthorization, context_level: i8 ) -> Vec<(String, f32)>{
        tracing::debug!("get_filtered_classifier_rankings()");
        let mut user_restricted_options: Vec<String> = Vec::new();

        let mut rejected_counter = 0;
        for item in self.command_hashset.clone() { // check if the user has a permission (as identified in the command mapping) before allowing it to be used in the prompt lookup
            if user_auths.has_permission(item.1){

                // if the command has the required context level, the user will receive it as an option
                if CommandController::is_command_allowed_at_context_level( item.1, context_level) {
                    user_restricted_options.push(item.0);
                }
            }
            else{
                rejected_counter = rejected_counter + 1;
            }
        }
        
        user_restricted_options.insert(0, prompt.clone() );        // the NLE wants the prompt we're checking against to be the first element of the list, so prepend it to the vector
        
        // get the classification rankings from the NL engine
        let classifer_results: Vec<(String, f32)> = self.nl_engine.get_classifier_rankings(user_restricted_options.clone() ).await;

        for item in classifer_results.clone(){
        //    tracing::info!("..(+) added permission for '{}' ('{:.1}%')", item.0, item.1 *100.);
            println!("..(+) added permission for '{}' ('{:.1}%')", item.0, item.1 *100.);
        }

        println!("....added {} permissions, rejected {}", user_restricted_options.len(), rejected_counter);

        classifer_results
    }

    ///
    /// Retrieves the id of the permission associated with the operation (column 0 from the command mapping)
    ///  that matches the prompt_string.
    /// 
    pub fn get_permission_and_label_for_operation (&self, prompt_string: String) -> (i64, String) {
        tracing::debug!("get_permission_and_label_for_operation(): Compare to prompt: '{}'", prompt_string);
        let mut results: (i64, String) = (constants::INVALID_OTHER_ID, "".to_string());
        for item in self.full_command_hashset.clone().iter(){
           if item.0 == prompt_string{
                results = (item.2, item.1.to_string());  // phrase, user control label, id
                break; // terminate early if we find a match
           }
        }
        results
    }

    ///
    /// Helper method to load the file lines into a BufReader, to better control flow
    /// REF: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    ///
    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
    }

    ///
    /// Checks if the user has access to any patients and if they were referenced in the prompt. If found, returns the basics of the record (Id, First and Last name).
    /// 
    /// Note: Rule is that oly a single patient may be referenced in a prompt, or more specifically, only one will be recognized and returned.
    /// 
    pub async fn get_referenced_patient(pdao: PatientDAO, userid: i64, prompt: String) -> (i8, Option<Patient>){
        tracing::debug!("get_referenced_patient():prompt='{}'", prompt.clone());
        let patients_list = pdao.get_patients_at_users_site_no_discharge(userid).await.expect( constants::DATABASE_ERROR_NOT_FOUND ).unwrap();
        let mut result_code: i8 = Self::NO_PATIENT_FOUND;
        let mut result: Option<Patient> = None;
        let mut first_patient: Option<Patient> = None;
        let tmp_prompt = prompt.to_lowercase();

        for p in patients_list {
            tracing::debug!("..checking for: {}, {}", &p.legal_last_name, &p.legal_first_name);
            if tmp_prompt.contains(&p.legal_first_name.to_lowercase()) || tmp_prompt.contains(&p.legal_last_name.to_lowercase()){
                tracing::debug!("...> matched patient: {}, {}", &p.legal_last_name, &p.legal_first_name);
                result_code = Self::KNOWN_PATIENT_FOUND;
                result = Some(p);
                break;
            }
            first_patient = match first_patient {
                None => Some (p),
                Some(p) => Some (p)
            };
        }
        if result_code == Self::NO_PATIENT_FOUND{
            tracing::debug!("..no patients found in prompt.");
        }

        result = match result {
            None =>  first_patient ,
            Some(p) => first_patient 
        };

        // if there was no known patient found, check if there is a new patient
        /* if result_code == Self::NO_PATIENT_FOUND {
            //if prompt.contains("admit") // TODO: replace this with an NLE check for "Admission"
            result_code = Self::UNKNOWN_PATIENT_FOUND;
            result = None;
        }*/

        ( result_code, result )
    }
}