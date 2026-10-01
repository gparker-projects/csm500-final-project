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
//!  CSM500 Project (April - October 2026)
//!  Graham Parker (Student ID: 240120522)
//! -------------------------------------------------------------------

use std::fs;
use derive_more::Display;
use std::collections::HashMap;
use tracing;

use crate::constants;
use crate::dto::user_auth::Permission;
use crate::dto::{patient::*, intervention::*, intervention_detail::*};
use crate::session::UserSession;
use crate::ui::data_forms::*;
use crate::ui::common_fmt::CommonFormatter;
use crate::ui::intervention_fmt::InterventionFormatter;

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
/// WebContentFactory is an object factory for creating web content tiles and pages
/// -------------------------------------------------------------------
#[derive(Clone)]
pub struct WebContentFactory
{
    tile_hashmap: HashMap< WebContentItem, String>,
}

impl WebContentFactory {

    /// ### WebContentFactory::new()
    ///   Creates a new web content factory, which is capable of prodicing Web Content Tile objects for reuse/consumption in the main application
    /// 
    ///   This follows the object factory pattern.
    /// 
    /// #### Parameters:
    /// * content_root_path: &str - the root path to the tile files (*.htl) that are to be loaded. Each file it associated with a WebContentItem enumeration entry.
    /// * app_version: String - version of the application to be inserted into calls to create HTMl tile content
    /// 
    /// #### References:
    ///   https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach
    ///   https://www.geeksforgeeks.org/system-design/factory-method-for-designing-pattern/
    /// 
    /// #### Returns:
    /// * a referenge to this WebContentFactory instance
    ///
    pub fn new( content_root_path: &str, app_version: String) -> Self {
        let mut tiles = HashMap::new();
        let tile_files = [("LoginTile.htl", WebContentItem::WCTypeLoginTile),
                                ("PatientListTile.htl", WebContentItem::WCTypePatientListTile),
                                ("Home.htl", WebContentItem::WCTypeHomePage),
                                ("AdmitTile.htl", WebContentItem::WCTypeAdmitTile),
                                ("DischargeTile.htl", WebContentItem::WCTypeDischargeTile), 
                                ("InterventionTile.htl", WebContentItem::WCTypeInterventionFullPageTile),
                                ("IntvDetailItemTile.htl", WebContentItem::WCTypeIntvDetailItemTile)       ];

        tracing::debug!("WebContentFactory:new()");
        //println!("WebContentFactory:new()");
        // load tiles from pre-defined files, assigning to known constants so that the application can reliably load them later
        for item in tile_files{
           tracing::debug!("..Load tile from: {}", item.0);
           //println!("..Load tile from: {}", item.0);
           let tmp_content = fs::read_to_string(&( content_root_path.to_owned() + item.0) ).expect(constants::ERROR_READING_TEMPLATE);

           tiles.insert(item.1, tmp_content.replace(constants::RELEASE_NUMBER, &app_version)); 
        }      
        WebContentFactory { tile_hashmap: tiles } 
    }

    ///
    /// TEST only: returns the number of tiles that have been loaded into the factory.
    /// Actually used, not dead_code. Cargo Check keeps flagging as it is only used in a TEST case
    ///
    #[allow(dead_code)]
    pub fn get_tile_count(&self) -> usize {
        return self.tile_hashmap.len();
    }


    /// ### WebContentFactory::get_tile()
    /// Obtains a specifically enumerated tile. This method does not require use of Options because we are
    /// keeping the key (tile_type: WebContentItem) tightly controlled at this point, so there is no risk
    /// of calling the method with an invalid (enumeration) entry.
    /// 
    /// #### Parameters:
    /// * tile_type: WebContentItem - enumeration element representing the Tile to be retrieved
    /// 
    /// #### Returns:
    /// * String: the resulting home tile HTML to be rendered
    /// 
    pub fn get_tile(&self, tile_type: WebContentItem) -> String {
        return self.tile_hashmap[&tile_type].clone();
    }

    /// ### WebContentFactory::get_home_tile_with_user_identity()
    ///   Wrapper method to return the main home page tile.
    /// 
    /// #### Parameters:
    /// * user_identity_label: String - the current user's identity string to be displayed
    /// 
    /// #### Returns:
    /// * String: the resulting home tile HTML to be rendered
    /// 
    pub fn get_home_tile_with_user_identity(&self, user_identity_label: String) -> String {
        let results = self.tile_hashmap[&WebContentItem::WCTypeHomePage].clone();

        // add the user's identity
        return results.replace(constants::USER_IDENTITY_TILE_TAG, &user_identity_label)
    }
  
