///
/// Unit & Integration tests for the User Interface (ui) module
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
///  CSM500 Project (April - October 2026)
///  Graham Parker (Student ID: 240120522)
/// -------------------------------------------------------------------

mod common;

use std::ops::IndexMut;

#[cfg(test)]
use maple_hms::constants;
use maple_hms::dto::encounter::Encounter;
use maple_hms::dto::user_auth::Permission;

use maple_hms::ui::tile_factory::{WebContentFactory, WebContentItem};
use maple_hms::ui::simple_fmt::SimpleFormatter;
use maple_hms::ui::common_fmt::CommonFormatter;
use maple_hms::ui::menu_fmt::MenuFormatter;
use maple_hms::ui::feat_preference_fmt::FeaturePreferenceFormatter;
use maple_hms::ui::intervention_fmt::InterventionFormatter;
use maple_hms::session::{UserSession}; // AppSession
use maple_hms::ui::data_forms::InterventionDataForm;

use common::entity_factory::EntityFactory;
use common::data_generator::DataGenerator;

///
/// Tests WebContentFactory::new() and general initalization
/// 
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
    let mut sess: UserSession = EntityFactory::create_user_session();
    let intv_type_list = EntityFactory::create_intervention_type_list("ITEM".to_string(), 2);
    
    // Test 1: user_session.has_permission(Permission::ALLOW_CREATE_UPDATE_DISCHARGE) == false
    let html_results1 = wcf.get_patient_details_full_tile(
        "patient_header".to_string(),
        "current_encounter".to_string(),
        "encounter_section".to_string(),
        sess.clone(), 
        "legacy_menu".to_string(),
        "intv_section".to_string(),
        intv_type_list.clone(),
        "feature_pref_section".to_string(),
        pw.patient.id.to_string(),
        pw.current_encounter.id.to_string()
    );
    assert_ne!(html_results1, "".to_string(), "Test 1: No HTML returned by: get_patient_details_full_tile()");

    sess.user_authorizations.granted_permissions.push( Permission::new(1, Permission::ALLOW_CREATE_UPDATE_DISCHARGE) );
    // Test 2: user_session.has_permission(Permission::ALLOW_CREATE_UPDATE_DISCHARGE) == True
    let html_results2 = wcf.get_patient_details_full_tile(
        "patient_header".to_string(),
        "current_encounter".to_string(),
        "encounter_section".to_string(),
        sess.clone(), 
        "legacy_menu".to_string(),
        "intv_section".to_string(),
        intv_type_list.clone(),
        "feature_pref_section".to_string(),
        pw.patient.id.to_string(),
        pw.current_encounter.id.to_string()
    );
    assert_ne!(html_results2, "".to_string(), "Test 2: No HTML returned by: get_patient_details_full_tile()");

    // Test 3: user_session.has_permission(Permission::ALLOW_CREATE_NON_CLINICAL_INTERVENTION) ONLY
    let perm1 = Permission::new(1, Permission::ALLOW_CREATE_NON_CLINICAL_INTERVENTION);
    sess.user_authorizations.granted_permissions = vec![perm1];
    let html_results3 = wcf.get_patient_details_full_tile(
        "patient_header".to_string(),
        "current_encounter".to_string(),
        "encounter_section".to_string(),
        sess.clone(), 
        "legacy_menu".to_string(),
        "intv_section".to_string(),
        intv_type_list.clone(),
        "feature_pref_section".to_string(),
        pw.patient.id.to_string(),
        pw.current_encounter.id.to_string()
    );
    assert_ne!(html_results3, "".to_string(), "Test 3: No HTML returned by: get_patient_details_full_tile()");

    // Test 4: user_session does not have ALLOW_CREATE_CLINICAL_INTERVENTION or ALLOW_CREATE_NON_CLINICAL_INTERVENTION
    sess.user_authorizations.granted_permissions = Vec::new();
    let html_results3 = wcf.get_patient_details_full_tile(
        "patient_header".to_string(),
        "current_encounter".to_string(),
        "encounter_section".to_string(),
        sess.clone(), 
        "legacy_menu".to_string(),
        "intv_section".to_string(),
        intv_type_list.clone(),
        "feature_pref_section".to_string(),
        pw.patient.id.to_string(),
        pw.current_encounter.id.to_string()
    );
    assert_ne!(html_results3, "".to_string(), "Test 4: No HTML returned by: get_patient_details_full_tile()");
}

