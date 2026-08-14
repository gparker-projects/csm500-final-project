/// -------------------------------------------------------------------
/// module for creating web (HTML) content
/// -------------------------------------------------------------------'
use std::fs;
//use std::ops::Add;
use derive_more::Display;
use std::collections::HashMap;

use crate::constants;
use crate::dto::{patient::*, encounter::*, intervention::*};

use crate::dao::patient_dao::PatientWrapper;

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
    #[display("Discharge Basic Tile")]
    WCTypeDischargeTile,
    #[display("Admit Basic Tile")]
    WCTypeAdmitTile,
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
        let mut contents = fs::read_to_string(&filename).expect(constants::ERROR_READING_TEMPLATE); 
        tiles.insert(WebContentItem::WCTypeLoginTile, contents ); 

        filename = content_root_path.to_owned() + "PatientListTile.htl";        
        //println!("2]WebContentFactory:new() : Attempting read of: {}", filename.clone());
        contents = fs::read_to_string(&filename).expect(constants::ERROR_READING_TEMPLATE);
        tiles.insert(WebContentItem::WCTypePatientListTile, contents ); 

        filename = content_root_path.to_owned() + "Workspace.htl";        
        contents = fs::read_to_string(&filename).expect(constants::ERROR_READING_TEMPLATE);
        tiles.insert(WebContentItem::WCTypeHomePage, contents ); 

        filename = content_root_path.to_owned() + "AdmitTile.htl";        
        contents = fs::read_to_string(&filename).expect(constants::ERROR_READING_TEMPLATE);
        tiles.insert(WebContentItem::WCTypeAdmitTile, contents ); 

        filename = content_root_path.to_owned() + "DischargeTile.htl";        
        contents = fs::read_to_string(&filename).expect(constants::ERROR_READING_TEMPLATE);
        tiles.insert(WebContentItem::WCTypeDischargeTile, contents ); 

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
    /// Obtains a specifically enumerated tile. This method does not require use of Options because we are
    /// keeping the key (tile_type: WebContentItem) tightly controlled at this point, so there is no risk
    /// of calling the method with an invalid (enumeration) entry.
    /// 
    pub fn get_tile(&self, tile_type: WebContentItem) -> String {
        return self.tile_hashmap[&tile_type].clone();
    }

    // -----------------------------------------------------------------------------------
    // Common formatters
    // -----------------------------------------------------------------------------------

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

    // -----------------------------------------------------------------------------------
    // Home tile formatters
    // -----------------------------------------------------------------------------------

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


    // -----------------------------------------------------------------------------------
    // Route formatters
    // -----------------------------------------------------------------------------------

    ///
    /// Provide rendering of a list of patients, as a screen tile
    /// 
    pub fn get_home_route_summary_of_patients_tile_using_wrapper(&self, patient_list: Vec<PatientWrapper>) -> String {
        let mut results_sbuf = String::with_capacity(100); 

        let mut counter: i8 = 1;

        results_sbuf.push_str(&self.get_hidden_form("patientdtls".to_owned(), "patientDtlsFrm".to_owned()) );

        for row in patient_list{

            let p = row.patient;
            let e = row.current_encounter;
            let i = row.most_recent_intervention;

            results_sbuf.push_str("<a href=\"#\" onclick=\"redirect_to_patient("  ); 
            results_sbuf.push_str( &p.id.to_string() ); 
            results_sbuf.push_str("); return false;\">");
            results_sbuf.push_str("<table class=\"hover-table\"><tr><td>"); 

            results_sbuf.push_str(&counter.to_string());
            results_sbuf.push_str(")&nbsp;<b>");
            results_sbuf.push_str( &p.legal_last_name ); 
            results_sbuf.push_str(","); 
            results_sbuf.push_str( &p.legal_first_name );

            results_sbuf.push_str("</b>&nbsp;PHN:<i>&nbsp;"); 
            results_sbuf.push_str( &p.phn_to_string() );
            results_sbuf.push_str("</i>&nbsp;");

            results_sbuf.push_str("&nbsp;<div class='clinical-electric-blue'>DOB:<b>&nbsp;"); 
            results_sbuf.push_str( &p.birth_date_for_display() );

            results_sbuf.push_str("</b></div>&nbsp;[");

            results_sbuf.push_str( &p.age() );
            results_sbuf.push_str("yrs]&nbsp;@");

            results_sbuf.push_str( &e.room_identifier );
            results_sbuf.push_str("</td></tr>");

            results_sbuf.push_str("<tr><td>");
            results_sbuf.push_str("Admitted: ");
            results_sbuf.push_str(&p.admit_timestamp_for_display() );
            results_sbuf.push_str("&nbsp;");
            results_sbuf.push_str( &i.intervention_type );
            results_sbuf.push_str("&nbsp;@&nbsp;");
            results_sbuf.push_str( &i.scheduled_date_for_display() );
            results_sbuf.push_str("&nbsp;(");
            results_sbuf.push_str( &i.status_code );
            results_sbuf.push_str(")</td></tr>");

            results_sbuf.push_str("<tr><td>");

            for m in row.intervention_detail{
                results_sbuf.push_str(&m.type_name());
                results_sbuf.push_str(":&nbsp;"); 
                results_sbuf.push_str(&m.value); 
                results_sbuf.push_str("&nbsp;"); 
            }
            results_sbuf.push_str("</td></tr>");
            results_sbuf.push_str("</table></a><p></p>");

            counter = counter + 1;
        }

        return results_sbuf;
    }

    // -----------------------------------------------------------------------------------
    // Patient formatters
    // -----------------------------------------------------------------------------------

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
    /// Provide HTML for creating a new patient admit, or completing it as a discharge for an existing patient
    /// It is the same table (Encounter), so one route should suffice
    /// 
    pub fn get_admit_discharge_tile(&self, user_identity_label: String, current_patient: Option<Patient>, legacy_menu: String) -> String {
        //let mut results_sbuf = String::with_capacity(100); 
        println!(">get_admit_discharge_tile()");

        let labels = ["<!--{patient_first_name}-->","<!--{patient_last_name}-->", "<!--{middle_name}-->",
                                  "<!--{phn}-->","<!--{birthdate}-->", "<!--{location}-->", "<!--{admit_notes}-->", 
                                  "<!--{temperature}-->","<!--{blood_pressure}-->", "<!--{weight}-->","<!--{intervention_notes}-->"];

        let base_tile_level_0 = self.get_tile(WebContentItem::WCTypeAdmitTile);
        let base_tile_level_1 = base_tile_level_0.replace(constants::LEGACY_MENU_TILE_TAG, &legacy_menu);

        // warning: match is on an Option<Patient>, NOT a tile
        let inner_content = match current_patient{ // get basic static tile loaded, make edits depending on type

            None =>{ // new patient (Admit) path
                println!("  Admit without Patient");

                // admitting a new patient with no data => wipe out the tags
                let mut result = base_tile_level_1.clone();
                for i in 0..labels.len() {
                    result = result.replace(labels[i], &"".to_string());
                }

                // ...except for admit_timestamp which will be Now()
                let result_2 = result.replace("<!--{admit_timestamp}-->", &chrono::Utc::now().format("%Y-%b-%d %H:%M:%S").to_string());

                result_2
            } 
            Some (p) => {
                println!("  Admit with existing Patient");
                let data_items = [&p.legal_first_name, &p.legal_last_name, &p.legal_middle_names,
                                                 &p.phn_to_string(), &p.birth_date_for_display(), &p.location_id.to_string(), &p.admit_notes, 
                                                 &"TODO".to_string(), &"TODO".to_string(), &"TODO".to_string(), &"TODO".to_string()];

                let mut result = base_tile_level_1.clone();
                for i in 0..labels.len() {
                    result = result.replace(labels[i], data_items[i]);
                }

                // ...except for admit_timestamp which will be Now()
                let result_2 = result.replace("<!--{admit_timestamp}-->", &p.admit_timestamp_for_display());

                result_2
            }
        };
        
        // this first one replaces the base tile (loaded from file) with the new "layout" provided above
        let home_tile_level_0 = &self.get_home_tile_with_user_identity(user_identity_label).replace(constants::BODY_TILE_CONTENT_TAG, &inner_content.clone()); // build the individual sections


        // common content
        let home_tile_level_1 = &home_tile_level_0.replace(constants::LEGACY_MENU_TILE_TAG, &legacy_menu);

        return home_tile_level_1.clone();
    }

   
    // -----------------------------------------------------------------------------------
    // Encounter formatters
    // -----------------------------------------------------------------------------------
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

    // -----------------------------------------------------------------------------------
    // Intervention formatters
    // -----------------------------------------------------------------------------------

    ///
    /// Provide HTML for modifying an Intervention
    /// 
    pub fn get_modify_intervention_tile(&self, current_intervention: Option<Intervention>) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        println!(">get_modify_intervention_tile()");

        let inner_content = match current_intervention{
            None =>{ // new intervention path
                println!("No Intervention provided");
                "No Encounters provided".to_owned()
            } // update intervention path
            Some (encounters) => {
                println!("Intervention provided");
                "Intervention provided".to_owned()
            }
        };

        results_sbuf.push_str(&self.get_hidden_form("admdis".to_owned(), "admdis".to_owned()) );

        let hidden_form = r##"<div id="hiddenSection" style="display: none; margin-top: 0px;">
                                      <form action="\{target_name}" method="post" id="{form_name}" name="{form_name}">
                                      <input type="hidden" name="target_id" id="target_id" value="0">
                                     </form></div>"##;

        results_sbuf.push_str(hidden_form);
        results_sbuf.push_str( &inner_content );        
        //results_sbuf.push_str("</table>");

        return results_sbuf;
    }

    // -----------------------------------------------------------------------------------
    // Menu formatters
    // -----------------------------------------------------------------------------------

    ///
    /// Provide HTML for the main system menu; replaces tag: <!--MapleEMR::LegacyMenu-->
    /// 
    pub fn get_legacy_menu(&self, patient_list: Vec<Patient>) -> String {
       return self.get_legacy_menu_with_patient(patient_list, constants::INVALID_PATIENT_ID);
    }

    ///
    /// Provide HTML for the main system menu; replaces tag: <!--MapleEMR::LegacyMenu-->
    /// 
    pub fn get_legacy_menu_with_patient(&self, patient_list: Vec<Patient>, patient_id: i64) -> String {
        let mut results_sbuf = String::with_capacity(100); 

        let template_sub_items = r#"<li><a class="menuNotCurrentSmall" href="javascript:selectPatientSub({id},1)">&nbsp;&nbsp;&nbsp;Medications</a></li>
                                    <li><a class="menuNotCurrentSmall" href="javascript:selectPatientSub({id},2)">&nbsp;&nbsp;&nbsp;Orders</a></li>
                                    <li<a class="menuNotCurrentSmall" href="javascript:selectPatientSub({id},3)">&nbsp;&nbsp;&nbsp;Allergies</a></li>
                                    "#;

        let admit_menu_item = r##"<form action="/admitnew" method="post" id="admitFrm" name="admitFrm"> <input type="hidden" id="target_id" name="target_id" value="-1"></form>"##;

        let mut first_entry: bool = true;

        //println!("> get_legacy_menu_with_patient({})", patient_id);

        results_sbuf.push_str("<div id=\"legacyMenu\" align=\"left\"><ul><li><a class=\"menuNotCurrent\" href=\"\\home\">My Dashboard</li>");
        for p in patient_list{

            // either we include ALL patients, OR we only include the current patient
            if patient_id == constants::INVALID_PATIENT_ID || p.id == patient_id{ 
                if ! first_entry {
                    results_sbuf.push_str("<li><a class=\"menuNotCurrent\" href=\"javascript:redirect_to_patient("); 
                }
                else{
                    results_sbuf.push_str("<li><a class=\"menuCurrent\" href=\"javascript:redirect_to_patient(");
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
        results_sbuf.push_str(admit_menu_item);
        results_sbuf.push_str("<li><a class=\"menuOther\" href=\"javascript:admit_patient()\">Admit New Patient</a></li>");
        results_sbuf.push_str("</ul></div>");

        return results_sbuf;
    }
}