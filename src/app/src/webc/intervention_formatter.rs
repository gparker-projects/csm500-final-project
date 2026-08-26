/// -------------------------------------------------------------------
/// Struct and implementation for creating html that formats Inverventions
///   and Intervention Details.
/// -------------------------------------------------------------------
use crate::dto::intervention_detail::InterventionDetail;

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
}