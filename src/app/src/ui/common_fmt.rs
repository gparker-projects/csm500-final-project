//! -------------------------------------------------------------------
//! Struct and implementation for creating html that formats common data
//! controls such as drop down lists, generic hidden forms etc.
//! -------------------------------------------------------------------
//! 
//!  CSM500 Project (April - October 2026)
//!  Graham Parker (Student ID: 240120522)
//! -------------------------------------------------------------------

pub struct CommonFormatter{}

impl CommonFormatter {

    /// ### CommonFormatter::get_hidden_form()
    ///    Returns a hidden form, used as a technique in several of the list tiles to submit a value for another screen
    /// 
    /// #### Parameters:
    /// * target_name: String - name of the target HTML action (submit target), for replacement in the generated HTML
    /// * form_name: String - name of the HTML form, for replacement in the generated HTML
    /// 
    /// #### Returns:
    /// * String: the resulting HTML for a hidden form
    /// 

    pub fn get_hidden_form(target_name: String, form_name: String) -> String {
        let body = r##"<div id="hiddenSection" style="display: none; margin-top: 0px;">
                               <form action="\{target_name}" method="post" id="{form_name}" name="{form_name}">
                               <input type="hidden" name="target_id" id="target_id" value="0">
                             </form></div>"##;

        body.replace("{form_name}", &form_name).replace("{target_name}", &target_name)
    }

    /// ### CommonFormatter::get_dropdown_generic()
    ///    Generates a list of locations based on what is in the system
    /// 
    /// #### Parameters:
    /// * item_list: Vec<(i64, String, String)> - a vector of items to be used to construct the dropdown list
    ///   * i64: the id of the entry
    ///   * String.0: the value of the entry, not shown to user unless they view the HTMl source
    ///   * String.1: the description of the option entry, shown to the user
    /// * list_name_and_id: String - text to be used for both the name and id of the HTML control
    /// * default_item_id: i64 - the id/index of the item that is be selectd by default
    /// 
    /// #### Returns:
    /// * String: the resulting HTML for a drop down list (<option></option> tag)
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
            if row.0 == default_item_id {
                results_sbuf.push_str("' selected >");
            }
            else{
                results_sbuf.push_str("' >");
            }
            results_sbuf.push_str(&row.1); // description here
            results_sbuf.push_str("</option>");
        }
        results_sbuf.push_str("</select>");

        return results_sbuf;
    }


    /// ### CommonFormatter::get_dropdown_user_with_department()
    ///    Generates a dropdown for users
    /// 
    /// #### Parameters:
    /// * item_list: Vec<(i64, String, String)> - a vector of items to be used to construct the dropdown list. List will have an id/name of "users_id".
    ///   * i64: the id of the entry
    ///   * String.0: the value of the entry, not shown to user unless they view the HTMl source
    ///   * String.1: the description of the option entry, shown to the user
    /// * default_item_id: i64 - the id/index of the list that is be selectd by default
    /// 
    /// #### Returns:
    /// * String: the resulting HTML for a drop down list (<option></option> tag)
    /// 
    pub fn get_dropdown_user_with_department(item_list: Vec<(i64, String, String)>, default_item_id: i64) -> String {
        CommonFormatter::get_dropdown_generic(item_list, "users_id".to_string(), default_item_id)
    }

    /// ### CommonFormatter::get_dropdown_intervention_status()
    ///    Generates a dropdown of intervention types, based on the provided list. List will have an id/name of "status_id".
    /// 
    /// #### Parameters:
    /// * item_list: Vec<(i64, String, String)> - a vector of items to be used to construct the dropdown list
    ///   * i64: the id of the entry
    ///   * String.0: the value of the entry, not shown to user unless they view the HTMl source
    ///   * String.1: the description of the option entry, shown to the user
    /// * default_item_id: i64 - the id/index of the list that is be selectd by default
    /// 
    /// #### Returns:
    /// * String: the resulting HTML for a drop down list (<option></option> tag)
    /// 
    pub fn get_dropdown_intervention_status(item_list: Vec<(i64, String, String)>, default_item_id: i64) -> String {
        CommonFormatter::get_dropdown_generic(item_list, "status_id".to_string(), default_item_id)
    }

    /// ### CommonFormatter::get_location_dropdown()
    ///    Generates a list of locations based on what is in the system
    /// 
    /// #### Parameters:
    /// * item_list: Vec<(i64, String)> - a vector of strings to be used to construct the dropdown list
    ///   * i64: the id of the entry
    ///   * String: the description of the option entry, shown to the user
    /// * default_item_id: i64 - the id/index of the item that is be selectd by default
    /// 
    /// #### Returns:
    /// * String: the resulting HTML for a drop down list (<option></option> tag)
    ///
    pub fn get_location_dropdown(location_list: Vec<(i64, String)>, default_location_id: i64) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        tracing::debug!("> get_location_dropdown()");

        // https://www.w3schools.com/tags/tag_select.asp
        results_sbuf.push_str("<select name='location_id' id='location_id'>");

        for row in location_list{
            results_sbuf.push_str("<option value='" );
            results_sbuf.push_str(&row.0.to_string()); // location_id here
            if row.0 == default_location_id {
                results_sbuf.push_str("' selected >");
            }
            else{
                results_sbuf.push_str("' >");
            }
            results_sbuf.push_str(&row.1); // description here
            results_sbuf.push_str("</option>");
        }
        results_sbuf.push_str("</select>");

        return results_sbuf;
    }
}