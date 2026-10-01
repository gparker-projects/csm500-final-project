//! Natural Language Engline (NLE) controller module
//!
//! CSM500 Project (April - October 2026)
//! Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tracing;

use crate::constants;
use crate::dto::user_auth::*;
use crate::nle::nle::*;
use crate::dto::patient::*;

/// # CommandController
/// 
/// ### Provides logic and constraints around commands being executed by the NL model
/// 
pub struct CommandController{
    command_hashset: Vec<(String, i64)>, // exposed for test purposes only
    full_command_hashset: Vec<(String, String, i64)>,
    nl_engine: NaturalLanguageEngine
}

impl CommandController{

    pub const NO_PATIENT_FOUND: i8 = 0;    
    pub const TARGET_PATIENT_FOUND: i8 = 2;
    pub const OTHER_PATIENT_FOUND: i8 = 1;

    pub const CONTEXT_LEVEL_NO_PATIENT_REQUIRED: i8 = 0; //context level: either no patient (0), within a patient/encounter (1) or within a patient's intervention (2)
    pub const CONTEXT_LEVEL_REQUIRES_PATIENT: i8 = 1;
    pub const CONTEXT_LEVEL_REQUIRES_PATIENT_INTERVENTION: i8 = 2;

    /// # CommandController::new()
    /// 
    ///  public constructor for the CommandController
    /// 
    /// ## Parameters:
    /// 
    /// * mapping_file_path: &str - the path to the command mapping.csv file
    /// * nle: NaturalLanguageEngine - a valid NaturalLanguageEngine object
    /// 
    /// ## Returns:
    /// 
    /// * A newly constructed CommandController
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

    /// # is_command_allowed_at_context_level()
    /// 
    ///  Given the id of a permission (command), return a true/false value indicating 
    ///  if it can be executed within the the context provided. Contexts are defined as 
    ///  constants above, in this class and are hierarchical:
    ///   - CONTEXT_LEVEL_NO_PATIENT_REQUIRED = 0           - The most broad context in the application where a Patient is not needed.
    ///   - CONTEXT_LEVEL_REQUIRES_PATIENT = 1              - Context requires a Patient to be executed.
    ///   - CONTEXT_LEVEL_REQUIRES_PATIENT_INTERVENTION = 2 - Context requires a Patient and an Intervention, to be executed.
    /// 
    /// ## Parameters:
    /// 
    /// * permission_id: i64 - the id of the permission to be checked
    /// * current_context_level: i8 - the current context level
    /// 
    /// ## Returns:
    /// 
    /// * bool: true if the given command is permitted at the current context level
    /// 
    pub fn is_command_allowed_at_context_level (permission_id: i64, current_context_level: i8) -> bool {
        tracing::info!("is_command_allowed_at_context_level()");
        let permitted_level: i8;

        if permission_id >= 100000 { // the permissions above 100000 are currently undefined, but reserved for intervention-level calls
            permitted_level = CommandController::CONTEXT_LEVEL_REQUIRES_PATIENT_INTERVENTION ; // 2
        }
        else if permission_id == 4 { // // admit new patient
            permitted_level = CommandController::CONTEXT_LEVEL_NO_PATIENT_REQUIRED; // 0
        }
        else if permission_id <= 12 { // we'll ignore Login as there is no Login (id=1) available when you're already in the system
            permitted_level = CommandController::CONTEXT_LEVEL_REQUIRES_PATIENT; // 1
        }
        else { // the permissions between 13 - 99999 are currently undefined
            permitted_level = CommandController::CONTEXT_LEVEL_NO_PATIENT_REQUIRED; // 0
        }

        tracing::info!("command_allowed_at_context_level({})", permission_id);
        tracing::info!("..current_context_level {} <= {} permitted_level ", current_context_level, permitted_level);

        // either meets the required level, or is "no patient context required"
        if permitted_level >= current_context_level { // || permitted_level == CommandController::CONTEXT_LEVEL_NO_PATIENT_REQUIRED
            tracing::info!("....allowed");
            return true;
        }
        return false;
    }

    /// # get_filtered_classifier_rankings()
    /// 
    ///   Obtains classifier rankings, only including items that the user has a permission for, and appropriate for the context level.
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

       // for item in classifer_results.clone(){
       //   tracing::info!("..(+) added permission for '{}' ('{:.1}%')", item.0, item.1 *100.);
       //}
       //tracing::info!("....added {} permissions, rejected {}", user_restricted_options.len(), rejected_counter);

