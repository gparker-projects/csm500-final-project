
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tracing;

use crate::constants;
use crate::dto::user_auth::*;
use crate::nle::nle::*;


// Commands that can be executed via the NL model
pub const COMMAND_ADMIT_NEW_PATIENT: i64 = 1;
pub const COMMAND_ADD_NEW_INTERVENTION: i64 = 2;
pub const COMMAND_DISCHARGE_PATIENT: i64 = 3;

///
/// Provides logic and constraints around commands being executed by the NL model
/// 
pub struct CommandController{
    command_hashset: Vec<(String, i64)>, 
    full_command_hashset: Vec<(String, String, i64)>,
    nl_engine: NaturalLanguageEngine
}

impl CommandController{
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
                let parts: Vec<&str> = line.split(',').collect();

                // convert to an i64, matching all other system data structures
                // be sure to trim() first, otherwise the parse fails!
                let tmp_index: i64 = parts[1].trim().parse::<i64>().unwrap_or(constants::INVALID_OTHER_ID); 
                tmp_command_hashset.push( ( parts[0].to_string(), tmp_index ) ); // phrase, id
                tmp_full_command_hashset.push( ( parts[0].to_string(), parts[2].to_string(), tmp_index ) ); // phrase, user control label, id
            }
        }

        CommandController{
            command_hashset: tmp_command_hashset,
            full_command_hashset: tmp_full_command_hashset,
            nl_engine: nle
        }
    }
    

    ///
    /// Obtains classifier rankings, only inlcuding items that the user has a permission for
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
                results.push( c_result );
            }
        }
        results
    }


    ///
    /// Obtains basic classifier rankings, without limits or security concerns applied
    /// 
    pub async fn get_classifier_rankings(&mut self, prompt: String ) -> Vec< (String, f32)>{
        tracing::debug!("get_classifier_rankings()");
        let ops_add_prompt: Vec<String> = self.get_all_operations_and_add_prompt( prompt.clone() );

        let classifer_results: Vec< (String, f32)> = self.nl_engine.get_classifier_rankings(ops_add_prompt ).await;

        classifer_results 
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
        tracing::debug!("get_permission_for_operation(): Compare to prompt: '{}'", prompt_string);
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



    // get patients

    // get_workflow
      // does it contain a patient?
      // does it match an operation?
}