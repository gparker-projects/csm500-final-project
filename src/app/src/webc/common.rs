/// -------------------------------------------------------------------
/// Struct and implementation for creating html that formats Inverventions
///   and Intervention Details.
/// -------------------------------------------------------------------
//use crate::dto::intervention_detail::InterventionDetail;

pub struct CommonFormatter{}

impl CommonFormatter {

    // -----------------------------------------------------------------------------------
    // Common formatters
    // -----------------------------------------------------------------------------------

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

}