        classifer_results
    }

    /// # get_permission_and_label_for_operation()
    /// 
    ///  Retrieves the id of the permission associated with the operation (column 0 from the command mapping)
    ///  that matches the prompt_string.
    /// 
    /// ## Parameters:
    /// 
    /// * prompt (String): the user provided prompt
    /// 
    /// ## Returns: Vec<(String, f32)>, which is a list of classifier rankings
    /// 
    /// * i64: Id of the command (permission) to be executed
    /// * String: User Label for the control
    /// 
    pub fn get_permission_and_label_for_operation (&self, prompt_string: String) -> (i64, String) {
        tracing::debug!("get_permission_and_label_for_operation(): Compare to prompt: '{}'", prompt_string);
        let mut results: (i64, String) = (constants::INVALID_OTHER_ID, "".to_string());
        for item in self.full_command_hashset.clone().iter(){
           if item.0 == prompt_string{
                let s = item.1.trim().to_string();
                results = ( item.2, s );  // phrase, user control label, id
                break; // terminate early if we find a match
           }
        }
        results
    }

    /// # read_lines<P>(filename: P) 
    /// 
    ///  Helper method to load the file lines into a BufReader, to better control flow
    /// 
    /// ## References:
    /// * https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    ///
    /// ## Parameters:
    /// 
    /// * filename (Path): the path to the file that is to be read
    /// 
    /// ## Returns:
    ///  * io::Result<io::Lines<io::BufReader<File>>> : the Buffered reader of the file to be read
    /// 
    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
    }

    /// # get_referenced_patient()
    /// 
    /// Checks if the user has access to any patients and if they were referenced in the prompt. If found, returns the basics of the record (Id, First and Last name).
    /// 
    /// Note: Rule is that oly a single patient may be referenced in a prompt, or more specifically, only one will be recognized and returned.
    ///
    /// ## Parameters:
    /// 
    /// * patients_list: Vec<Patient> - the list of patients to be used to search within the prompt
    /// * _audit_userid: i64 - future feature only; the user for auditing purposes
    /// * prompt: String - the prompt to be searched for the patient.
    /// 
    /// ## Returns: (i8, Option<Patient>): a tuple of the results code and an Option<Patient>
    /// * i8: - NO_PATIENT_FOUND (0) if the patient was not found and NO patient was found
    ///       - TARGET_PATIENT_FOUND (2) if the patient indicated by the prompt was found
    ///       - OTHER_PATIENT_FOUND (1) if the target patient was not found, but we've been able to provide a substitute.
    /// 
    pub async fn get_referenced_patient(patients_list: Vec<Patient>, _audit_userid: i64, prompt: String) -> (i8, Option<Patient>){
        //tracing::info!("get_referenced_patient()");
        println!("get_referenced_patient()");

        let mut result_code: i8 = CommandController::NO_PATIENT_FOUND;
        let mut result: Option<Patient> = None;
        let tmp_prompt = prompt.to_lowercase(); // ensure the prompt matches case with the search string

        println!("..checking prompt: {}", tmp_prompt.clone());

        for p in patients_list.clone() {
            //tracing::info!("..checking for: {}, {}", &p.legal_last_name, &p.legal_first_name);
            println!("..checking against: {}, {}", &p.legal_last_name, &p.legal_first_name);
            if tmp_prompt.contains(&p.legal_first_name.to_lowercase()) || tmp_prompt.contains(&p.legal_last_name.to_lowercase()){
                //tracing::info!("...> matched patient: {}, {}", &p.legal_last_name, &p.legal_first_name);
                println!("...> matched patient: {}, {}", &p.legal_last_name, &p.legal_first_name);
                result_code = CommandController::TARGET_PATIENT_FOUND;
                result = Some(p);
                break;
            }
        }

        // if the target patient was not found, return the first one and update the status code to OTHER_PATIENT_FOUND
        if result_code == CommandController::NO_PATIENT_FOUND  {
            println!("...<< target patient was not found, substituting with first patient");
            //if patients_list.len() > 0 { //  a zero-patient list would mean there are none in the facility, which is not realistic
            result_code = CommandController::OTHER_PATIENT_FOUND;
            let p_list = patients_list.clone();
            result = Some(p_list.first().unwrap().clone()); // this was kind of crazy
            //}
        }

        println!("..returning patient: {} {}", result.clone().unwrap().legal_first_name, result.clone().unwrap().legal_last_name);

        ( result_code, result )
    }

    /// # unit_test_purge_command_hashset()
    /// 
    ///  Resets the command hashset to a new vector. For Unit Testing purposes only.
    ///
    #[allow(dead_code)]
    pub fn unit_test_invalidate_command_hashset(&mut self){
        self.command_hashset = Vec::new();
        self.command_hashset.push( (String::new(), -1) );
    }
}