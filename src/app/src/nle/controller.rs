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

    ///
    /// # Obtains classifier rankings, only including items that the user has a permission for
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
    pub async fn get_classifier_rankings_filtered_for_permissions(&mut self, prompt: String, user_auths: UserAuthorization ) -> Vec<(String, f32)>{
        tracing::debug!("get_classifier_rankings_filtered_for_permissions()");
        let ops_add_prompt: Vec<String> = self.get_all_operations_and_add_prompt( prompt.clone() );
        let mut results: Vec<(String, f32)> = vec![]; 
        let classifer_results: Vec<(String, f32)> = self.nl_engine.get_classifier_rankings(ops_add_prompt ).await;

        // check that the user has the permission before adding it 
        for c_result in classifer_results{
            let pid: i64 = self.get_permission_for_operation (c_result.clone().0);
            if user_auths.has_permission(pid){
                tracing::debug!("..(+) adding permission ({}) for: {}", pid, c_result.clone().0);
                //println!("..(+) adding permission ({}) for: {}", pid, c_result.clone().0);
                results.push( c_result );
            }
            //else{
            //    tracing::debug!("..(x) no permission found for: {}, excluding", c_result.clone().0);
                //println!("..(x) no permission found for: {}, excluding", c_result.clone().0);
            //}
        }

        for item in results.clone(){
            tracing::info!("..(+) added permission for '{}' ('{:.1}%')", item.0, item.1 *100.);
            println!("..(+) added permission for '{}' ('{:.1}%')", item.0, item.1 *100.);
        }

        results
    }

    ///
    /// Constructs a list of strings ( Vec<String> ) from the previously loaded command mapping file, and adds the user's prompt as the first element
    /// This required by the NL model we are using ATM, for its comparison routine.
    /// 
    pub fn get_all_operations_and_add_prompt (&self, prompt: String) -> Vec<String> {
        // take the list we loaded, cut it into separate vectors by the columns
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.unzip
        //
        let  (mut sentences, _ignore_command_id): (Vec::<String>, Vec::<i64>) = self.command_hashset.clone().into_iter().unzip();
        sentences.insert(0, prompt.clone() );

        return sentences;
    }

    ///
    /// Retrieves the id of the permission associated with the operation (column 0 from the command mapping)
    ///  that matches the prompt_string.
    /// 
    pub fn get_permission_for_operation (&self, prompt_string: String) -> i64 {
        tracing::debug!("get_permission_for_operation(): Compare to prompt: '{}'", prompt_string);
        let mut results: i64 = constants::INVALID_OTHER_ID;
        for item in self.command_hashset.clone().iter(){
           if item.0 == prompt_string{
                results = item.1;
                break; // terminate early if we find a match
           }
        }
        results
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
        let tmp_prompt = prompt.to_lowercase();

        for p in patients_list {
            tracing::debug!("..checking for: {}, {}", &p.legal_last_name, &p.legal_first_name);
            if tmp_prompt.contains(&p.legal_first_name.to_lowercase()) || tmp_prompt.contains(&p.legal_last_name.to_lowercase()){
                tracing::debug!("...> matched patient: {}, {}", &p.legal_last_name, &p.legal_first_name);
                result_code = Self::KNOWN_PATIENT_FOUND;
                result = Some(p);
                break;
            }
        }
        if result_code == Self::NO_PATIENT_FOUND{
            tracing::debug!("..no patients found in prompt.");
        }

        // if there was no known patient found, check if there is a new patient
        /* if result_code == Self::NO_PATIENT_FOUND {
            //if prompt.contains("admit") // TODO: replace this with an NLE check for "Admission"
            result_code = Self::UNKNOWN_PATIENT_FOUND;
            result = None;
        }*/

        ( result_code, result )
    }
}