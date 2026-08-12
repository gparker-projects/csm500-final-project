/// -------------------------------------------------------------------
/// module for creating web (HTML) content
/// -------------------------------------------------------------------'
use std::fs;
use derive_more::Display;
use std::collections::HashMap;

use crate::constants;
use crate::dto::encounter::*;
use crate::dto::patient::*;
use crate::dto::intervention::*;

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
    /// Wrapper method to return the main home page tile.
    /// 
    pub fn get_home_tile_with_user_identity(&self, user_identity_label: String) -> String {
        let results = self.tile_hashmap[&WebContentItem::WCTypeHomePage].clone();

        // add the user's identity
        return results.replace(constants::USER_IDENTITY_TILE_TAG, &user_identity_label)
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

        let mut counter: i8 = 1;

        results_sbuf.push_str(&self.get_hidden_form("patientdtls".to_owned(), "patientDtlsFrm".to_owned()) );

        for row in patient_list{
  
            results_sbuf.push_str("<a href=\"#\" onclick=\"redirect_to_patient("  ); 
            results_sbuf.push_str( &row.id.to_string() ); 
            results_sbuf.push_str("); return false;\"><table><tr><td>"); 

            results_sbuf.push_str(&counter.to_string());
            results_sbuf.push_str(")&nbsp;");
            results_sbuf.push_str( &row.legal_last_name ); 
            results_sbuf.push_str(","); 
            results_sbuf.push_str( &row.legal_first_name );
            results_sbuf.push_str("&nbsp;<b>DOB:&nbsp;"); 

            results_sbuf.push_str( &row.birth_date_for_display() );

            results_sbuf.push_str("</b>&nbsp;[");
            results_sbuf.push_str( &row.age() );
            results_sbuf.push_str("yrs]&nbsp;<i>@</i>");

            //results_sbuf.push_str( &row.short_location() );

            results_sbuf.push_str("&nbsp;&nbsp; Admitted: ");
            results_sbuf.push_str(&row.admit_timestamp_for_display() );
            results_sbuf.push_str("</td></tr>");

            results_sbuf.push_str("<tr><td>");
            results_sbuf.push_str("<i>FLAGS</i><br>");
            results_sbuf.push_str("</td></tr>");

            results_sbuf.push_str("<tr><td>");
            results_sbuf.push_str("<i>MEASURES</i><br>"); 
            results_sbuf.push_str("</td></tr>");
            results_sbuf.push_str("</table></a><p></p>");

            counter = counter + 1;
        }


        return results_sbuf;
    }

    ///
    /// Provide HTML for a single Patient
    /// 
    pub fn get_patient_details_tile(&self, p: Patient) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        println!(">get_patient_details_tile()");

        // build the header
        results_sbuf.push_str("<table>");

        results_sbuf.push_str("  <tr><th class='data-label'>Last, First Name</th><td class='data-field-ro'>");
        results_sbuf.push_str(&p.legal_last_name );
        results_sbuf.push_str( "</td></tr>\n");

        results_sbuf.push_str("  <tr><th class='data-label'>First Name</th><td class='data-field-ro'>");
        results_sbuf.push_str(&p.legal_first_name );
        results_sbuf.push_str( "</td></tr>\n");

        results_sbuf.push_str("</table>");

        return results_sbuf;
    }

    ///
    /// Formats the identity of the user, for replacement of the constants::USER_IDENTITY_TILE_TAG tag
    /// 
    pub fn get_user_identity_label(&self,user_display_name: String) -> String{
        "<div class=\"userIdentification\"id=\"userIdentityLbl\"><b>".to_owned() + &user_display_name + "</b></div>"
    }

    ///
    /// Provide full details of a patient
    /// 
    /// 
    pub fn get_patient_details_full_tile(&self, patient_header: String, current_encounter: String, encounter_section: String,
                                                user_identity_label: String, legacy_menu: String, intv_section: String) -> String {

        let layout = r##"<h3>Patient Header</h3><p>
                               <div id="MapleEMR::PatientHeader"></div><p></p>
                               <h3>Current Encounter</h3>
                               <div id="MapleEMR::CurrentEncounter"></div><p></p>
                               <h3>Current Interventions</h3>
                               <div id="MapleEMR::CurrentInterventions"></div><p></p>
                               <h3>Encounter History</h3>
                               <div id="MapleEMR::EncounterHistory"></div>"##; // this one is not a constant as it only appears in this function

        // base content
        let ht2 = &self.get_home_tile_with_user_identity(user_identity_label).replace(constants::BODY_TILE_CONTENT_TAG, layout); // build the individual sections

        // page body content
        let ht3 = &ht2.replace(constants::PATIENT_HEADER_TILE_TAG, &patient_header);
        let ht4 = &ht3.replace(constants::CURRENT_ENCOUNTER_TILE_TAG, &current_encounter);
        let ht5 = &ht4.replace(constants::CURRENT_INTERVENTIONS_TILE_TAG, &intv_section); // "CURRENT_INTERVENTIONS-REPLACED"); // 
        let ht6 = &ht5.replace(constants::ENCOUNTER_HISTORY_TILE_TAG, &encounter_section);

        // common content
        let ht7 = &ht6.replace(constants::LEGACY_MENU_TILE_TAG, &legacy_menu);

        return ht7.clone();
    }

    ///
    /// Returns a hidden form, used as a technique in several of the list tiles to submit a value for another screen
    /// 
    fn get_hidden_form(&self, target_name: String, form_name: String) -> String {
        let body = r##"<div id="hiddenSection" style="display: none; margin-top: 0px;">
                               <form action="\{target_name}" method="post" id="{form_name}" name="{form_name}">
                               <input type="hidden" name="target_id" id="target_id" value="0">
                             </form></div>"##;

        body.replace("{form_name}", &form_name).replace("{target_name}", &target_name)
    }
   
    pub fn get_single_encounter_summary_tile(&self, encounter: Encounter) -> String {
        let mut results_sbuf = String::with_capacity(100);

        results_sbuf.push_str("<table <tr><th>Admit Date</th><th>Site/Facility</th></tr>"); 

                results_sbuf.push_str("  <tr>");
        results_sbuf.push_str("<td><a href=\"#\" onclick=\"redirect_to_enc("  ); 
        results_sbuf.push_str( &encounter.to_string() ); 
        results_sbuf.push_str("); return false;\">"); 
        results_sbuf.push_str( &encounter.admit_timestamp_for_display()); 
        results_sbuf.push_str("</a></td><td>"); 
        results_sbuf.push_str( &encounter.encounter_site_name );
        results_sbuf.push_str("</td>"); 
        results_sbuf.push_str("  </tr>\n");
        
        results_sbuf.push_str("</table>");

        return results_sbuf;
    }

    ///
    /// Provide HTML for all of a Patient's encounters
    /// 
    pub fn get_encounter_list_tile(&self, encounter_list: Vec<Encounter>) -> String {
        let mut results_sbuf = String::with_capacity(100); 

        //println!(">get_encounter_list_tile()");

        results_sbuf.push_str(&self.get_hidden_form("encounterDtls".to_owned(), "encounterDtlsFrm".to_owned()) );

        results_sbuf.push_str("<table <tr><th>Admit Date</th><th>Site/Facility</th></tr>"); 

        for row in encounter_list{
            results_sbuf.push_str("  <tr>");
            results_sbuf.push_str("<td><a href=\"#\" onclick=\"redirect_to_enc("  ); 
            results_sbuf.push_str( &row.id.to_string() ); 
            results_sbuf.push_str("); return false;\">"); 
            results_sbuf.push_str( &row.admit_timestamp_for_display()); 
            results_sbuf.push_str("</a></td><td>"); 
            results_sbuf.push_str( &row.encounter_site_name );
            results_sbuf.push_str("</td>"); 
            results_sbuf.push_str("  </tr>\n");
        }
        results_sbuf.push_str("</table>");

        return results_sbuf;
    }

    ///
    /// Provide HTML for all of a (Patient's) Encounter's Interventions
    /// 
    pub fn get_intervention_list_tile(&self, intervention_list: Vec<Intervention>) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        println!(">get_intervention_list_tile()");

        results_sbuf.push_str(&self.get_hidden_form("intvDtls".to_owned(), "intvDtlsFrm".to_owned()) );

        results_sbuf.push_str("<table <tr><th>Description</th><th>Date Performed</th><th>Date Scheduled</th><th>State</th></tr>"); 

        for row in intervention_list{
            results_sbuf.push_str("  <tr>");
            results_sbuf.push_str("<td><a href=\"#\" onclick=\"redirect_to_enc("  ); 
            results_sbuf.push_str( &row.id.to_string() ); 
            results_sbuf.push_str("); return false;\">"); 
            results_sbuf.push_str( &row.type_description_for_display()); 
            results_sbuf.push_str("</a></td><td>"); 
            results_sbuf.push_str( &row.performed_date_for_display() );
            results_sbuf.push_str("</td>"); 
            results_sbuf.push_str("<td>"); 
            results_sbuf.push_str( &row.scheduled_date_for_display() );
            results_sbuf.push_str("</td>"); 
            results_sbuf.push_str("  </tr>\n");
        }
        results_sbuf.push_str("</table>");

        return results_sbuf;
    }

    ///
    /// Provide HTML for the main system menu; replaces tag: <!--MapleEMR::LegacyMenu-->
    /// 
    pub fn get_standard_menu(&self, patient_list: Vec<Patient>) -> String {
       return self.get_standard_menu_with_patient(patient_list, constants::INVALID_PATIENT_ID);
    }

    ///
    /// Provide HTML for the main system menu; replaces tag: <!--MapleEMR::LegacyMenu-->
    /// 
    pub fn get_standard_menu_with_patient(&self, patient_list: Vec<Patient>, patient_id: i64) -> String {
        let mut results_sbuf = String::with_capacity(100); 

        let template_sub_items = r#"<li><a class="menuNotCurrentSmall" href="javascript:selectPatientSub({id},1)">&nbsp;&nbsp;&nbsp;Medications</a></li>
                                    <li><a class="menuNotCurrentSmall" href="javascript:selectPatientSub({id},2)">&nbsp;&nbsp;&nbsp;Orders</a></li>
                                    <li><a class="menuNotCurrentSmall" href="javascript:selectPatientSub({id},3)">&nbsp;&nbsp;&nbsp;Allergies</a></li>
                                    "#;

        let mut first_entry: bool = true;

        print!("> get_standard_menu_with_patient({})", patient_id);

        results_sbuf.push_str("<div id=\"legacyMenu\" align=\"left\"><ul><li><a class=\"menuNotCurrent\" href=\"\\home\">My Dashboard</li>");
        for p in patient_list{

            // either we include ALL patients, OR we only include the current patient
            if patient_id == constants::INVALID_PATIENT_ID || p.id == patient_id{ 
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

                if p.id == patient_id {
                    let sub_menus = template_sub_items.replace("{id}", &p.id.to_string());  // replace default string       

                    results_sbuf.push_str(&sub_menus);
                }
            }
        }
        results_sbuf.push_str("</ul></div>");

        return results_sbuf;
    }
}