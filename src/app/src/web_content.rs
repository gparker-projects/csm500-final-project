/// -------------------------------------------------------------------
/// module for creating web (HTML) content
/// -------------------------------------------------------------------'

use std::fs;
use std::io; //::{self, BufRead};
//use std::path::Path;
use derive_more::Display;
use std::collections::HashMap;

/// Enumeration for Web Content Tiles, each representing a tile of information
/// to be presented by the application
/// 
/// Ref: Practical Rust Projects, pg 181
/// 
#[derive(Display, Eq, Hash, PartialEq)]
pub enum WebContentTile {
    #[display("Login Tile")]
    WCTypeLoginTile,
    #[display("Patient List Tile")]
    WCTypePatientListTile,
    #[display("Patient Summary Tile")]
    WCTypePatientSummaryTile,
    #[display("Patient Detail Tile")]
    WCTypePatientDetailTile,
}



/// -------------------------------------------------------------------
/// Create a factory for generating web content
/// -------------------------------------------------------------------
pub struct WebContentFactory
{
    tile_hashmap: HashMap< WebContentTile, String>,
}

impl WebContentFactory {


    /// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach
    /// 
    pub fn new( content_root_path: &str) -> Self { //&self, 

      //  let content_files = ["logintile.wc.htm", "patientlist.wc.htm"];
      //  let content_enums = [WebContentTile::LoginTile, WebContentTile::PatientListTile];

        let mut tiles = HashMap::new();
        let mut filename = "LoginTile.rshtm";

        let mut contents = fs::read_to_string(filename).expect("Error while reading ");
        tiles.insert(WebContentTile::WCTypeLoginTile, contents ); 

        filename = "PatientListTile.rshtm";
        contents = fs::read_to_string(filename).expect("Error while reading ");
        tiles.insert(WebContentTile::WCTypePatientListTile, contents ); 

          WebContentFactory { tile_hashmap: tiles } 
    }

}