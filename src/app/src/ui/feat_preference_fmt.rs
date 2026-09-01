//! -------------------------------------------------------------------
//! Struct and implementation for creating "Fast action" one-click button forms. These HTML components are used by the NLE and Feature Preference
//!  operations to create instant-forms that will submit to the regular actions in the system. This is a means to keep the requests on server
//!   side and avoid messy/brittle javascript calls.
//! 
//! Rules: 1) New Interventions and Discharges can only be created from within a Patient (due to context requirement)
//!        2) New Measures (Intervention Details) can only be created from within an Intervention (due to context requirement)
//!        3) Admissions can happen anywhere (no context needed)
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 
//! -------------------------------------------------------------------
use crate::constants;

use crate::dto::intervention_detail::InterventionDetail;
use crate::ui::common::CommonFormatter;
use crate::dto::feature_preference::FeaturePreference;

pub struct FeaturePreferenceFormatter{}

impl FeaturePreferenceFormatter{

       ///
    /// Generates an HTML tile based on a list of FeaturePreference objects.
    /// 
    pub fn get_feature_preference_section(feature_pref_list: Option<Vec<FeaturePreference>>) -> String {
        tracing::debug!("get_feature_preference_tile()");
        let mut results_sbuf = String::with_capacity(500);

        let results = match feature_pref_list{
            Some(pref_list) => {
                let single_pref_tile: &str = r##"<input type="submit"
                                                  name="fast_action_btn_id_{fast_action_id}"
                                                    id="fast_action_btn_id_{fast_action_id}"
                                                  value="(+) {fast_action_name}"
                                                  onclick="event.preventDefault(); return fastAction({fast_action_id});">
                                                 "##;
                for item in pref_list{
                    let item_layout_0 = single_pref_tile; // start with base tile
                    let item_layout_1 = item_layout_0.replace("{fast_action_id}", &item.feature_id.to_string()); 
                    let item_layout_final = item_layout_1.replace("{fast_action_name}", &item.ref_name); 
                    results_sbuf.push_str(&item_layout_final); 
                }
                results_sbuf
            },
            None => "".to_string(), // if no content, return nothing
        };
        results        
    }

    ///
    /// Provide HTML for a single-click button that will navigate to admit a patient
    /// 
    pub fn get_admit_button(patient_id: i64, _payload: String) -> String {
        tracing::debug!(">get_discharge_button()");
        let mut results_sbuf = String::with_capacity(500); 

        results_sbuf
    }

    ///
    /// Provide HTML for a single-click button that will navigate to discharge a patient
    /// 
    pub fn get_discharge_button(patient_id: i64, _payload: String) -> String {
        tracing::debug!(">get_discharge_button()");
        let mut results_sbuf = String::with_capacity(500); 

        results_sbuf
    }

    ///
    /// Provide HTML for a single-click button that will navigate to discharge a patient
    /// 
    pub fn get_add_intervention_button(patient_id: i64, encounter_id: i64, _payload: String) -> String {
        tracing::debug!(">get_discharge_button()");
        let mut results_sbuf = String::with_capacity(500); 

        results_sbuf
    }

    ///
    /// Provide HTML for a single-click button that will navigate to discharge a patient
    /// 
    pub fn get_add_measure_button(patient_id: i64, encounter_id: i64, intervention_id: i64, _payload: String) -> String {
        tracing::debug!(">get_discharge_button()");
        let mut results_sbuf = String::with_capacity(500); 

        results_sbuf
    }
}