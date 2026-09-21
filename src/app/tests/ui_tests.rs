///
/// Unit & Integration tests for the User Interface (ui) module
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 

mod common;

use std::ops::IndexMut;

#[cfg(test)]
use maple_emr::constants;
use maple_emr::dto::convert_utils::ConvertUtils;
//use maple_emr::dao::patient_dao::PatientWrapper;
use maple_emr::ui::tile_factory::{WebContentFactory, WebContentItem};
use maple_emr::ui::simple_fmt::SimpleFormatter;
use maple_emr::ui::common_fmt::CommonFormatter;
use maple_emr::ui::menu_fmt::MenuFormatter;

use maple_emr::dto::user_auth::{Permission, UserAuthorization};
use maple_emr::session::{UserSession}; // AppSession

use common::entity_factory::EntityFactory;

use maple_emr::dto::encounter::Encounter;

use crate::common::test_utils::DataGenerator;

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

    println!("As of Sept 19, there are [7] tiles being loaded");
    assert_eq!(wcf.get_tile_count(), 7); 

    let tmp_tile = wcf.get_tile(WebContentItem::WCTypeLoginTile);
    assert!(tmp_tile.len() > 0);

    let tmp_tile2 = wcf.get_home_tile_with_user_identity("gparker-test".to_string());
    assert!(tmp_tile2.len() > 0);
    assert!( tmp_tile2.contains( &"gparker-test".to_string() ) );
}

///
/// Tests WebContentFactory::get_patient_details_full_tile()
/// 
#[test]
fn test_wcf_get_patient_details_full_tile() {
    let path = std::env::current_dir().expect("Base path to executable could not be found");
    let newpath = path.display().to_string() + "\\webc\\static\\";
    let wcf = WebContentFactory::new(&newpath, "fake user".to_string());

    let pw = EntityFactory::create_patient_wrapper();
    let sess: UserSession = EntityFactory::create_user_session();
    let intv_type_list = EntityFactory::create_intervention_type_list("ITEM".to_string(), 2);
    
    let html_results = wcf.get_patient_details_full_tile(
        "patient_header".to_string(),
        "current_encounter".to_string(),
        "encounter_section".to_string(),
        sess, 
        "legacy_menu".to_string(),
        "intv_section".to_string(),
        intv_type_list,
        "feature_pref_section".to_string(),
        pw.patient.id.to_string(),
        pw.current_encounter.id.to_string()
    );

    assert_ne!(html_results, "".to_string(), "No HTML returned by: get_patient_details_full_tile()");

    //let html = wcf.get_admit_discharge_full_tile();

    //let html = wcf.get_modify_intervention_full_tile();
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
    
    // get_single_patient_summary Test 1
    results = SimpleFormatter::get_single_patient_summary(EntityFactory::create_patient_wrapper(), 1);
    assert!( results.contains("Admission Concern") , "..formatting incomplete"); // from the static part of the content

    // get_single_patient_summary Test 2: with non-patient Id
    results = SimpleFormatter::get_single_patient_summary(EntityFactory::create_patient_wrapper(), -1);
    assert!( results.contains("Admission Concern") , "..formatting incomplete"); // from the static part of the content

    // get_single_patient_summary Test 3: with an aged admit_timestamp 
    let mut pcw = EntityFactory::create_patient_wrapper();
    pcw.patient.admit_timestamp = DataGenerator::get_date(); // all dates provided are now > 4hrs old
    pcw.most_recent_intervention = None; // also test the "is_some()" condition for the method
    results = SimpleFormatter::get_single_patient_summary(pcw, 1);
    assert!( results.contains("Admission Concern") , "..formatting incomplete"); // from the static part of the content

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
    let items = EntityFactory::create_intervention_type_list("ITEM".to_string(), 2);

    let html_result = CommonFormatter::get_dropdown_generic(items.clone(), "dummy_id".to_string(), 1);
    assert!(html_result.contains(&"TEST-ITEM-2".to_string()), "CommonFormatter::get_dropdown_generic() produced incorrect HTML");
    
    let html_result2 = CommonFormatter::get_dropdown_user_with_department(items.clone(), 1);
    assert!(html_result2.contains(&"TEST-ITEM-2".to_string()), "CommonFormatter::get_dropdown_user_with_department() produced incorrect HTML");
    
    let html_result3 = CommonFormatter::get_dropdown_intervention_status(items.clone(), 1);
    assert!(html_result3.contains(&"TEST-ITEM-2".to_string()), "CommonFormatter::get_dropdown_intervention_status() produced incorrect HTML");

    let items2 = EntityFactory::create_location_list( 2);
    let html_result4 = CommonFormatter::get_location_dropdown(items2.clone(), 1);
    assert!(html_result4.contains(&"TEST-LOCATION-2".to_string()), "CommonFormatter::get_location_dropdown() produced incorrect HTML");
}

#[test]
fn test_menu_formatter() {
    let mut sess: UserSession = EntityFactory::create_user_session();

    let html_result = {MenuFormatter{}}.get_legacy_menu(EntityFactory::create_vector_of_patients(), sess.clone());
    assert!(html_result.contains(&"menuNotCurrent".to_string()), "MenuFormatter::get_legacy_menu() produced incorrect HTML");

    let html_result = {MenuFormatter{}}.get_legacy_menu_with_patient(EntityFactory::create_vector_of_patients(), 5, sess.clone());
    assert!(html_result.contains(&"menuNotCurrent".to_string()), "MenuFormatter::get_legacy_menu_with_patient() produced incorrect HTML");

    //let perm: Permission = Permission::new(1, 1);
    //let perm2: Permission = Permission::new(1, 2);
    sess.user_authorizations.granted_permissions.remove(2); // reduced permissions
    //sess.user_authorizations.granted_permissions = vec![perm, perm2]; // reduced permissions
    sess.email = "test3@gmail.com".to_string();
    let html_result = {MenuFormatter{}}.get_legacy_menu_with_patient(EntityFactory::create_vector_of_patients(), 5, sess.clone());
    assert!(html_result.contains(&"menuNotCurrent".to_string()), "MenuFormatter::get_legacy_menu_with_patient() produced incorrect HTML");
}

#[test]
fn test_feature_pref_formatter() {
    
    //FeaturePreferenceFormatter 
    //get_feature_preference_section
}