///
/// Tests WebContentFactory::test_wcf_get_admit_discharge_full_tile()
/// 
#[test]
fn test_wcf_get_admit_discharge_full_tile() {
    let path = std::env::current_dir().expect("Base path to executable could not be found");
    let newpath = path.display().to_string() + "\\webc\\static\\";
    let wcf = WebContentFactory::new(&newpath, "fake user".to_string());

    let mut tmp_patient = EntityFactory::create_patient();

    // Test 1: Actual Patient, is_discharge_flag = false
    let html_results1 = wcf.get_admit_discharge_full_tile(
       "UNIT TEST, USER".to_string(), // user_identity_label
       Some(tmp_patient.clone()), // current_patient
       "LEGACY-MENU".to_string(), // legacy_menu
       "LOCATION-MENU".to_string(), // location_menu
       false, // is_discharge_flag
       "USER-PROMPT".to_string() // user_prompt
    );
    assert_ne!(html_results1, "".to_string(), "Test 1: No HTML returned by: get_admit_discharge_full_tile()");

    // Test 1a: Patient has existing admit notes
    tmp_patient.admit_notes = "UNIT TEST Admit Notes".to_string();
    let html_results2 = wcf.get_admit_discharge_full_tile(
       "UNIT TEST, USER".to_string(), // user_identity_label
       Some(tmp_patient.clone()), // current_patient
       "LEGACY-MENU".to_string(), // legacy_menu
       "LOCATION-MENU".to_string(), // location_menu
       false, // is_discharge_flag
       "USER-PROMPT".to_string() // user_prompt
    );
    assert_ne!(html_results2, "".to_string(), "Test 1: No HTML returned by: get_admit_discharge_full_tile()");

    // Test 2: Actual Patient, is_discharge_flag = true
    let html_results3 = wcf.get_admit_discharge_full_tile(
       "UNIT TEST, USER".to_string(), // user_identity_label
       Some(tmp_patient.clone()), // current_patient
       "LEGACY-MENU".to_string(), // legacy_menu
       "LOCATION-MENU".to_string(), // location_menu
       true, // is_discharge_flag
       "USER-PROMPT".to_string() // user_prompt
    );
    assert_ne!(html_results3, "".to_string(), "Test 2: No HTML returned by: get_admit_discharge_full_tile()");

    // Test 2b: Actual Patient, is_discharge_flag = true, Admit notes="", request prompt <> ""
    tmp_patient.admit_notes = String::new();
    let html_results4 = wcf.get_admit_discharge_full_tile(
       "UNIT TEST, USER".to_string(), // user_identity_label
       Some(tmp_patient.clone()), // current_patient
       "LEGACY-MENU".to_string(), // legacy_menu
       "LOCATION-MENU".to_string(), // location_menu
       true, // is_discharge_flag
       "USER-PROMPT".to_string() // user_prompt
    );
    assert_ne!(html_results4, "".to_string(), "Test 2: No HTML returned by: get_admit_discharge_full_tile()");

    tmp_patient.admit_notes = "UNIT TEST Admit Notes".to_string();

    // Test 2a: Patient has existing discharge notes
    tmp_patient.discharge_notes = "UNIT TEST Discharge Notes".to_string();
    let html_results5 = wcf.get_admit_discharge_full_tile(
       "UNIT TEST, USER".to_string(), // user_identity_label
       Some(tmp_patient.clone()), // current_patient
       "LEGACY-MENU".to_string(), // legacy_menu
       "LOCATION-MENU".to_string(), // location_menu
       true, // is_discharge_flag
       "USER-PROMPT".to_string() // user_prompt
    );
    assert_ne!(html_results5, "".to_string(), "Test 2: No HTML returned by: get_admit_discharge_full_tile()");
    
    // reset for the rest of the tests
    tmp_patient.admit_notes = String::new();
    tmp_patient.discharge_notes = String::new();

    // Test 2b: Patient has existing admit notes
    tmp_patient.admit_notes = "UNIT TEST Admit Notes".to_string();
    let html_results6 = wcf.get_admit_discharge_full_tile(
       "UNIT TEST, USER".to_string(), // user_identity_label
       Some(tmp_patient.clone()), // current_patient
       "LEGACY-MENU".to_string(), // legacy_menu
       "LOCATION-MENU".to_string(), // location_menu
       true, // is_discharge_flag
       "USER-PROMPT".to_string() // user_prompt
    );
    assert_ne!(html_results6, "".to_string(), "Test 2: No HTML returned by: get_admit_discharge_full_tile()");

    // Test 3: None Patient, is_discharge_flag = false
    let html_results7 = wcf.get_admit_discharge_full_tile(
       "UNIT TEST, USER".to_string(), // user_identity_label
       None, // current_patient
       "LEGACY-MENU".to_string(), // legacy_menu
       "LOCATION-MENU".to_string(), // location_menu
       false, // is_discharge_flag
       "USER-PROMPT".to_string() // user_prompt
    );
    assert_ne!(html_results7, "".to_string(), "Test 3: No HTML returned by: get_admit_discharge_full_tile()");

    // Test 4: None Patient, is_discharge_flag = true
    let html_results8 = wcf.get_admit_discharge_full_tile(
       "UNIT TEST, USER".to_string(), // user_identity_label
       None, // current_patient
       "LEGACY-MENU".to_string(), // legacy_menu
       "LOCATION-MENU".to_string(), // location_menu
       true, // is_discharge_flag
       "USER-PROMPT".to_string() // user_prompt
    );
    assert_ne!(html_results8, "".to_string(), "Test 4: No HTML returned by: get_admit_discharge_full_tile()");
}