    /// ### WebContentFactory::get_patient_details_full_tile()
    ///   Provide (deep) summary details of a patient
    /// 
    /// #### Parameters:
    /// * patient_header: String - HTML for the patient header section, to be inserted into the tile
    /// * user_identity_label: String - the current user's identity string to be displayed
    /// * current_encounter: String - HTML for the current encounter section, to be inserted into the tile
    /// * encounter_section: String - HTML for the historical encounter section, to be inserted into the tile
    /// * active_user_session: UserSession - the session of the current user, providing additional context data for display
    /// * intv_section: String - HTML for the intevention section, to be inserted into the tile 
    /// * intervention_type_list: Vec<(i64, String, String)>  - a list of Intervention Types for the current intervention, to construct a dropdown list to be displayed
    /// * legacy_menu: String - HTML for the legacy menu to be inserted into the tile
    /// * feature_pref_section: String - HTML for the feature preference list to be inserted into the tile
    /// * patient_id: String - the id of the patient to be displayed
    /// * encounter_id: String - the id of the encounter to be displayed
    /// 
    /// #### Returns:
    /// * String: the resulting HTML to be rendered
    /// 
    pub fn get_patient_details_full_tile(&self, patient_header: String, current_encounter: String, encounter_section: String,
                                                active_user_session: UserSession, legacy_menu: String, intv_section: String, 
                                                intervention_type_list: Vec<(i64, String, String)>,
                                                feature_pref_section: String,
                                                patient_id: String,
                                                encounter_id: String) -> String {
        tracing::debug!(">get_patient_details_full_tile()");
        let layout = self.get_tile(WebContentItem::WCTypePatientListTile);

        // base content
        let ht2 = &self.get_home_tile_with_user_identity(active_user_session.get_user_display_name()).replace(constants::BODY_TILE_CONTENT_TAG, &layout); // build the individual sections

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
        let ht10 = &ht9.replace(constants::FEATURE_PREFERENCE_TILE_TAG, &feature_pref_section);

        // hide the discharge form & button (<div>) if the user does not have access to the feature
        let visible_tag = match active_user_session.has_permission(Permission::ALLOW_CREATE_UPDATE_DISCHARGE) {
            true => String::new(),
            false => " class='hidden' ".to_string(),
        };
        let ht11 = &ht10.replace(constants::SECTION_1_VISIBLE_TAG, &visible_tag);

        let visible_tag = match active_user_session.has_permission(Permission::ALLOW_CREATE_CLINICAL_INTERVENTION) || active_user_session.has_permission(Permission::ALLOW_CREATE_NON_CLINICAL_INTERVENTION)  {
            true => String::new(),
            false => " class='hidden' ".to_string(),
        };
        let ht12 = &ht11.replace(constants::SECTION_2_VISIBLE_TAG, &visible_tag);

        let ht_final = &ht12.replace(constants::LEGACY_MENU_TILE_TAG, &legacy_menu);

        return ht_final.clone();
    }

