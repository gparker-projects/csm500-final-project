///
/// Unit & Integration tests for the Web Content (webc) module
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 
#[cfg(test)]
mod tests {

  use MapleEMR::webc::web_content::WebContentItem;

  #[test]
  fn test_web_content_load() {
    // https://stackoverflow.com/questions/61974382/load-a-resource-file-at-runtime
    // https://doc.rust-lang.org/std/env/fn.current_dir.html

    let path = std::env::current_dir().expect("Base path to executable could not be found");
    let newpath = path.display().to_string() + "\\webc\\static\\";

    // check path was constructed correctly
    assert_eq!(newpath, "C:\\uol\\csm500-final-project\\src\\app\\webc\\static\\"); 

    let wcf = MapleEMR::webc::web_content::WebContentFactory::new(&newpath);
    // content factor should have two entries currently
    assert_eq!(wcf.get_tile_count(), 3); 

    let tmp_tile = wcf.get_tile(WebContentItem::WCTypeLoginTile);
    //println!("WCTypeLoginTile: {}", tmp_tile.clone());
    // check data was actually loaded
    assert!(tmp_tile.len() > 0); 
  }

  #[test]
  fn test_hteml_formatter() {

    let data = vec!["a","b","c"];
    let head = vec!["ColA","ColB","ColC"];

    let line1 = MapleEMR::webc::html_formatter::HTMLFormatter::format_row(head, true);
    assert_eq!(line1, "  <tr><th>ColA</th><th>ColB</th><th>ColC</th>  </tr>\n".to_string()); 

    let line2 = MapleEMR::webc::html_formatter::HTMLFormatter::format_row(data, false);
    assert_eq!(line2, "  <tr><td>a</td><td>b</td><td>c</td>  </tr>\n".to_string()); 
  }
}