///
/// Tests WebContentFactory::get_modify_intervention_full_tile()
/// 
#[test]
fn test_get_modify_intervention_full_tile() {
    let path = std::env::current_dir().expect("Base path to executable could not be found");
    let newpath = path.display().to_string() + "\\webc\\static\\";
    let wcf = WebContentFactory::new(&newpath, "fake user".to_string());

    let tmp_intv = EntityFactory::create_intervention();
    let intv_type_list = EntityFactory::create_intervention_type_list("ITEM".to_string(), 2);
    let intvdtls_list = EntityFactory::create_vector_of_intervention_details();
    let measures_dropdown_list = EntityFactory::create_intervention_type_list("INTV-DTLS".to_string(), 2);
    let user_dropdown_list = EntityFactory::create_intervention_type_list("USER-NAME".to_string(), 2);
    let status_dropdown_list = EntityFactory::create_intervention_type_list("INTV-STATUS".to_string(), 2);

    let mut tmp_frm: InterventionDataForm = InterventionDataForm {    
        ..Default::default()
    };

    // Test 1: Some() current Intervention, Some() <Vec<InterventionDetail>
    let html_results = wcf.get_modify_intervention_full_tile(
        "UNIT TEST, USER".to_string(), 
        Some(tmp_intv.clone()), 
        "LEGACY-MENU".to_string(), 
        user_dropdown_list.clone(),
        status_dropdown_list.clone(),
        "LOCATION-MENU".to_string(), 
        intv_type_list[0].clone(), 
        tmp_frm.clone(),
        Some(intvdtls_list.clone()),
        measures_dropdown_list.clone(),
        "feature_pref_section".to_string() 
    );
    assert_ne!(html_results, "".to_string(), "Test 1: No HTML returned by: get_modify_intervention_full_tile()");

    // Test 1a: Some() current Intervention, Some() <Vec<InterventionDetail>; form_errors exist
    tmp_frm.form_errors = "FORM ERRORS".to_string();
    let html_results = wcf.get_modify_intervention_full_tile(
        "UNIT TEST, USER".to_string(), 
        Some(tmp_intv.clone()), 
        "LEGACY-MENU".to_string(), 
        user_dropdown_list.clone(),
        status_dropdown_list.clone(),
        "LOCATION-MENU".to_string(), 
        intv_type_list[0].clone(), 
        tmp_frm.clone(),
        Some(intvdtls_list.clone()),
        measures_dropdown_list.clone(),
        "feature_pref_section".to_string() 
    );
    assert_ne!(html_results, "".to_string(), "Test 1a: No HTML returned by: get_modify_intervention_full_tile()");

    // Test 2: Some() current Intervention, None <Vec<InterventionDetail>
    let html_results2 = wcf.get_modify_intervention_full_tile(
        "UNIT TEST, USER".to_string(), 
        Some(tmp_intv.clone()), 
        "LEGACY-MENU".to_string(), 
        user_dropdown_list.clone(),
        status_dropdown_list.clone(),
        "LOCATION-MENU".to_string(), 
        intv_type_list[0].clone(), 
        tmp_frm.clone(),
        None,
        measures_dropdown_list.clone(),
        "feature_pref_section".to_string() 
    );
    assert_ne!(html_results2, "".to_string(), "Test 2: No HTML returned by: get_modify_intervention_full_tile()");

    // Test 3: None current Intervention, Some() <Vec<InterventionDetail>
    let html_results3 = wcf.get_modify_intervention_full_tile(
        "UNIT TEST, USER".to_string(), 
        None, 
        "LEGACY-MENU".to_string(), 
        user_dropdown_list.clone(),
        status_dropdown_list.clone(),
        "LOCATION-MENU".to_string(), 
        intv_type_list[0].clone(), 
        tmp_frm.clone(),
        Some(intvdtls_list),
        measures_dropdown_list.clone(),
        "feature_pref_section".to_string() 
    );
    assert_ne!(html_results3, "".to_string(), "Test 3: No HTML returned by: get_modify_intervention_full_tile()");

    // Test 4: None current Intervention, None <Vec<InterventionDetail
    let html_results4 = wcf.get_modify_intervention_full_tile(
        "UNIT TEST, USER".to_string(), 
        None, 
        "LEGACY-MENU".to_string(), 
        user_dropdown_list.clone(),
        status_dropdown_list.clone(),
        "LOCATION-MENU".to_string(), 
        intv_type_list[0].clone(), 
        tmp_frm.clone(),
        None,
        measures_dropdown_list.clone(),
        "feature_pref_section".to_string() 
    );
    assert_ne!(html_results4, "".to_string(), "Test 4: No HTML returned by: get_modify_intervention_full_tile()");
}