    /// ### WebContentFactory::get_admit_discharge_full_tile()
    ///   Provide HTML for creating a new patient admit, or completing it as a discharge for an existing patient
    ///   It is the same table (Encounter), so one route should suffice
    /// 
    /// #### Parameters:
    /// * user_identity_label: String - the current user's identity string to be displayed
    /// * current_patient: Option<Patient> - the current (option-wrapped) patient to be displayed
    /// * legacy_menu: String - HTML for the legacy menu to be inserted into the tile
    /// * location_menu: String - HTML for the dropdown list of locations to be inserted into the tile
    /// * is_discharge_flag: bool - True/False flag to indicate if the discharged patients are to be displayed.
    /// * user_prompt: String - the NL prompt provided by the user (if available)
    /// 
    /// #### Returns:
    /// * String: the resulting HTML to be rendered
    /// 
    pub fn get_admit_discharge_full_tile(&self, user_identity_label: String,
                                                current_patient: Option<Patient>,
                                                legacy_menu: String,
                                                location_menu: String,
                                                is_discharge_flag: bool,
                                                user_prompt: String) -> String {

        tracing::debug!(">get_admit_discharge_full_tile()");
        println!(">get_admit_discharge_full_tile()");

        let labels = ["{patient_first_name}","{patient_last_name}", "{middle_name}",
                                  "{phn}","{birthdate}",  "{encounter_id}", "{patient_id}"];

        let base_tile_level_1 = match is_discharge_flag{
            true => self.get_tile(WebContentItem::WCTypeDischargeTile),
            false => self.get_tile(WebContentItem::WCTypeAdmitTile),
        };

        println!("..prompt: {}", user_prompt);

        // warning: match is on an Option<Patient>, NOT a tile
        let inner_content = match current_patient{ // get basic static tile loaded, make edits depending on type

            None =>{ // new patient (Admit) patH
                if is_discharge_flag {
                    tracing::debug!("..Discharge without Patient => INVALID");
                    println!("..Discharge without Patient => INVALID");
                }
                else{
                    tracing::debug!("..Admit New Patient");
                    println!("..Admit New Patient");
                }

                let data_items = [&String::new(),
                                                &String::new(),
                                                &String::new(),
                                                &String::new(),
                                                &String::new(),    
                                                &String::new(),
                                                &"-1".to_string()];

                let mut result = base_tile_level_1.clone();
                for i in 0..labels.len() {
                    result = result.replace(labels[i], data_items[i]);
                } 

                // ...except for admit_timestamp which will be Now()
                let result_2 = result.replace("{admit_timestamp}", &chrono::Utc::now().format(constants::SYSTEM_DATETIME_FORMAT ).to_string());
                let result_3 = result_2.replace("{location_id}", &location_menu);
                let result_4 = result_3.replace("{user_prompt}", &user_prompt);
                let result_5 = result_4.replace("{admit_notes}", &user_prompt);
                
                result_5
            } 
            Some (p) => {
                if is_discharge_flag {
                    tracing::debug!("  Discharge existing Patient");
                    println!("..Discharge existing Patient");
                }
                else{
                    tracing::debug!("  Admit update: existing Patient");
                    println!("..Admit update: existing Patient");
                }         
                let data_items = [&p.legal_first_name, &p.legal_last_name, &p.legal_middle_names,
                                                &p.phn_to_string(), &p.birth_date_for_display(),        
                                                &p.encounter_id.to_string(),
                                                &p.id.to_string()];

                let mut result = base_tile_level_1.clone();
                for i in 0..labels.len() {
                    result = result.replace(labels[i], data_items[i]);
                }
               
                let result_2 = result.replace("{admit_timestamp}", &p.admit_timestamp_for_display());
                let result_3 = result_2.replace("{user_prompt}", &user_prompt);
                                
                if is_discharge_flag {
                    let result_4 = result_3.replace("{discharge_timestamp}", &chrono::Utc::now().format(constants::SYSTEM_DATETIME_FORMAT).to_string());
                    let result_5 = match p.discharge_notes.len() == 0 && user_prompt.len() > 0 {
                        true => result_4.replace("{discharge_notes}", &user_prompt),
                        false => result_4.replace("{discharge_notes}", &p.discharge_notes),
                    };

                    let result_6 = match p.admit_notes.len() == 0 && user_prompt.len() > 0 { // discharge also includes the admission details
                        true => result_5.replace("{admit_notes}", &user_prompt),
                        false => result_5.replace("{admit_notes}", &p.admit_notes),
                    };

                    let result_7 = result_6.replace("{location_short_name}", &p.location_short_name);
                    let result_8 = result_7.replace("{location_id}", &p.location_id.to_string());
                    result_8
                }
                else{
                    let result_4 = match p.admit_notes.len() == 0 && user_prompt.len() > 0 {
                        true => result_3.replace("{admit_notes}", &user_prompt),
                        false => result_3.replace("{admit_notes}", &p.admit_notes),
                    };
                    
                    let result_5 = result_4.replace("{discharge_notes}", &p.discharge_notes);
                    let result_6 = result_5.replace("{location_id}", &location_menu);
                    result_6
                }
            }
        };
        
        // this first one replaces the base tile (loaded from file) with the new "layout" provided above
        let home_tile_level_0 = &self.get_home_tile_with_user_identity(user_identity_label).replace(constants::BODY_TILE_CONTENT_TAG, &inner_content.clone()); // build the individual sections
        let home_tile_level_1 = &home_tile_level_0.replace(constants::LEGACY_MENU_TILE_TAG, &legacy_menu);

        return home_tile_level_1.clone();
    }
   
