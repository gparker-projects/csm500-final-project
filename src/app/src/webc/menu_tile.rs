/// -------------------------------------------------------------------
/// Struct and implementation for creating the legacy html menu
/// -------------------------------------------------------------------
use crate::constants;
use crate::dto::{patient::*};

pub struct MenuFormatter{}

impl MenuFormatter {
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

        let admit_menu_item = r##"<form action="/admitnew" method="post" id="admitFrm" name="admitFrm"> <input type="hidden" id="adm_target_id" name="adm_target_id" value="-1"></form>"##;

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