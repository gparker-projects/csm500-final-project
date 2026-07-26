/// -------------------------------------------------------------------
/// Struct ease the fomatting of HTML
/// 
/// 
/// 
/// -------------------------------------------------------------------
pub struct HTMLFormatter; // no variables at this time

impl HTMLFormatter{

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

        if !is_header {
            for item in data_row.iter() {
                results_sbuf.push_str("<td>"); 
                results_sbuf.push_str(item); 
                results_sbuf.push_str("</td>"); 
            }
        }
        else{
            for item in data_row.iter() {
                results_sbuf.push_str("<th>"); 
                results_sbuf.push_str(item); 
                results_sbuf.push_str("</th>"); 
            }
        }

        results_sbuf.push_str("  </tr>\n");

        return results_sbuf;
    }

    /// formats a data field for display as an edistable or uneditable field
    pub fn format_field(label: String, field_id: String, data: String, is_editable: bool) -> String {
        let mut results_sbuf = String::with_capacity(40); 
        results_sbuf.push_str("<label class=\"data-label\">");
        results_sbuf.push_str(&label);
        results_sbuf.push_str("<\"label>");

        if is_editable {
            results_sbuf.push_str("<input class=\"rw-input-field\" type=\"text\" id=\"");
            results_sbuf.push_str(&field_id);
            results_sbuf.push_str("\" name=\"");
            results_sbuf.push_str(&field_id);
            results_sbuf.push_str("\"\\>");  
        }
        else{
            results_sbuf.push_str("<label class=\"ro-input-field\">");
            results_sbuf.push_str(&data);
            results_sbuf.push_str("<\"label>");
        }

        return results_sbuf;
    }

}