    /// ### WebContentFactory::get_modify_intervention_full_tile()
    ///   Provides HTML for rendering the view/modify (full) intervention tile
    /// 
    /// #### Parameters:
    /// * user_identity_label: String - the current user's identity string to be displayed
    /// * current_intervention: Option<Intervention> - the current (option-wrapped) intervention to be displayed
    /// * legacy_menu: String - HTML for the legacy menu to be inserted into the tile
    /// * user_dropdown_list: Vec<(i64, String, String)> - HTML for the dropdown list of users to be inserted into the tile
    /// * status_dropdown_list: Vec<(i64, String, String)> - HTML for the dropdown list of intervention statuses to be inserted into the tile
    /// * location_menu: String - HTML for the dropdown list of locations to be inserted into the tile
    /// * intv_type: (i64, String, String) - the current type of the intervention, along with its id number and display name. This is used in lieu of a list
    /// * req: InterventionDataForm - the user's current request data, encapsulated in an InterventionDataForm
    /// * intv_details_list: Option<Vec<InterventionDetail>> - an option-wrapped list of InterventionDetails for the current intervention, to construct a list to be displayed
    /// * measures_dropdown_list: Vec<(i64, String, String)> - a list of Intervention Types (measures) for the current intervention, to construct a dropdown list to be displayed
    /// * feature_pref_section: String - HTML for the feature preference list to be inserted into the tile
    /// 
    /// #### Returns:
    /// * String: the resulting HTML to be rendered
    /// 
    pub fn get_modify_intervention_full_tile(&self, user_identity_label: String,
                                                  current_intervention: Option<Intervention>,
                                                  legacy_menu: String,
                                                  user_dropdown_list: Vec<(i64, String, String)>,
                                                  status_dropdown_list: Vec<(i64, String, String)>,
                                                  location_menu: String,
                                                  intv_type: (i64, String, String),
                                                  req: InterventionDataForm,
                                                  intv_details_list: Option<Vec<InterventionDetail>>,
                                                  measures_dropdown_list: Vec<(i64, String, String)>,
                                                  feature_pref_section: String
                                                  ) -> String {
        tracing::debug!(">get_modify_intervention_full_page_tile()");
        println!(">get_modify_intervention_full_page_tile()");

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
                                constants::ERR_LABEL_NO_ERROR_TAG,
                                "<div id=\"MapleEMR::InterventionDetailsList\">",
                                constants::FEATURE_PREFERENCE_TILE_TAG,
                                "{hide_add_new_measure}"];

        let id: String; // this entire block is required in order to address partial moves that occur below when we copy over the String values
        let intervention_type: String;             // this must all occur before the copy and outside of the match block below
        let tmp_intervention_type_id: String;          // as the selection of Some()/None does not allow the move
        let mut scheduled_timestamp: String = chrono::Utc::now().format(constants::SYSTEM_DATETIME_FORMAT ).to_string();
        let mut performed_timestamp: String = scheduled_timestamp.clone();
        let dd_user: String;
        let dd_intv_status: String;
        let description: String;
        let notes: String;
        let tmp_encounter_id: String;

        println!("..form errors?: >{}<", req.clone().form_errors);