///
/// Tests methods of the SimpleFormatter struct
/// 
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

    //Test 3a: get_single_patient_summary with an aged admit_timestamp 
    let mut pcw = EntityFactory::create_patient_wrapper();
    pcw.patient.admit_timestamp = DataGenerator::get_date(); // all dates provided are now > 4hrs old
    pcw.most_recent_intervention = None; // also test the "is_some()" condition for the method
    results = SimpleFormatter::get_single_patient_summary(pcw, 1);
    assert!( results.contains("Admission Concern") , "..formatting incomplete"); // from the static part of the content

    //Test 3b: get_single_patient_summary with a NEW (not aged) admit_timestamp 
    let mut pcw2 = EntityFactory::create_patient_wrapper();
    pcw2.patient.admit_timestamp = DataGenerator::now();
    pcw2.most_recent_intervention = None; // also test the "is_some()" condition for the method 
    results = SimpleFormatter::get_single_patient_summary(pcw2.clone(), 1);
    assert!( results.contains("Admission Concern") , "..formatting incomplete"); // from the static part of the content

    //Test 3c: get_single_patient_summary with a  -1 index
    results = SimpleFormatter::get_single_patient_summary(pcw2, -1);
    assert!( results.contains("Admission Concern") , "..formatting incomplete"); // from the static part of the content

    // Test 4: get_home_route_summary_of_patients_tile_using_wrapper()
    results = SimpleFormatter::get_home_route_summary_of_patients_tile_using_wrapper(EntityFactory::create_vector_of_patient_wrappers() );
    assert!( results.contains("patientDtlsFrm") , "..formatting incomplete"); // from the static part of the content
    //assert!( results.contains( &search_string ) , "..formatting incomplete"); // from the data

    // Test 5: get_encounter_list_tile()
    results = SimpleFormatter::get_encounter_list_tile(EntityFactory::create_vector_of_encounters());
    assert!( results.contains("encounterDtlsFrm") , "..formatting incomplete"); // from the static part of the content
    //assert!( results.contains( &search_string ) , "..formatting incomplete"); // from the data

    // Test 6: create_vector_of_interventions()
    let mut lst = EntityFactory::create_vector_of_interventions();
    lst.index_mut(0).intervention_type_id = constants::CRT_INTERVENTION_ALERT_TYPE;
    lst.index_mut(5).intervention_type_id = 100052; // this is a clinical type
    lst.index_mut(3).intervention_type_id = 100006; // this is Patient Transfer, a non-clinical type

    // Test 7: get_intervention_list_for_patient_details_tile() - non-clinical records only
    results = SimpleFormatter::get_intervention_list_for_patient_details_tile(lst.clone(), false);
    assert!( results.contains("intvDtlsFrm") , "..formatting incomplete"); // from the static part of the content

    // Test 7b: secondary check that the user can view CLINICAL records
    results = SimpleFormatter::get_intervention_list_for_patient_details_tile(lst.clone(), true);
    assert!( results.contains("intvDtlsFrm") , "..formatting incomplete"); // from the static part of the content
}

