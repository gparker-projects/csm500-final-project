//! -------------------------------------------------------------------
//! Struct and implementation for creating html that formats Inverventions
//!   and Intervention Details.
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

pub struct InterventionFormatter{}

impl InterventionFormatter{

    ///
    /// Provide HTML for a readonly list of InterventionDetail(s)
    /// 
    pub fn get_view_only_intervention_details_list(intervention_details_item_tile: String,
                                                   intvdtls_list: Option<Vec<InterventionDetail>>, 
                                                   measures_dropdown_list: Vec<(i64, String, String)>,
                                                   patient_id: String) -> String {
        let mut results_sbuf = String::with_capacity(500); 
        let mut counter = 1;
        tracing::debug!(">get_view_only_intervention_details_list()");
       
        let results_html_final = match intvdtls_list { // if there were no intervention details, we still want to output the "add new" section
            None => {
                "No Measures Added".to_string() 
            },
            Some(inner_list) => {  // if there were no intervention details, fill out the details
                for item in inner_list{ // assemble each separate intervention detail
                    let raw_tile = intervention_details_item_tile.clone(); // we receive a raw tile that has the Intervention Details add/edit HTML
                    let type_drop_down_control_name = "type_id_".to_string() + &item.id.to_string(); // due to dynamic additions etc, careful naming required
                    //println!("..Create control id={}", type_drop_down_control_name.clone());

                    // page body content
                    let html_level_0 = &raw_tile.replace(constants::ITEM_ID_INLINE_TAG, &item.id.to_string());
                    let html_level_1 = &html_level_0.replace("{notes}",  &item.notes.to_string());
                    let html_level_2 = &html_level_1.replace("{value}",  &item.value.to_string());
                    let html_level_3 = &html_level_2.replace("{type_name}",  &item.type_name().to_string());
                    let html_level_4 = &html_level_3.replace("{intervention_id}",  &item.intervention_id.to_string());
                    let html_level_5 = &html_level_4.replace("{count}",  &counter.to_string());
                    let html_level_6 = &html_level_5.replace("{entry_timestamp}",  &item.entry_timestamp_for_display().to_string());
                    let html_level_7 = &html_level_6.replace("{patient_id}",  &patient_id); 

                    let html_level_final = &html_level_7.replace(constants::INTERVENTION_DETAILS_TYPE_DROP_DOWN_CONTROL_TAG,
                                                                        &&CommonFormatter::get_dropdown_generic( measures_dropdown_list.clone(),
                                                                                type_drop_down_control_name,
                                                                                item.type_id));
                    results_sbuf.push_str(&html_level_final);
                    counter = counter + 1;
                }
                results_sbuf
            }                
        };

        return results_html_final;
    }

 /*
    ///
    /// Provide HTML for all of a (Patient's) Encounter's Interventions
    /// 
   pub fn get_intervention_list_for_patient_details_tile(intervention_list: Vec<Intervention>) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        tracing::debug!(">get_intervention_list_tile()");

        results_sbuf.push_str(&CommonFormatter::get_hidden_form("intvDtls".to_owned(), "intvDtlsFrm".to_owned()) );
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
    }*/

}