        let data_items = match current_intervention{
            None =>{ // Create new Intervention path
                tracing::debug!("..View to create new Intervention");
                println!("..View to create new Intervention");
                println!("..(debug001) encounter_id: {}", req.encounter_id.to_string());
                dd_user = CommonFormatter::get_dropdown_user_with_department(user_dropdown_list,constants::NOT_SPECIFIED_ID); // "<div id=\"MapleEMR::UserIdDropDownControl\">",
                dd_intv_status =  CommonFormatter::get_dropdown_intervention_status(status_dropdown_list, constants::DEFAULT_INTERVENTION_STATUS_NEW); // "<div id=\"MapleEMR::StatusIdDropDownControl\">",

                let tmp_data_items = [constants::NOT_SPECIFIED_ID.to_string(), //"{intervention_id}",
                                                  intv_type.1, //"{intervention_type}",
                                                  req.intervention_type_id.to_string(), //"{intervention_type_id}",  //TODO
                                                  scheduled_timestamp, //"{scheduled_timestamp}",
                                                  performed_timestamp, //"{performed_timestamp}",
                                                  location_menu , //"{location_id}", 
                                                  dd_user,        //"<div id=\"MapleEMR::UserIdDropDownControl\">",
                                                  dd_intv_status, //"<div id=\"MapleEMR::StatusIdDropDownControl\">",
                                                  String::new(), //"{description}",
                                                  String::new(), //"{notes}",
                                                  req.encounter_id.to_string(), // "{encounter_id}",  //TODO
                                                  req.patient_id.to_string(),
                                                  constants::ERR_LABEL_NO_ERROR_TAG.to_string(), // **preserve the tag** .. no error to display
                                                  String::new(),  //"<div id=\"MapleEMR::InterventionDetailsList\">"
                                                  String::new(), // feature_pref_section // if the intervention has not been saved, do not allow preference additions
                                                  " class='hidden'".to_string()
                                                  ];
                tmp_data_items
            } 
            Some (intv) => {
                tracing::debug!("..Update existing Intervention");
                println!("..Update existing Intervention");
                dd_user = CommonFormatter::get_dropdown_user_with_department(user_dropdown_list,intv.users_id); // "<div id=\"MapleEMR::UserIdDropDownControl\">",
                dd_intv_status =  CommonFormatter::get_dropdown_intervention_status(status_dropdown_list, intv.status_id); // "<div id=\"MapleEMR::StatusIdDropDownControl\">",

                let intv_details_html =  InterventionFormatter::get_view_only_intervention_details_list(
                                                  self.get_tile(WebContentItem::WCTypeIntvDetailItemTile),
                                                                   intv_details_list,
                                                                                  measures_dropdown_list.clone(),
                                                                                  req.patient_id.to_string());

                id = intv.id.to_string();
                intervention_type = intv.clone().intervention_type; // intv_type.1
                tmp_intervention_type_id = intv.intervention_type_id.clone().to_string();
                scheduled_timestamp = intv.clone().scheduled_timestamp_for_display();
                performed_timestamp = intv.clone().performed_timestamp_for_display();
                //location_id = intv.location_id.to_string(); // location_menu below
                description = intv.description.to_string();
                notes = intv.notes.to_string();
                tmp_encounter_id = intv.encounter_id.to_string();
                println!("..(debug002) encounter_id: {}", intv.encounter_id.to_string());

                let form_errors = match req.clone().form_errors.as_str() {
                    "" => constants::ERR_LABEL_NO_ERROR_TAG.to_string(), 
                    _ => {
                        let mut tmp_error = constants::ERR_LABEL_WITH_ERROR_TAG.to_string();
                        let error_msg = req.clone().form_errors;
                        tmp_error = tmp_error.replace ("{form_errors}",&error_msg);
                        println!("..errors reported: {}", error_msg.clone());
                        tmp_error
                    },
                };

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
                                                    req.patient_id.to_string(),
                                                    form_errors,
                                                    intv_details_html,
                                                    feature_pref_section,
                                                    String::new() // by not hiding {hide_add_new_measure}, we show the section
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

        // this one is required for the local Add Measures dropdown
        let body_tile_level_2 = &body_tile_level_1.replace(constants::INTERVENTION_DETAILS_TYPE_DROP_DOWN_CONTROL_TAG,
                                                                        &&CommonFormatter::get_dropdown_generic( measures_dropdown_list,
                                                                        "addFrm_type_id".to_string(),
                                                                        constants::INVALID_OTHER_ID));
        
        let body_tile_level_3 = body_tile_level_2.replace(constants::LEGACY_MENU_TILE_TAG, &legacy_menu);
        //let body_tile_level_3 = body_tile_level_2.replace(constants::FEATURE_PREFERENCE_TILE_TAG, &feature_pref_section);
        
        let home_tile_level_0 = &self.get_home_tile_with_user_identity(user_identity_label).replace(constants::BODY_TILE_CONTENT_TAG, &body_tile_level_3.clone()); // build the individual sections
        let home_tile_level_final = &home_tile_level_0.replace(constants::LEGACY_MENU_TILE_TAG, &legacy_menu);

        return home_tile_level_final.clone();
    }
}