///
/// Tests methods of the CommonFormatter struct
/// 
#[test]
fn test_common_formatter() {
    let items = EntityFactory::create_intervention_type_list("ITEM".to_string(), 2);

    let html_test1 = CommonFormatter::get_dropdown_generic(items.clone(), "dummy_id".to_string(), 1);
    assert!(html_test1.contains(&"TEST-ITEM-2".to_string()), "CommonFormatter::get_dropdown_generic() produced incorrect HTML");
    
    let html_test2 = CommonFormatter::get_dropdown_user_with_department(items.clone(), 1);
    assert!(html_test2.contains(&"TEST-ITEM-2".to_string()), "CommonFormatter::get_dropdown_user_with_department() produced incorrect HTML");
    
    let html_test3 = CommonFormatter::get_dropdown_intervention_status(items.clone(), 1);
    assert!(html_test3.contains(&"TEST-ITEM-2".to_string()), "CommonFormatter::get_dropdown_intervention_status() produced incorrect HTML");

    let items2 = EntityFactory::create_location_list( 2);
    let html_test4 = CommonFormatter::get_location_dropdown(items2.clone(), 1);
    assert!(html_test4.contains(&"UNIT TEST-Location-2".to_string()), "CommonFormatter::get_location_dropdown() produced incorrect HTML");
}

