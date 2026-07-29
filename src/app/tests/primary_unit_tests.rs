///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
///

#[cfg(test)]
mod tests {

  use super::*; // provides access to all the rest of the code
  use MapleEMR::webc::web_content;

  #[test]
  fn test_stub() {

    let content_root_path = ".\\mplhtm";
    let wcf = MapleEMR::webc::web_content::WebContentFactory::new(content_root_path);

    assert_eq!(true, true); 
  }
}