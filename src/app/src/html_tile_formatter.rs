/// -------------------------------------------------------------------
/// Struct to ease the fomatting of general HTML I expect to be needed
/// -------------------------------------------------------------------
pub struct HTMLTileFormatter; // no variables at this time

impl HTMLTileFormatter{

    /// Default constructor; no params at this time
    /// 
   // pub fn new() -> Self {
   //     Self { }
   // }

    /// takes in a vector of strings, returns as an HTML table row
    /// 
    pub fn format_row(data_row: Vec<&str>, is_header: bool) -> String {
        let mut results_sbuf = String::with_capacity(40); 
        results_sbuf.push_str("  <tr>");


        results_sbuf.push_str("  </tr>\n");

        return results_sbuf;
    }
}