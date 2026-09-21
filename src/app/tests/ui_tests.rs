///
/// Unit & Integration tests for the User Interface (ui) module
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 

mod common;

use std::ops::IndexMut;

#[cfg(test)]
use maple_emr::constants;
//use maple_emr::dao::patient_dao::PatientWrapper;
use maple_emr::ui::tile_factory::{WebContentFactory, WebContentItem};
use maple_emr::ui::simple_fmt::SimpleFormatter;
use maple_emr::ui::common_fmt::CommonFormatter;
use maple_emr::ui::menu_fmt::MenuFormatter;

use maple_emr::dto::user_auth::{Permission, UserAuthorization};
use maple_emr::session::{UserSession}; // AppSession

use common::entity_factory::EntityFactory;

use maple_emr::dto::encounter::Encounter;

#[test]
fn test_wcf() {
  // https://stackoverflow.com/questions/61974382/load-a-resource-file-at-runtime
  // https://doc.rust-lang.org/std/env/fn.current_dir.html

  let path = std::env::current_dir().expect("Base path to executable could not be found");
  let newpath = path.display().to_string() + "\\webc\\static\\";

  // check path was constructed correctly
  assert_eq!(newpath, "C:\\uol\\csm500-final-project\\src\\app\\webc\\static\\"); 

  let wcf = WebContentFactory::new(&newpath, "fake user".to_string());
  // content factor should have two entries currently

  tracing::debug!("As of Sept 19, there are [7] tiles being loaded");
  assert_eq!(wcf.get_tile_count(), 7); 

  let tmp_tile = wcf.get_tile(WebContentItem::WCTypeLoginTile);
  assert!(tmp_tile.len() > 0);

  let tmp_tile2 = wcf.get_home_tile_with_user_identity("gparker-test".to_string());
  assert!(tmp_tile2.len() > 0);
  assert!( tmp_tile2.contains( &"gparker-test".to_string() ) );
}

#[test]
fn test_simple_formatter() {

    let search_string = "Unit test: test_simple_formatter()".to_string();
    let e: Encounter = Encounter{
        id: 1,
        admit_notes: search_string.clone(), 
        ..Default::default()
    };

    let mut results = SimpleFormatter::get_single_encounter_summary_tile(e);

    println!("SimpleFormatter::get_single_encounter_summary_tile()");
    assert!( results.contains("Admit Reason") , "..formatting incomplete"); // from the static part of the content
    assert!( results.contains( &search_string ) , "..formatting incomplete"); // from the data of the Encounter
    
    results = SimpleFormatter::get_single_patient_summary(EntityFactory::create_patient_wrapper(), 1);
    assert!( results.contains("Admission Concern") , "..formatting incomplete"); // from the static part of the content
    //assert!( results.contains( &search_string ) , "..formatting incomplete"); // from the data

    results = SimpleFormatter::get_home_route_summary_of_patients_tile_using_wrapper(EntityFactory::create_vector_of_patient_wrappers() );
    assert!( results.contains("patientDtlsFrm") , "..formatting incomplete"); // from the static part of the content
    //assert!( results.contains( &search_string ) , "..formatting incomplete"); // from the data

    results = SimpleFormatter::get_encounter_list_tile(EntityFactory::create_vector_of_encounters());
    assert!( results.contains("encounterDtlsFrm") , "..formatting incomplete"); // from the static part of the content
    //assert!( results.contains( &search_string ) , "..formatting incomplete"); // from the data

    let mut lst = EntityFactory::create_vector_of_interventions();
    lst.index_mut(0).intervention_type_id = constants::CRT_INTERVENTION_ALERT_TYPE;
    lst.index_mut(5).intervention_type_id = 100052; // this is a clinical type

    results = SimpleFormatter::get_intervention_list_for_patient_details_tile(lst.clone(), false);
    assert!( results.contains("intvDtlsFrm") , "..formatting incomplete"); // from the static part of the content

    // secondary check that the user can view clinical records
    results = SimpleFormatter::get_intervention_list_for_patient_details_tile(lst.clone(), true);
    assert!( results.contains("intvDtlsFrm") , "..formatting incomplete"); // from the static part of the content
}

#[test]
fn test_common_formatter() {
    let mut items = Vec::new();
    let item1 : (i64, String, String) = (1, "TEST ITEM 1".to_string(), "TEST ITEM 1".to_string());
    let item2 : (i64, String, String) = (2, "TEST-ITEM-2".to_string(), "TEST-ITEM-2".to_string());
    let item3 : (i64, String, String) = (3, "TEST ITEM 3".to_string(), "TEST ITEM 3".to_string());
    items.push(item1);
    items.push(item2);
    items.push(item3);

    let html_result = CommonFormatter::get_dropdown_generic(items.clone(), "dummy_id".to_string(), 1);
    assert!(html_result.contains(&"TEST-ITEM-2".to_string()), "CommonFormatter::get_dropdown_generic() produced incorrect HTML");
    
    let html_result2 = CommonFormatter::get_dropdown_user_with_department(items.clone(), 1);
    assert!(html_result2.contains(&"TEST-ITEM-2".to_string()), "CommonFormatter::get_dropdown_user_with_department() produced incorrect HTML");
    
    let html_result3 = CommonFormatter::get_dropdown_intervention_status(items.clone(), 1);
    assert!(html_result3.contains(&"TEST-ITEM-2".to_string()), "CommonFormatter::get_dropdown_intervention_status() produced incorrect HTML");

    let mut items2 = Vec::new();
    let item21 : (i64, String) = (1, "TEST LOCATION 1".to_string());
    let item22 : (i64, String) = (2, "TEST-LOCATION-2".to_string());
    let item23 : (i64, String) = (3, "TEST LOCATION 3".to_string());
    items2.push(item21);
    items2.push(item22);
    items2.push(item23);
    let html_result4 = CommonFormatter::get_location_dropdown(items2.clone(), 1);
    assert!(html_result4.contains(&"TEST-LOCATION-2".to_string()), "CommonFormatter::get_location_dropdown() produced incorrect HTML");
}

#[test]
fn test_menu_formatter() {
    // create a couple of permission and put them in a Vector, then add to the UserAuthorization
    // the UserAuthorization gets put into the Session
    let perm: Permission = Permission::new(1, 1);
    let perm2: Permission = Permission::new(1, 2);
    let perm3: Permission = Permission::new(1, Permission::ALLOW_CREATE_UPDATE_ADMIT);

    let ua = UserAuthorization {
        granted_permissions: vec![perm, perm2, perm3]
    };

    let sess: UserSession = UserSession {
        user_id: "5".to_string(), // 
        user_display_name: "TEST, UNIT".to_string(), //
        email: "test@gmail.com".to_string(), //
        user_authorizations: ua //
    };

    let html_result = {MenuFormatter{}}.get_legacy_menu(EntityFactory::create_vector_of_patients(), sess.clone());
    assert!(html_result.contains(&"menuNotCurrent".to_string()), "MenuFormatter::get_legacy_menu() produced incorrect HTML");

    let html_result = {MenuFormatter{}}.get_legacy_menu_with_patient(EntityFactory::create_vector_of_patients(), 5, sess.clone());
    assert!(html_result.contains(&"menuNotCurrent".to_string()), "MenuFormatter::get_legacy_menu_with_patient() produced incorrect HTML");
}

#[test]
fn test_feature_pref_formatter() {
    
    //FeaturePreferenceFormatter 
    //get_feature_preference_section
}