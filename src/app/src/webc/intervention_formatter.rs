/// -------------------------------------------------------------------
/// Struct and implementation for creating html that formats Inverventions
///   and Intervention Details.
/// -------------------------------------------------------------------
use crate::dto::intervention_detail::InterventionDetail;
use crate::dto::intervention::Intervention;

use crate::webc::common::CommonFormatter;

pub struct InterventionFormatter{}

impl InterventionFormatter {

    ///
    /// Provide HTML for a readonly list of InterventionDetail(s)
    /// 
    pub fn get_view_only_intervention_details_list(intvdtls_list: Vec<InterventionDetail>) -> String {
        let mut results_sbuf = String::with_capacity(500); 

        results_sbuf.push_str("<table>");
        for item in intvdtls_list{
            results_sbuf.push_str("<tr><td>");
            results_sbuf.push_str( &item.id.to_string() ); 
            results_sbuf.push_str("</td><td>");
            results_sbuf.push_str( &item.intervention_type ); 
            results_sbuf.push_str("</td><td>"); 
            results_sbuf.push_str( &item.value );
            results_sbuf.push_str("</td><td>"); 
            results_sbuf.push_str( &item.entry_timestamp_for_display() );
            results_sbuf.push_str("<td></tr>\n");
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
    }

}