///
/// Tests methods of the MenuFormatter struct
/// 
#[test]
fn test_menu_formatter() {
    let mut sess: UserSession = EntityFactory::create_user_session();

    let html_test1 = {MenuFormatter{}}.get_legacy_menu(EntityFactory::create_vector_of_patients(), sess.clone());
    assert!(html_test1.contains(&"menuNotCurrent".to_string()), "MenuFormatter::get_legacy_menu() produced incorrect HTML");

    let html_test2 = {MenuFormatter{}}.get_legacy_menu_with_patient(EntityFactory::create_vector_of_patients(), 5, sess.clone());
    assert!(html_test2.contains(&"menuNotCurrent".to_string()), "MenuFormatter::get_legacy_menu_with_patient() produced incorrect HTML");

    //let perm: Permission = Permission::new(1, 1);
    //let perm2: Permission = Permission::new(1, 2);
    sess.user_authorizations.granted_permissions.remove(2); // reduced permissions
    //sess.user_authorizations.granted_permissions = vec![perm, perm2]; // reduced permissions
    sess.email = "test3@gmail.com".to_string(); 
    let html_test3 = {MenuFormatter{}}.get_legacy_menu_with_patient(EntityFactory::create_vector_of_patients(), 5, sess.clone());
    assert!(html_test3.contains(&"menuNotCurrent".to_string()), "MenuFormatter::get_legacy_menu_with_patient() produced incorrect HTML");
}

///
/// Tests methods of the FeaturePreferenceFormatter struct
/// 
#[test]
fn test_feature_pref_formatter() {
    let mut fp_list = EntityFactory::create_vector_of_feature_preferences();

    fp_list[0].ref_group_id = constants::CRT_CLINICAL_INTERVENTION_GRP_ID; // for test 2.1
    fp_list[1].ref_group_id = constants::CRT_NON_CLINICAL_INTERVENTION_GRP_ID; // for test 2.2
    fp_list[2].ref_group_id = 100001; // for test 2.3

    // Test 1: list == None situation (quick win)
    let html_test1 = FeaturePreferenceFormatter::get_feature_preference_section(None);
    assert_eq!(html_test1, String::new(), "FeaturePreferenceFormatter produced HTML; none expected");

    // Test 2: Basic test
    let html_test2 = FeaturePreferenceFormatter::get_feature_preference_section(Some(fp_list));

    // should always contain the button code
    assert!(html_test2.contains(&"fast_action_btn_id_".to_string()), "FeaturePreferenceFormatter produced incorrect HTML: missing fast_action_btn_id_");

    // test 2.1: item.ref_group_id == constants::CRT_CLINICAL_INTERVENTION_GRP_ID => "fast_action_add_intv"
    // test 2.2: item.ref_group_id == constants::CRT_NON_CLINICAL_INTERVENTION_GRP_ID => "fast_action_add_intv"
    assert!(html_test2.contains(&"fast_action_add_intv".to_string()), "FeaturePreferenceFormatter produced incorrect HTML: missing fast_action_add_intv");

    // test 2.3: item.ref_group_id == anything other than the prior two => "fast_action_add_measure"
    assert!(html_test2.contains(&"fast_action_add_measure".to_string()), "FeaturePreferenceFormatter produced incorrect HTML: missing fast_action_add_measure");
    println!("End test: test_feature_pref_formatter()");
}

///
/// Tests methods of the InterventionFormatter struct
/// 
#[test]
fn test_intervention_formatter() {
    let intervention_details_item_tile = "intervention_details_item_tile".to_string();
    let patient_id: i64 = 1;
    let intvdtls_list = EntityFactory::create_vector_of_intervention_details();
    let measures_dropdown_list = EntityFactory::create_intervention_type_list("INTV-DTLS".to_string(), 2);

    // Test 1: intvdtls_list = None
    let html_test1 = InterventionFormatter::get_view_only_intervention_details_list(intervention_details_item_tile.clone(), None, measures_dropdown_list.clone(), patient_id.to_string());
    assert_eq!(html_test1, "No Measures Added".to_string(), "InterventionFormatter produced unexpected HTML");

    // Test 1: 
    let html_test2 = InterventionFormatter::get_view_only_intervention_details_list(intervention_details_item_tile.clone(), Some(intvdtls_list), measures_dropdown_list.clone(), patient_id.to_string());
    assert!(html_test2.contains(""), "InterventionFormatter produced unexpected HTML");
}