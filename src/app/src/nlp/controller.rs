
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//use crate::constants;

pub const COMMAND_MAPPING_FILE_NAME: &str = "command_mapping.csv";

// Commands that can be executed via the NLE
pub const COMMAND_ADMIT_NEW_PATIENT: i64 = 1;
pub const COMMAND_ADD_NEW_INTERVENTION: i64 = 2;
pub const COMMAND_DISCHARGE_PATIENT: i64 = 3;

///
/// Provides logic and constraints around commands being elecuted by the NLP module
/// 
pub struct CommandController{
    command_hashset: Vec<(String, String)>, // first 
}

impl CommandController{

    pub fn get_all_operations_and_add_prompt (&self, prompt: String) -> Vec<String> {
        //let mut results = Vec::<String>::with_capacity(10);

        println!("CommandController::get_all_operations_and_add_prompt()");

        // take the list we loaded, cut it into separate vectors by the columns
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.unzip
        //
        let  (mut sentences, _command_id): (Vec::<String>, Vec::<String>) = self.command_hashset.clone().into_iter().unzip();
        sentences.insert(0, prompt.clone() );

        return sentences;
    }

    pub fn get_permission_for_operation (&self, prmpt_id: String) -> i64 {
    
      //https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples/iter_find.html
         let idx = self.command_hashset.iter().find(|(p2, _) | *p2 == prmpt_id) ;
         match idx {
            Some(item) => item.1.parse::<i64>().unwrap_or(-1),
            None => -1
         }
    }


    pub fn new(content_root_path: &str) -> Self {
        let mut tmp_command_hashset = Vec::<(String, String)>::with_capacity(10);
        let filename = Path::new( content_root_path )
                                              .join("data")
                                              .join(COMMAND_MAPPING_FILE_NAME).to_string_lossy().to_string();

        println!( "CommandController::New() {}", filename );

        // read in the command mapping config .CSV
        if let Ok(lines) = CommandController::read_lines( filename ) {
            for line in lines.map_while(Result::ok) {
                let parts: Vec<&str> = line.split(',').collect();
                tmp_command_hashset.push( (parts[0].to_string(), parts[1].to_string()) ); // only first two items are actually used
                //println!( "..Loaded: {}, {}", parts[0].to_string(), parts[1].to_string() );
            }
        }

        CommandController{
            command_hashset: tmp_command_hashset,
        }
    }


    // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
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