//! -------------------------------------------------------------------
//! Struct and implementation for creating html that formats common data
//! controls such as drop down lists, generic hidden forms etc.
//! -------------------------------------------------------------------
//! 
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 


pub struct CommonFormatter{}

impl CommonFormatter {
    ///
    /// Returns a hidden form, used as a technique in several of the list tiles to submit a value for another screen
    /// 
    pub fn get_hidden_form(target_name: String, form_name: String) -> String {
        let body = r##"<div id="hiddenSection" style="display: none; margin-top: 0px;">
                               <form action="\{target_name}" method="post" id="{form_name}" name="{form_name}">
                               <input type="hidden" name="target_id" id="target_id" value="0">
                             </form></div>"##;

        body.replace("{form_name}", &form_name).replace("{target_name}", &target_name)
    }

    ///
    /// Generates a list of locations based on what is in the system
    /// 
    pub fn get_dropdown_generic(item_list: Vec<(i64, String, String)>, list_name_and_id: String, default_item_id: i64) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        tracing::debug!("> get_dropdown_generic({})", list_name_and_id);

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
    pub fn get_dropdown_user_with_department(item_list: Vec<(i64, String, String)>, default_item_id: i64) -> String {
        CommonFormatter::get_dropdown_generic(item_list, "users_id".to_string(), default_item_id)
    }

    pub fn get_dropdown_intervention_status(item_list: Vec<(i64, String, String)>, default_item_id: i64) -> String {
        CommonFormatter::get_dropdown_generic(item_list, "status_id".to_string(), default_item_id)
    }

    ///
    /// Generates a list of locations based on what is in the system
    /// 
    pub fn get_location_dropdown(location_list: Vec<(i64, String)>, default_location_id: i64) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        tracing::debug!("> get_location_dropdown()");

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

}