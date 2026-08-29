//! -------------------------------------------------------------------
//! Module for loading pre-existing web (HTML) content from .htl files
//!  (which are really just HTML with custom tags), as well as dynamically
//!  create some HTML.
//! 
//! The concept is that each page is a tile, potentially with subtiles.
//!  Following the MVC pattern, we are using data provided by the Controller (from the Model)
//!  to create a presentation of it (the View)
//! -------------------------------------------------------------------
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 
//! -------------------------------------------------------------------

use std::fs;
use derive_more::Display;
use std::collections::HashMap;
use tracing;

use crate::constants;
use crate::dto::{patient::*, intervention::*, intervention_detail::*};

use crate::ui::common::CommonFormatter;
use crate::ui::intervention_fmt::InterventionFormatter;
use crate::ui::simple_fmt::SimpleFormatter;

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
    #[display("Intervention-Detail Item Tile")]
    WCTypeIntvDetailItemTile,
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

        tracing::debug!(">WebContentFactory:new() : Attempting read of: {}", filename.clone());
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

        filename = content_root_path.to_owned() + "IntvDetailItemTile.htl";        
        contents = fs::read_to_string(&filename).expect(constants::ERROR_READING_TEMPLATE);
        tiles.insert(WebContentItem::WCTypeIntvDetailItemTile, contents ); 
            
        
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
    /// Provide (deep) summary details of a patient
    /// 
    pub fn get_patient_details_full_tile(&self, patient_header: String, current_encounter: String, encounter_section: String,
                                                user_identity_label: String, legacy_menu: String, intv_section: String, 
                                                intervention_type_list: Vec<(i64, String, String)>,
                                                patient_id: String, encounter_id: String) -> String {
        tracing::debug!(">get_patient_details_full_tile()");
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

        let ht9 = &ht8.replace(constants::INTERVENTION_TYPE_DROP_DOWN_CONTROL_TAG, &&CommonFormatter::get_dropdown_generic( intervention_type_list,
                                                                                                                                  "intervention_type_id".to_string(),
                                                                                                                                  constants::NOT_SPECIFIED_ID));
        let ht_final = &ht9.replace(constants::LEGACY_MENU_TILE_TAG, &legacy_menu);

        return ht_final.clone();
    }

    ///
    /// Provide HTML for creating a new patient admit, or completing it as a discharge for an existing patient
    /// It is the same table (Encounter), so one route should suffice
    /// 
    pub fn get_admit_discharge_full_tile(&self, user_identity_label: String, current_patient: Option<Patient>, legacy_menu: String, location_menu: String, is_discharge_flag: bool) -> String {
        tracing::debug!(">get_admit_discharge_tile()");

        let labels = ["{patient_first_name}","{patient_last_name}", "{middle_name}",
                                  "{phn}","{birthdate}", "{admit_notes}", "{encounter_id}", "{patient_id}"];

        let base_tile_level_1 = match is_discharge_flag{
            true => self.get_tile(WebContentItem::WCTypeDischargeTile),
            false => self.get_tile(WebContentItem::WCTypeAdmitTile),
        };

        // warning: match is on an Option<Patient>, NOT a tile
        let inner_content = match current_patient{ // get basic static tile loaded, make edits depending on type

            None =>{ // new patient (Admit) patH
                if is_discharge_flag {
                    tracing::debug!("  Discharge without Patient => INVALID");
                }
                else{
                    tracing::debug!("  Admit New Patient");
                }                

                // admitting a new patient with no data => wipe out the tags
                let mut result = base_tile_level_1.clone();
                for i in 0..labels.len() {
                    result = result.replace(labels[i], &"".to_string());
                }

                // ...except for admit_timestamp which will be Now()
                let result_2 = result.replace("{admit_timestamp}", &chrono::Utc::now().format("%Y-%b-%d %H:%M:%S").to_string());
                let result_3 = result_2.replace("{location_id}", &location_menu);

                result_3
            } 
            Some (p) => {
                if is_discharge_flag {
                    tracing::debug!("  Discharge existing Patient");
                }
                else{
                    tracing::debug!("  Admit update: existing Patient");
                }         
                let data_items = [&p.legal_first_name, &p.legal_last_name, &p.legal_middle_names,
                                                &p.phn_to_string(), &p.birth_date_for_display(),  &p.admit_notes,
                                                &p.encounter_id.to_string(),
                                                &p.id.to_string()];

                let mut result = base_tile_level_1.clone();
                for i in 0..labels.len() {
                    result = result.replace(labels[i], data_items[i]);
                }
               
                let result_2 = result.replace("{admit_timestamp}", &p.admit_timestamp_for_display());      

                if is_discharge_flag {
                    let result_3 = result_2.replace("{discharge_timestamp}", &chrono::Utc::now().format("%Y-%b-%d %H:%M:%S").to_string());
                    let result_4 = result_3.replace("{discharge_notes}", &p.discharge_notes);
                    let result_5 = result_4.replace("{location_short_name}", &p.location_short_name);
                    let result_6 = result_5.replace("{location_id}", &p.location_id.to_string());
                    result_6
                }
                else{
                    let result_3 = result_2.replace("{discharge_notes}", &p.discharge_notes);
                    let result_4 = result_3.replace("{location_id}", &location_menu);
                    result_4
                }
            }
        };
        
        // this first one replaces the base tile (loaded from file) with the new "layout" provided above
        let home_tile_level_0 = &self.get_home_tile_with_user_identity(user_identity_label).replace(constants::BODY_TILE_CONTENT_TAG, &inner_content.clone()); // build the individual sections
        let home_tile_level_1 = &home_tile_level_0.replace(constants::LEGACY_MENU_TILE_TAG, &legacy_menu);

        return home_tile_level_1.clone();
    }
   
   
    ///
    /// Provide HTML for modifying an Intervention
    /// 
    pub fn get_modify_intervention_full_tile(&self, user_identity_label: String,
                                                  current_intervention: Option<Intervention>,
                                                  legacy_menu: String,
                                                  user_dropdown_list: Vec<(i64, String, String)>,
                                                  status_dropdown_list: Vec<(i64, String, String)>,
                                                  location_menu: String,
                                                  intv_type: (i64, String, String),
                                                  patient_id: String,
                                                  intervention_type_id: String,
                                                  encounter_id: String,
                                                  intv_details_list: Option<Vec<InterventionDetail>>,
                                                  measures_dropdown_list: Vec<(i64, String, String)>,
                                                  ) -> String {
        tracing::debug!(">get_modify_intervention_full_page_tile()");

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
                                "{patient_id}",
                                "{form_errors}",
                                "<div id=\"MapleEMR::InterventionDetailsList\">"];

        let id: String; // this entire block is required in order to address partial moves that occur below when we copy over the String values
        let intervention_type: String;             // this must all occur before the copy and outside of the match block below
        let tmp_intervention_type_id: String;          // as the selection of Some()/None does not allow the move
        let mut scheduled_timestamp: String = chrono::Utc::now().format(constants::SYSTEM_DATETIME_FORMAT ).to_string();
        let mut performed_timestamp: String = chrono::Utc::now().format(constants::SYSTEM_DATETIME_FORMAT ).to_string();
        let dd_user: String;
        let dd_intv_status: String;
        //let location_id: String;
        let description: String;
        let notes: String;
        let tmp_encounter_id: String;

        let data_items = match current_intervention{
            None =>{ // new patient (Admit) path
                tracing::debug!("  Create new Intervention");
                dd_user = CommonFormatter::get_dropdown_user_with_department(user_dropdown_list,constants::NOT_SPECIFIED_ID); // "<div id=\"MapleEMR::UserIdDropDownControl\">",
                dd_intv_status =  CommonFormatter::get_dropdown_intervention_status(status_dropdown_list, constants::DEFAULT_INTERVENTION_STATUS_NEW); // "<div id=\"MapleEMR::StatusIdDropDownControl\">",

                let tmp_data_items = [constants::NOT_SPECIFIED_ID.to_string(), //"{intervention_id}",
                                                  intv_type.1, //"{intervention_type}",
                                                  intervention_type_id.to_string(), //"{intervention_type_id}",  //TODO
                                                  scheduled_timestamp, //"{scheduled_timestamp}",
                                                  performed_timestamp, // "{performed_timestamp}",
                                                  location_menu, //"{location_id}", 
                                                  dd_user, // "<div id=\"MapleEMR::UserIdDropDownControl\">",
                                                  dd_intv_status, // "<div id=\"MapleEMR::StatusIdDropDownControl\">",
                                                  "".to_string(), // "{description}",
                                                  "".to_string(), // "{notes}",
                                                  encounter_id.to_string(), // "{encounter_id}",  //TODO
                                                  patient_id.clone(),
                                                  "".to_string(), // "{form_errors}"];
                                                  "".to_string()
                                                  ];
                tmp_data_items
            } 
            Some (intv) => {
                tracing::debug!("  Update existing Intervention");
                dd_user = CommonFormatter::get_dropdown_user_with_department(user_dropdown_list,intv.users_id); // "<div id=\"MapleEMR::UserIdDropDownControl\">",
                dd_intv_status =  CommonFormatter::get_dropdown_intervention_status(status_dropdown_list, intv.status_id); // "<div id=\"MapleEMR::StatusIdDropDownControl\">",

                let intv_details_html =  InterventionFormatter::get_view_only_intervention_details_list( self.get_tile(WebContentItem::WCTypeIntvDetailItemTile), intv_details_list, measures_dropdown_list );

                id = intv.id.to_string();
                intervention_type = intv.clone().intervention_type; // intv_type.1
                tmp_intervention_type_id = intv.intervention_type_id.clone().to_string();
                scheduled_timestamp = intv.clone().scheduled_timestamp_for_display();
                performed_timestamp = intv.clone().performed_timestamp_for_display();
                //location_id = intv.location_id.to_string(); // location_menu below
                description = intv.description.to_string();
                notes = intv.notes.to_string();
                tmp_encounter_id = intv.encounter_id.to_string();

                let tmp_data_items = [id,
                                                    intervention_type,
                                                    tmp_intervention_type_id,
                                                    scheduled_timestamp,
                                                    performed_timestamp,
                                                    location_menu, //location_id 
                                                    dd_user, // "<div id=\"MapleEMR::UserIdDropDownControl\">",
                                                    dd_intv_status, // "<div id=\"MapleEMR::StatusIdDropDownControl\">",
                                                    description,
                                                    notes,
                                                    tmp_encounter_id,
                                                    patient_id.clone(),
                                                    "{form_errors}".to_string(),
                                                    intv_details_html
                                                ];
                tmp_data_items
            }
        };

        // replace all of the body tile contents
        let body_tile_level_0 = self.get_tile(WebContentItem::WCTypeInterventionFullPageTile);
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