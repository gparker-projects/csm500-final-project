//! -------------------------------------------------------------------
//! Struct and implementation for creating the legacy html menu
//! -------------------------------------------------------------------
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 
use crate::session::UserSession;
use crate::constants;
use crate::dto::user_auth::Permission;
use crate::dto::patient::*;

pub struct MenuFormatter{}

impl MenuFormatter {
    ///
    /// Provide HTML for the main system menu; replaces tag: <!--MapleEMR::LegacyMenu-->
    /// 
    pub fn get_legacy_menu(&self, patient_list: Vec<Patient>, active_user_session: UserSession) -> String {
       return self.get_legacy_menu_with_patient(patient_list, constants::INVALID_PATIENT_ID, active_user_session);
    }

    ///
    /// Provide HTML for the main system menu; replaces tag: <!--MapleEMR::LegacyMenu-->
    /// 
    pub fn get_legacy_menu_with_patient(&self, patient_list: Vec<Patient>, patient_id: i64, active_user_session: UserSession) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        let mut first_entry: bool = true;

        results_sbuf.push_str("<div id=\"legacyMenu\" align=\"left\"><ul><li><a class=\"menuNotCurrent\" href=\"\\home\">Current Patients</li>");
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
                results_sbuf.push_str(")\">&nbsp;&nbsp;");
                results_sbuf.push_str( &p.legal_last_name ); 
                results_sbuf.push_str(",&nbsp;"); 
                results_sbuf.push_str( &p.legal_first_name );
                results_sbuf.push_str("</a></li>\n");
            }
        }

        // user must have admit permission to admit a Patient
        if active_user_session.has_permission(Permission::ALLOW_CREATE_UPDATE_ADMIT) {
            let admit_form = r##"<form action="/admitnew" method="post" id="admitFrm" name="admitFrm">
                                        <input type="hidden" id="patient_id" name="patient_id" value="-1">
                                        <input type="hidden" id="action_flag" name="action_flag" value="admit">
                                        <input type="hidden" id="user_prompt" name="user_prompt" value="">
                                    </form>"##;
            results_sbuf.push_str(admit_form);
            results_sbuf.push_str("<li><a class=\"menuOther\" href=\"javascript:admit_patient();\">Admit New Patient</a></li>"); // does not actually pass in a prompt from this method
        }
        
        results_sbuf.push_str("<li><p><p><p><p></li>"); // spacer
        results_sbuf.push_str("<li><a class=\"menuOther\" href=\"\\\">Log Out</a></li>");
        results_sbuf.push_str("<li><p></p><div class='userIdentity'>&nbsp;&nbsp;");
        results_sbuf.push_str(&active_user_session.user_display_name.clone());
        results_sbuf.push_str("</div></li></ul></div>");

        return results_sbuf;
    }
}