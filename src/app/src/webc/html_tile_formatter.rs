/// -------------------------------------------------------------------
/// Struct to ease the fomatting of Tiles of data
/// -------------------------------------------------------------------

pub struct HTMLTileFormatter; // no variables at this time

impl HTMLTileFormatter{

    /// Formats a patient object
    /// 
    pub fn format_patient() -> String {
        let mut results_sbuf = String::with_capacity(40); 
        results_sbuf.push_str("  <tr>");


        results_sbuf.push_str("  </tr>\n");

        return results_sbuf;
    }

    /// Formats an intervention object
    /// 
    pub fn format_intervention() -> String {
        let mut results_sbuf = String::with_capacity(40); 
        results_sbuf.push_str("  <tr>");


        results_sbuf.push_str("  </tr>\n");

        return results_sbuf;
    }

    /// Formats an encounter object
    /// 
    pub fn format_encounter() -> String {
        let mut results_sbuf = String::with_capacity(40); 
        results_sbuf.push_str("  <tr>");


        results_sbuf.push_str("  </tr>\n");

        return results_sbuf;
    }
}