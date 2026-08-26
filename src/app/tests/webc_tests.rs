///
/// Unit & Integration tests for the Web Content (webc) module
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 
#[cfg(test)]
mod tests {
  use tracing;

  use MapleEMR::webc::tile_factory::{WebContentFactory, WebContentItem};

  #[test]
  fn test_web_content_load() {
    // https://stackoverflow.com/questions/61974382/load-a-resource-file-at-runtime
    // https://doc.rust-lang.org/std/env/fn.current_dir.html

    let path = std::env::current_dir().expect("Base path to executable could not be found");
    let newpath = path.display().to_string() + "\\webc\\static\\";

    // check path was constructed correctly
    assert_eq!(newpath, "C:\\uol\\csm500-final-project\\src\\app\\webc\\static\\"); 

    let wcf = WebContentFactory::new(&newpath);
    // content factor should have two entries currently

    tracing::debug!("As of Aug 17, there are [5] tiles being loaded");
    assert_eq!(wcf.get_tile_count(), 5); 

    let tmp_tile = wcf.get_tile(WebContentItem::WCTypeLoginTile);
    assert!(tmp_tile.len() > 0); 
  }
}