///
/// Unit & Integration tests for the User Interface (ui) module
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 
#[cfg(test)]
mod ui_tests {
  use tracing;

  use maple_emr::ui::tile_factory::{WebContentFactory, WebContentItem};

  #[test]
  fn test_wcf() {
    // https://stackoverflow.com/questions/61974382/load-a-resource-file-at-runtime
    // https://doc.rust-lang.org/std/env/fn.current_dir.html

    let path = std::env::current_dir().expect("Base path to executable could not be found");
    let newpath = path.display().to_string() + "\\webc\\static\\";

    // check path was constructed correctly
    assert_eq!(newpath, "C:\\uol\\csm500-final-project\\src\\app\\webc\\static\\"); 

    let wcf = WebContentFactory::new(&newpath);
    // content factor should have two entries currently

    tracing::debug!("As of Aug 28, there are [6] tiles being loaded");
    assert_eq!(wcf.get_tile_count(), 6); 

    let tmp_tile = wcf.get_tile(WebContentItem::WCTypeLoginTile);
    assert!(tmp_tile.len() > 0);

    let tmp_tile2 = wcf.get_home_tile();
    assert!(tmp_tile2.len() > 0);

    let tmp_tile3 = wcf.get_home_tile_with_user_identity("gparker-test".to_string());
    assert!(tmp_tile3.len() > 0);
    assert!( tmp_tile3.contains( &"gparker-test".to_string() ) );
  }
}