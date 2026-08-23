
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::constants;


// Commands that can be executed via the NL model
pub const COMMAND_ADMIT_NEW_PATIENT: i64 = 1;
pub const COMMAND_ADD_NEW_INTERVENTION: i64 = 2;
pub const COMMAND_DISCHARGE_PATIENT: i64 = 3;

///
/// Provides logic and constraints around commands being executed by the NL model
/// 
pub struct CommandController{
    command_hashset: Vec<(String, i64)>, // first item is a phrase for matching; second item is the index of a permission it would enable for the user, if matched
}

impl CommandController{
    ///
    /// Public constructor for the CommandController
    /// 
    pub fn new(mapping_file_path: &str) -> Self {
        //println!( "CommandController::New() {}", mapping_file_path );
        let mut tmp_command_hashset = Vec::<(String, i64)>::with_capacity(10);

        // read in the command mapping config .CSV
        if let Ok(lines) = CommandController::read_lines( mapping_file_path ) {
            for line in lines.map_while(Result::ok) {
                let parts: Vec<&str> = line.split(',').collect();

                // convert to an i64, matching all other system data structures
                // be sure to trim() first, otherwise the parse fails!
                let tmp_index: i64 = parts[1].trim().parse::<i64>().unwrap_or(constants::INVALID_OTHER_ID); 
                tmp_command_hashset.push( ( parts[0].to_string(), tmp_index ) ); // only first two items are actually used
            }
        }

        CommandController{
            command_hashset: tmp_command_hashset,
        }
    }
    
    ///
    /// Constructs a list of strings ( Vec<String> ) from the previously loaded command mapping file, and adds the user's prompt as the first element
    /// This required by the NL model we are using ATM, for its comparison routine.
    /// 
    pub fn get_all_operations_and_add_prompt (&self, prompt: String) -> Vec<String> {
        // take the list we loaded, cut it into separate vectors by the columns
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.unzip
        //
        let  (mut sentences, _command_id): (Vec::<String>, Vec::<i64>) = self.command_hashset.clone().into_iter().unzip();
        sentences.insert(0, prompt.clone() );

        return sentences;
    }

    ///
    /// Retrieves the id of the permission associated with the operation (column 0 from the command mapping)
    ///  that matches the prompt_string.
    /// 
    pub fn get_permission_for_operation (&self, prompt_string: String) -> i64 {
        //println!("get_permission_for_operation(): Compare to prompt: '{}'", prompt_string);
        let mut results: i64 = constants::INVALID_OTHER_ID;
        for item in self.command_hashset.clone().iter(){
           //println!("item.0='{}' item.1={}", item.0, item.1);
           if item.0 == prompt_string{
                results = item.1;
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