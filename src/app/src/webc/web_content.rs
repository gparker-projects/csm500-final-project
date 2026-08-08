/// -------------------------------------------------------------------
/// module for creating web (HTML) content
/// -------------------------------------------------------------------'
use std::fs;
use derive_more::Display;
use std::collections::HashMap;

use crate::dto::patient::Patient;

/// Enumeration for Web Content Tiles, each representing a tile of information
/// to be presented by the application
/// 
/// Ref: Practical Rust Projects, pg 181
/// 
#[derive(Display, Eq, Hash, PartialEq)]
pub enum WebContentItem {
    #[display("Login Tile")]
    WCTypeLoginTile,
    #[display("Patient List Tile")]
    WCTypePatientListTile,
    #[display("Home")]
    WCTypeHomePage,
    //#[display("Patient Summary Tile")]
    //WCTypePatientSummaryTile,
    //#[display("Patient Detail Tile")]
    //WCTypePatientDetailTile,
}

/// -------------------------------------------------------------------
/// Create a factory for creating web content tiles and pages
/// -------------------------------------------------------------------
pub struct WebContentFactory
{
    tile_hashmap: HashMap< WebContentItem, String>,
}

impl WebContentFactory {
    ///
    /// Creates a new web content factory, which is capable of prodicing Web Content Tile objects for reuse/consumption in the main application
    /// This follows the object factory pattern: https://www.geeksforgeeks.org/system-design/factory-method-for-designing-pattern/
    /// 
    /// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach
    /// 
    pub fn new( content_root_path: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut filename = content_root_path.to_owned() + "LoginTile.htl";

        //println!("1]WebContentFactory:new() : Attempting read of: {}", filename.clone());
        let mut contents = fs::read_to_string(&filename).expect("Error reading tile template file");
        tiles.insert(WebContentItem::WCTypeLoginTile, contents ); 

        filename = content_root_path.to_owned() + "PatientListTile.htl";        
        //println!("2]WebContentFactory:new() : Attempting read of: {}", filename.clone());
        contents = fs::read_to_string(&filename).expect("Error reading tile template file");
        tiles.insert(WebContentItem::WCTypePatientListTile, contents ); 

        filename = content_root_path.to_owned() + "Workspace.htl";        
        contents = fs::read_to_string(&filename).expect("Error reading tile template file");
        tiles.insert(WebContentItem::WCTypeHomePage, contents ); 

        WebContentFactory { tile_hashmap: tiles } 
    }

    ///
    /// TEST only: returns the number of tiles that have been loaded into the factory.
    ///
    #[cfg(test)]
    pub fn get_tile_count(&self) -> usize {
        return self.tile_hashmap.len();
    }

    ///
    /// Wrapper method to return the main home page tile.
    /// 
    pub fn get_home_tile(&self) -> String {
        return self.tile_hashmap[&WebContentItem::WCTypeHomePage].clone();
    }

    ///
    /// Obtains a specifically enumerated tile. This method does not require use of Options because we are
    /// keeping the key (tile_type: WebContentItem) tightly controlled at this point, so there is no risk
    /// of calling the method with an invalid (enumeration) entry.
    /// 
    pub fn get_tile(&self, tile_type: WebContentItem) -> String {
        return self.tile_hashmap[&tile_type].clone();
    }

    ///
    /// Provide rendering of a list of patients, as a screen tile
    /// 
    pub fn get_patient_list_tile(&self, patient_list: Vec<Patient>) -> String {
        let mut results_sbuf = String::with_capacity(100); 

        results_sbuf.push_str("<div id=\"hiddenSection\" style=\"display: none; margin-top: 0px;\">");
        results_sbuf.push_str("<form action=\"\\patientdtls\" method=\"post\" id=\"patientDtlsFrm\" name=\"patientDtlsFrm\">");
        results_sbuf.push_str("<input type=\"hidden\" name=\"target_id\" id=\"target_id\" value=\"0\">");
        results_sbuf.push_str("</form></div>");

        results_sbuf.push_str("<table>");
        results_sbuf.push_str("  <tr><th>Last Name</th><th>First Name</th><th>SIN</th></tr>"); 

        for p in patient_list{
            results_sbuf.push_str("  <tr>");
            results_sbuf.push_str("<td><a href=\"#\" onclick=\"redirect_to_patient("  ); 
            results_sbuf.push_str( &p.id.to_string() ); 
            results_sbuf.push_str("); return false;\">"); 
            results_sbuf.push_str( &p.legal_last_name ); 
            results_sbuf.push_str("</a></td><td>"); 
            results_sbuf.push_str( &p.legal_first_name );
            results_sbuf.push_str("</td><td>"); 
            results_sbuf.push_str( &p.sin.to_string() ); 
            results_sbuf.push_str("</td>"); 
            results_sbuf.push_str("  </tr>\n");
        }
        results_sbuf.push_str("</table>");

        return results_sbuf;
    }

    ///
    /// Provide HTML for a single Patient
    /// 
    pub fn get_patient_details_tile(&self, p: Patient) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        results_sbuf.push_str("<H2>Patient Details Tile</H2>");

        results_sbuf.push_str("<table>");
        
        results_sbuf.push_str("  <tr><th class='data-label'>Last Name</th><td class='data-field-ro'>");
        results_sbuf.push_str(&p.legal_last_name );
        results_sbuf.push_str( "</td></tr>\n");

        results_sbuf.push_str("  <tr><th class='data-label'>First Name</th><td class='data-field-ro'>");
        results_sbuf.push_str(&p.legal_first_name );
        results_sbuf.push_str( "</td></tr>\n");

        results_sbuf.push_str("</table>");

        return results_sbuf;
    }

    ///
    /// Provide HTML for the main system menu; replaces tag: <!--MapleEMR::LegacyMenu-->
    /// 
    pub fn get_standard_menu(&self, patient_list: Vec<Patient>) -> String {
        let mut results_sbuf = String::with_capacity(100); 

        let template_sub_items = r#"<li><a class="menuNotCurrentSmall" href="javascript:selectPatientSub({id},1)">&nbsp;&nbsp;&nbsp;Medications</a></li>
                                    <li><a class="menuNotCurrentSmall" href="javascript:selectPatientSub({id},2)">&nbsp;&nbsp;&nbsp;Orders</a></li>
                                    <li><a class="menuNotCurrentSmall" href="javascript:selectPatientSub({id},3)">&nbsp;&nbsp;&nbsp;Allergies</a></li>
                                    "#;

        let mut first_entry: bool = true;

        results_sbuf.push_str("<div id=\"leftMenu\" align=\"left\"><ul>");
        for p in patient_list{
            if ! first_entry {
               results_sbuf.push_str("<li><a class=\"menuNotCurrent\"href=\"javascript:selectPatient("); 
            }
            else{
               results_sbuf.push_str("<li><a class=\"menuCurrent\" href=\"javascript:selectPatient(");
               first_entry = false;
            }
            results_sbuf.push_str( &p.id.to_string() ); 
            results_sbuf.push_str(")\">");
            results_sbuf.push_str( &p.legal_last_name ); 
            results_sbuf.push_str(",&nbsp;"); 
            results_sbuf.push_str( &p.legal_first_name );
            results_sbuf.push_str("</a></li>\n");

            let sub_menus = template_sub_items.replace("{id}", &p.id.to_string());  // replace default string       

            results_sbuf.push_str(&sub_menus);
        }
        results_sbuf.push_str("</ul></div>");

        return results_sbuf;
    }
}