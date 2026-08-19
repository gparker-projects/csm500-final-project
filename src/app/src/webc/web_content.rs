/// -------------------------------------------------------------------
/// module for creating web (HTML) content
/// -------------------------------------------------------------------'
use std::fs;
//use std::ops::Add;
use derive_more::Display;
use std::collections::HashMap;

use crate::constants;
use crate::dto::{patient::*, encounter::*, intervention::*};
//use crate::webc::menu_tile::MenuFormatter;
use crate::dao::patient_dao::PatientWrapper;

/// Enumeration for Web Content Tiles, each representing a tile of information
/// to be presented by the application
/// 
/// Ref: Practical Rust Projects, pg 181
/// 
#[derive(Clone, Display, Eq, Hash, PartialEq)]
pub enum WebContentItem {
    #[display("Login Full Page Tile")]
    WCTypeLoginTile,
    #[display("Patient Full Page Tile")]
    WCTypePatientListTile,
    #[display("Home Full Page Tile")]
    WCTypeHomePage,
    #[display("Discharge Full Page Tile")]
    WCTypeDischargeTile,
    #[display("Admit Full Page Tile")]
    WCTypeAdmitTile,
    #[display("Intervention Full Page Tile")]
    WCTypeInterventionFullPageTile,
}

/// -------------------------------------------------------------------
/// Create a factory for creating web content tiles and pages
/// -------------------------------------------------------------------
#[derive(Clone)]
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

        filename = content_root_path.to_owned() + "InterventionTile.htl";        
        contents = fs::read_to_string(&filename).expect(constants::ERROR_READING_TEMPLATE);
        tiles.insert(WebContentItem::WCTypeInterventionFullPageTile, contents ); 
        
        WebContentFactory { tile_hashmap: tiles } 
    }

    ///
    /// TEST only: returns the number of tiles that have been loaded into the factory.
    ///
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

        for pwrap in patient_list{
            results_sbuf.push_str("<a href=\"#\" onclick=\"redirect_to_patient("  ); 
            results_sbuf.push_str( &pwrap.patient.id.to_string() ); 
            results_sbuf.push_str("); return false;\">");
            results_sbuf.push_str(&self.get_single_patient_summary( pwrap, counter));
            results_sbuf.push_str("</a><p></p>");
            counter = counter + 1;
        }

        return results_sbuf;
    }

    // -----------------------------------------------------------------------------------
    // Patient formatters
    // -----------------------------------------------------------------------------------
    pub fn get_single_patient_summary(&self, pwrap: PatientWrapper, index: i8) -> String {
        let mut results_sbuf = String::with_capacity(500); 
	    let p = pwrap.patient;
	    let e = pwrap.current_encounter;
	    let i = pwrap.most_recent_intervention;

	    results_sbuf.push_str("<table class=\"hover-table\"><tr><td>"); 

        if index != -1 {
            let idx = index.to_string();
            results_sbuf.push_str(&idx);
            results_sbuf.push_str(")&nbsp;");
        }
	    results_sbuf.push_str("<b>");
	    results_sbuf.push_str( &p.legal_last_name ); 
	    results_sbuf.push_str(", "); 
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

	    if i.is_some() {
	       let tmp_intv = i.unwrap();

	       results_sbuf.push_str("&nbsp;");
	       results_sbuf.push_str( &tmp_intv.intervention_type );
	       results_sbuf.push_str("&nbsp;@&nbsp;");
	       results_sbuf.push_str( &tmp_intv.scheduled_timestamp_for_display() );

	       results_sbuf.push_str("&nbsp;(");
	       results_sbuf.push_str( &tmp_intv.status_code );
	       results_sbuf.push_str(")");
	    }
	    results_sbuf.push_str("</td></tr>");

	    results_sbuf.push_str("</table>");

        return results_sbuf;
    }

    ///
    /// Provide (deep) summary details of a patient
    /// 
    pub fn get_patient_details_full_tile(&self, patient_header: String, current_encounter: String, encounter_section: String,
                                                user_identity_label: String, legacy_menu: String, intv_section: String, 
                                                intervention_type_list: Vec<(i64, String, String)>,
                                                patient_id: String, encounter_id: String) -> String {
        let layout = self.get_tile(WebContentItem::WCTypePatientListTile);

        // base content
        let ht2 = &self.get_home_tile_with_user_identity(user_identity_label).replace(constants::BODY_TILE_CONTENT_TAG, &layout); // build the individual sections

        // page body content
        let ht3 = &ht2.replace(constants::PATIENT_HEADER_TILE_TAG, &patient_header);
        let ht4 = &ht3.replace(constants::CURRENT_ENCOUNTER_TILE_TAG, &current_encounter);
        let ht5 = &ht4.replace(constants::CURRENT_INTERVENTIONS_TILE_TAG, &intv_section);
        let ht6 = &ht5.replace(constants::ENCOUNTER_HISTORY_TILE_TAG, &encounter_section);

        let ht7 = &ht6.replace("{patient_id}",  &patient_id);
        let ht8 = &ht7.replace("{encounter_id}",  &encounter_id);

        let ht9 = &ht8.replace(constants::INTERVENTION_TYPE_DROP_DOWN_CONTROL_TAG, &&self.get_dropdown_generic( intervention_type_list,
                                                                                                                                  "intervention_type_id".to_string(),
                                                                                                                                  constants::NOT_SPECIFIED_ID));
        let ht_final = &ht9.replace(constants::LEGACY_MENU_TILE_TAG, &legacy_menu);

        return ht_final.clone();
    }

    ///
    /// Provide HTML for creating a new patient admit, or completing it as a discharge for an existing patient
    /// It is the same table (Encounter), so one route should suffice
    /// 
    pub fn get_admit_discharge_tile(&self, user_identity_label: String, current_patient: Option<Patient>, legacy_menu: String, location_menu: String) -> String {
        println!(">get_admit_discharge_tile()");

        let labels = ["<!--{patient_first_name}-->","<!--{patient_last_name}-->", "<!--{middle_name}-->",
                                  "<!--{phn}-->","<!--{birthdate}-->", "<!--{admit_notes}-->"];

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
                                                 &p.phn_to_string(), &p.birth_date_for_display(),  &p.admit_notes];

                let mut result = base_tile_level_1.clone();
                for i in 0..labels.len() {
                    result = result.replace(labels[i], data_items[i]);
                }

                // ...except for admit_timestamp which will always be Now()
                let result_2 = result.replace("<!--{admit_timestamp}-->", &p.admit_timestamp_for_display());                
                result_2
            }
        };

        let inner_content_1 = inner_content.replace("<!--{location_id}-->", &location_menu); // &p.admit_timestamp_for_display());
        
        // this first one replaces the base tile (loaded from file) with the new "layout" provided above
        let home_tile_level_0 = &self.get_home_tile_with_user_identity(user_identity_label).replace(constants::BODY_TILE_CONTENT_TAG, &inner_content_1.clone()); // build the individual sections

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
    /// Generates a list of locations based on what is in the system
    /// 
    pub fn get_dropdown_generic(&self, item_list: Vec<(i64, String, String)>, list_name_and_id: String, default_item_id: i64) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        println!("> get_location_dropdown()");

        // https://www.w3schools.com/tags/tag_select.asp
        results_sbuf.push_str("<select name='");
        results_sbuf.push_str(&list_name_and_id.to_string());
        results_sbuf.push_str("' id='");
        results_sbuf.push_str(&list_name_and_id.to_string());
        results_sbuf.push_str("'>");

        for row in item_list{
            results_sbuf.push_str("<option value='" );
            results_sbuf.push_str(&row.0.to_string()); // location_id here
            results_sbuf.push_str("'" );
            if row.0 == default_item_id {
                results_sbuf.push_str(" selected ");
            }
            results_sbuf.push_str(">");
            results_sbuf.push_str(&row.1); // description here
            results_sbuf.push_str("</option>");
        }
        results_sbuf.push_str("</select>");

        return results_sbuf;
    }


    ///
    /// Generates a dropdown for users
    /// 
    pub fn get_dropdown_user_with_department(&self, item_list: Vec<(i64, String, String)>, default_item_id: i64) -> String {
        self.get_dropdown_generic(item_list, "user_id".to_string(), default_item_id)
    }

    pub fn get_dropdown_intervention_status(&self, item_list: Vec<(i64, String, String)>, default_item_id: i64) -> String {
        return self.get_dropdown_generic(item_list, "intervention_id".to_string(), default_item_id);
    }

    ///
    /// Generates a list of locations based on what is in the system
    /// 
    pub fn get_location_dropdown(&self, location_list: Vec<(i64, String)>, default_location_id: i64) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        println!("> get_location_dropdown()");

        // https://www.w3schools.com/tags/tag_select.asp
        results_sbuf.push_str("<select name='location_id' id='location_id'>");

        for row in location_list{
            results_sbuf.push_str("<option value='" );
            results_sbuf.push_str(&row.0.to_string()); // location_id here
            results_sbuf.push_str("'" );
            if row.0 == default_location_id {
                results_sbuf.push_str(" selected ");
            }
            results_sbuf.push_str(">");
            results_sbuf.push_str(&row.1); // description here
            results_sbuf.push_str("</option>");
        }
        results_sbuf.push_str("</table>");

        return results_sbuf;
    }

    ///
    /// Provide HTML for all of a (Patient's) Encounter's Interventions
    /// 
    pub fn get_intervention_list_for_patient_details_tile(&self, intervention_list: Vec<Intervention>) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        println!(">get_intervention_list_tile()");

        results_sbuf.push_str(&self.get_hidden_form("intvDtls".to_owned(), "intvDtlsFrm".to_owned()) );

        results_sbuf.push_str("<table <tr><th>Description</th><th>Date Performed</th><th>Date Scheduled</th><th>State</th></tr>"); 

        for row in intervention_list{
            results_sbuf.push_str("  <tr>");
            results_sbuf.push_str("<td><a href=\"#\" onclick=\"redirect_to_intv("  ); 
            results_sbuf.push_str( &row.id.to_string() ); 
            results_sbuf.push_str("); return false;\">"); 
            results_sbuf.push_str( &row.type_description_for_display()); 
            results_sbuf.push_str("</a></td><td>"); 
            results_sbuf.push_str( &row.performed_timestamp_for_display() );
            results_sbuf.push_str("</td>"); 
            results_sbuf.push_str("<td>"); 
            results_sbuf.push_str( &row.scheduled_timestamp_for_display() );
            results_sbuf.push_str("</td>"); 
            results_sbuf.push_str("<td>"); 
            results_sbuf.push_str( &row.status_for_display() );
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
    pub fn get_modify_intervention_full_page_tile(&self, user_identity_label: String, current_intervention: Option<Intervention>,
                                                  legacy_menu: String,
                                                  user_dropdown_list: Vec<(i64, String, String)>,
                                                  status_dropdown_list: Vec<(i64, String, String)>
                                                  ) -> String {
        println!(">get_modify_intervention_tile()");

        let tags = ["{intervention_id}",
                                "{intervention_type}",
                                "{intervention_type_id}",
                                "{scheduled_timestamp}",
                                "{performed_timestamp}",
                                "{location_id}",
                                "<div id=\"MapleEMR::UserIdDropDownControl\">",
                                "<div id=\"MapleEMR::StatusIdDropDownControl\">",
                                "{description}",
                                "{notes}",
                                "{encounter_id}",
                                "{form_errors}"];

        let scheduled_time = chrono::Utc::now().format("%Y-%b-%d %H:%M:%S").to_string();
        let performed_time = chrono::Utc::now().format("%Y-%b-%d %H:%M:%S").to_string();

        let id: String; // this entire block is required in order to address partial moves that occur below when we copy over the String values
        let intervention_type: String;             // this must all occur before the copy and outside of the match block below
        let intervention_type_id: String;          // as the selection of Some()/None does not allow the move
        let scheduled_timestamp: String;
        let performed_timestamp: String;
        let dd_user: String;
        let dd_intv_status: String;
        let location_id: String;
        let description: String;
        let notes: String;
        let encounter_id: String;

        let data_items = match current_intervention{
            None =>{ // new patient (Admit) path
                println!("  Create new Intervention");
                dd_user = self.get_dropdown_user_with_department(user_dropdown_list,constants::NOT_SPECIFIED_ID); // "<div id=\"MapleEMR::UserIdDropDownControl\">",
                dd_intv_status =  self.get_dropdown_intervention_status(status_dropdown_list, constants::NOT_SPECIFIED_ID); // "<div id=\"MapleEMR::StatusIdDropDownControl\">",

                let tmp_data_items: [String; 12] = ["".to_string(), //"{intervention_id}",
                                                  "".to_string(), //"{intervention_type}",
                                                  "".to_string(), //"{intervention_type_id}",
                                                  scheduled_time, //"{scheduled_timestamp}",
                                                  performed_time, // "{performed_timestamp}",
                                                  "".to_string(), //"{location_id}",
                                                  dd_user, // "<div id=\"MapleEMR::UserIdDropDownControl\">",
                                                  dd_intv_status, // "<div id=\"MapleEMR::StatusIdDropDownControl\">",
                                                  "".to_string(), // "{description}",
                                                  "".to_string(), // "{notes}",
                                                  "".to_string(), // "{encounter_id}",
                                                  "".to_string() // "{form_errors}"];
                                                  ];
                tmp_data_items
            } 
            Some (intv) => {
                println!("  Update existing Intervention");
                dd_user = self.get_dropdown_user_with_department(user_dropdown_list,intv.users_id); // "<div id=\"MapleEMR::UserIdDropDownControl\">",
                dd_intv_status =  self.get_dropdown_intervention_status(status_dropdown_list, intv.status_id); // "<div id=\"MapleEMR::StatusIdDropDownControl\">",

                id = intv.id.to_string();
                intervention_type = intv.clone().intervention_type;
                intervention_type_id = intv.intervention_type_id.clone().to_string();
                scheduled_timestamp = intv.clone().scheduled_timestamp_for_display();
                performed_timestamp = intv.clone().performed_timestamp_for_display();
                location_id = intv.location_id.to_string();
                description = intv.intervention_type_id.to_string();
                notes = intv.intervention_type_id.to_string();
                encounter_id = intv.intervention_type_id.to_string();

                let tmp_data_items = [id,
                                                    intervention_type,
                                                    intervention_type_id,
                                                    scheduled_timestamp,
                                                    performed_timestamp,
                                                    location_id,
                                                    dd_user, // "<div id=\"MapleEMR::UserIdDropDownControl\">",
                                                    dd_intv_status, // "<div id=\"MapleEMR::StatusIdDropDownControl\">",
                                                    description,
                                                    notes,
                                                    encounter_id,
                                                    "{form_errors}".to_string()
                                                ];
                tmp_data_items
            }
        };

        // replace all of the body tile contents
        let body_tile_level_0 = self.get_tile(WebContentItem::WCTypeAdmitTile);
        let mut body_tile_level_1 = body_tile_level_0.clone();
        for i in 0..tags.len() {
            body_tile_level_1 = body_tile_level_1.replace(tags[i], &data_items[i]);
        }
        
        let body_tile_level_2 = body_tile_level_1.replace(constants::LEGACY_MENU_TILE_TAG, &legacy_menu);
        
        let home_tile_level_0 = &self.get_home_tile_with_user_identity(user_identity_label).replace(constants::BODY_TILE_CONTENT_TAG, &body_tile_level_2.clone()); // build the individual sections
        let home_tile_level_final = &home_tile_level_0.replace(constants::LEGACY_MENU_TILE_TAG, &legacy_menu);

        return home_tile_level_final.clone();
    }
}