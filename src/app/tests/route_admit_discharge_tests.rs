/// -------------------------------------------------------------------
/// #Unit & Integration tests for the Admit and Discharge routes
/// 
/// ###Includes:
/// * AdmitRoute::route_to_admit_new_no_patient()
/// * AdmitRoute::route_to_discharge_patient()
/// * AdmitRoute::route_to_admit_discharge()
/// * AdmitRoute::route_to_discharge_patient_save()
/// * AdmitRoute::route_to_admit_save()
/// 
///  CSM500 Project (April - October 2026)
///  Graham Parker (Student ID: 240120522)
/// -------------------------------------------------------------------

mod common;

#[cfg(test)]

use maple_hms::constants;
use maple_hms::route::admit_route::*;
//use maple_hms::session::*;
use maple_hms::ui::data_forms::*;
//use maple_hms::dto::user_auth::*;

use actix_session::SessionExt;
use actix_web::{body::to_bytes, http::StatusCode, test, web, Responder};
use common::entity_factory::EntityFactory;

const ADMIT_SCREEN_ID_TAG_1 : &str = "<div id=\"MapleHMS::ID=Admit\"></div>";

/// ### test_route_to_admit_new_no_patient()
/// 
/// Tests:
///   AdmitRoute::route_to_admit_new_no_patient()
/// 
#[actix_web::test]
async fn test_route_to_admit_new_no_patient(){
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test
    let app_session = EntityFactory::get_mock_app_session().await; // create app session

    let frm = AdmitFormBasic {
        patient_id: "-1".to_string(),
        user_prompt: "1".to_string(),
    };
    //AdmitRoute::route_to_admit_new_no_patient(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<AdmitFormBasic>)
    // Test 1: AdmitRoute::route_to_admit_new_no_patient
    println!("Test 1: AdmitRoute::route_to_admit_new_no_patient");
    let responder = AdmitRoute::route_to_admit_new_no_patient( app_session.clone(), user_session.clone(),  web::Form( frm.clone() ) ).await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp.into_body()).await{
        Ok(item) => {
            //println!("Body = {}", String::from_utf8_lossy(&item));
            // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
            assert!( String::from_utf8_lossy(&item).contains( ADMIT_SCREEN_ID_TAG_1 ), "Response did not contain expected content"); 
        },
        Err(_e) => assert!( false, "Error Response received" ),
    };
}

/// ### test_route_to_discharge_patient()
/// 
/// Tests:
///   AdmitRoute::test_route_to_discharge_patient()
/// 
#[actix_web::test]
async fn test_route_to_discharge_patient(){ 
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test
    let app_session = EntityFactory::get_mock_app_session().await; // create app session

    let frm = AdmitFormBasic {
        patient_id: "-1".to_string(),
        user_prompt: "1".to_string(),
    };
    //AdmitRoute::route_to_discharge_patient(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<AdmitFormBasic>)
    // Test 1: AdmitRoute::route_to_discharge_patient
    println!("Test 1: AdmitRoute::route_to_discharge_patient");
    let responder = AdmitRoute::route_to_discharge_patient( app_session.clone(), user_session.clone(), web::Form( frm.clone() ) ).await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp.into_body()).await{
        Ok(item) => {
            //println!("Body = {}", String::from_utf8_lossy(&item));
            const DISCHARGE_SCREEN_TAG_1 : &str = "id=\"disclaimerChk\"";
            const DISCHARGE_SCREEN_TAG_2 : &str = "id=\"patient_id\" value=\"-1\"";
            const DISCHARGE_SCREEN_TAG_3 : &str = "<form action=\"/dischargesave\" method=\"post\" id=\"dischargeForm\"";
            const DISCHARGE_SCREEN_TAG_4 : &str = "<div id=\"MapleHMS::ID=Discharge\"></div>";

            // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
            assert!( String::from_utf8_lossy(&item).contains( DISCHARGE_SCREEN_TAG_1 ), "Response 1 did not contain expected content"); 
            assert!( String::from_utf8_lossy(&item).contains( DISCHARGE_SCREEN_TAG_2 ), "Response 2 did not contain expected content"); 
            assert!( String::from_utf8_lossy(&item).contains( DISCHARGE_SCREEN_TAG_3 ), "Response 3 did not contain expected content"); 
            assert!( String::from_utf8_lossy(&item).contains( DISCHARGE_SCREEN_TAG_4 ), "Response 4 did not contain expected content"); 
        },
        Err(_e) => assert!( false, "Error Response received" ),
    };
}

/// ### test_route_to_admit_discharge()
/// 
/// Tests:
///   AdmitRoute::route_to_admit_discharge()
/// 
#[actix_web::test]
async fn test_route_to_admit_discharge(){
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test
    let app_session = EntityFactory::get_mock_app_session().await; // create app session

    let frm_admit_test_1 = AdmitDataForm {
        patient_id: "-1".to_string(),
        user_prompt: "1".to_string(),
        action_flag: "admit".to_string(),
        admit_notes: "admitting UNIT TEST patient".to_string(),
	    ..Default::default()
    };

    //AdmitRoute::route_to_admit_discharge(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<AdmitDataForm>)
    // Test 1: AdmitRoute::route_to_admit_discharge
    println!("Test 1: AdmitRoute::route_to_admit_discharge");
    let responder = AdmitRoute::route_to_admit_discharge( app_session.clone(), user_session.clone(), web::Form( frm_admit_test_1.clone() ) ).await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp.into_body()).await{
        Ok(item) => {
            //println!("Body = {}", String::from_utf8_lossy(&item));
            // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
            assert!( String::from_utf8_lossy(&item).contains( ADMIT_SCREEN_ID_TAG_1 ), "Response did not contain expected content"); 
        },
        Err(_e) => assert!( false, "Error Response received" ),
    };

    // make some forms
    let frm_discharge_test_1 = AdmitDataForm {
        patient_id: "-1".to_string(),
        user_prompt: String::new(),
        action_flag: "discharge".to_string(),
        admit_notes: "admitting UNIT TEST patient".to_string(),
	    ..Default::default()
    };

    let frm_discharge_test_2 = AdmitDataForm {
        patient_id: "305".to_string(),
        user_prompt: String::new(),
        action_flag: "discharge".to_string(),
        admit_notes: "admitting UNIT TEST patient".to_string(),
	    ..Default::default()
    };

    let frm_discharge_test_3 = AdmitDataForm {
        patient_id: "188".to_string(),
        user_prompt: "discharge this person".to_string(),
        action_flag: "discharge".to_string(),
        admit_notes: "admitting UNIT TEST patient".to_string(),
	    ..Default::default()
    };

    let frm_discharge_test_4 = AdmitDataForm {
        patient_id: "305".to_string(),
        user_prompt: "discharge this person".to_string(),
        action_flag: "discharge".to_string(),
        admit_notes: "admitting UNIT TEST patient".to_string(),
	    ..Default::default()
    };

    let frm_discharge_test_5 = AdmitDataForm {
        patient_id: "305".to_string(),
        user_prompt: "discharge this person".to_string(),
        action_flag: "discharge".to_string(),
        admit_notes: "admitting UNIT TEST patient".to_string(),
	    ..Default::default()
    };

    const VALIDATION_STRING_TEST_1 : &str  = "<div id=\"MapleHMS::ID=Discharge\"></div>";
    const VALIDATION_STRING_TEST_2 : &str  = VALIDATION_STRING_TEST_1;
    const VALIDATION_STRING_TEST_3 : &str  = VALIDATION_STRING_TEST_1;
    const VALIDATION_STRING_TEST_4 : &str  = VALIDATION_STRING_TEST_1;
    const VALIDATION_STRING_TEST_5 : &str  = VALIDATION_STRING_TEST_1;
    
    // we're going to loop through them as a vector of tuples. The tuple will be the form and a string to validate
    let all_forms = vec![("Test 1", frm_discharge_test_1.clone(), VALIDATION_STRING_TEST_1),
                                                        ("Test 2", frm_discharge_test_2.clone(), VALIDATION_STRING_TEST_2),
                                                        ("Test 3", frm_discharge_test_3.clone(), VALIDATION_STRING_TEST_3),
                                                        ("Test 4", frm_discharge_test_4.clone(), VALIDATION_STRING_TEST_4),
                                                        ("Test 5", frm_discharge_test_5.clone(), VALIDATION_STRING_TEST_5),
                                                       ];
    for cur_frm in all_forms{
        println!("{}: AdmitRoute::route_to_admit_discharge", cur_frm.0 );
        let responder = AdmitRoute::route_to_admit_discharge( app_session.clone(), user_session.clone(), web::Form( cur_frm.1 ) ).await;
        let http_resp = responder.respond_to(&req);
        assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");
        match to_bytes(http_resp.into_body()).await{
            Ok(item) => {
                //println!("Body = {}", String::from_utf8_lossy(&item));
                // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
                assert!( String::from_utf8_lossy(&item).contains( cur_frm.2 ), "Response did not contain expected content"); 
            },
            Err(_e) => assert!( false, "Error Response received" ),
        };
    }
}


/// ### test_route_to_admit_save()
/// 
/// Tests:
///   AdmitRoute::route_to_admit_save()
/// 
#[actix_web::test]
async fn test_route_to_admit_save(){ 
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test
    let app_session = EntityFactory::get_mock_app_session().await; // create app session

    let frm_test_1 = AdmitDataForm {
        patient_id: "-1".to_string(),
        patient_first_name: "UNIT-FIRST-NAME".to_string(), // invalid, PHN is not 10 digits
        patient_last_name: "UNIT-LAST-NAME".to_string(),
        patient_middle_name: "UNIT-MIDDLE-NAME".to_string(),
        user_prompt: "1".to_string(),
        action_flag: "admit".to_string(),
        admit_notes: "UPDATED UNIT TEST PATIENT".to_string(),
	    ..Default::default()
    };

    let frm_test_2 = AdmitDataForm { // valid form
        patient_id: "34".to_string(),
        patient_first_name: "UNIT-FIRST-NAME".to_string(),
        patient_last_name: "UNIT-LAST-NAME".to_string(),
        patient_middle_name: "UNIT-MIDDLE-NAME".to_string(),
        user_prompt: "1".to_string(),
        phn: "9123123123".to_string(),
        birthdate: "2026-JUN-01".to_string(),
        form_errors: "".to_string(),
        location_id: "12".to_string(),
        encounter_id: "104".to_string(),
        action_flag: "admit".to_string(),
        admit_notes: "UPDATED UNIT TEST PATIENT".to_string(),
	    ..Default::default()
    };

    let frm_test_3 = AdmitDataForm { // invalid encounter_id
        patient_id: "34".to_string(),
        patient_first_name: "UNIT-FIRST-NAME".to_string(),
        patient_last_name: "UNIT-LAST-NAME".to_string(),
        patient_middle_name: "UNIT-MIDDLE-NAME".to_string(),
        user_prompt: "1".to_string(),
        phn: "9123123123".to_string(),
        birthdate: "2026-JUN-01".to_string(),
        form_errors: "".to_string(),
        location_id: "12".to_string(),
        encounter_id: "999999".to_string(),
        action_flag: "admit".to_string(),
        admit_notes: "UPDATED UNIT TEST PATIENT".to_string(),
	    ..Default::default()
    };

    let frm_test_4 = AdmitDataForm { // invalid patient_id and encounter_id
        patient_id: "9999999".to_string(),
        patient_first_name: "UNIT-FIRST-NAME".to_string(),
        patient_last_name: "UNIT-LAST-NAME".to_string(),
        patient_middle_name: "UNIT-MIDDLE-NAME".to_string(),
        user_prompt: "1".to_string(),
        phn: "9123123123".to_string(),
        birthdate: "2026-JUN-01".to_string(),
        form_errors: "".to_string(),
        location_id: "12".to_string(),
        encounter_id: "999999".to_string(),
        action_flag: "admit".to_string(),
        admit_notes: "UPDATED UNIT TEST PATIENT".to_string(),
	    ..Default::default()
    };

    const VALIDATION_STRING_TEST_1 : &str  = "<div id=\"MapleHMS::ID=Admit\"></div>";
    const VALIDATION_STRING_TEST_2 : &str  = VALIDATION_STRING_TEST_1;
    const VALIDATION_STRING_TEST_3 : &str  = VALIDATION_STRING_TEST_1;
    const VALIDATION_STRING_TEST_4 : &str  = VALIDATION_STRING_TEST_1;
    
    // we're going to loop through them as a vector of tuples. The tuple will be the form and a string to validate
    let all_forms = vec![("Test 1", frm_test_1.clone(), VALIDATION_STRING_TEST_1),
                                                        ("Test 2", frm_test_2.clone(), VALIDATION_STRING_TEST_2),
                                                        ("Test 3", frm_test_3.clone(), VALIDATION_STRING_TEST_3),
                                                        ("Test 4", frm_test_4.clone(), VALIDATION_STRING_TEST_4),
                                                      ];
    for cur_frm in all_forms{
        // Test 1: AdmitRoute::route_to_admit_save
        println!("{} AdmitRoute::route_to_admit_save", cur_frm.0);
        let responder = AdmitRoute::route_to_admit_save( app_session.clone(), user_session.clone(), web::Form( cur_frm.1.clone() ) ).await;
        let http_resp = responder.respond_to(&req);
        assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");
        match to_bytes(http_resp.into_body()).await{
            Ok(item) => {
                //println!("Body = {}", String::from_utf8_lossy(&item));
                // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
                assert!( String::from_utf8_lossy(&item).contains( cur_frm.2 ), "Response did not contain expected content"); 
            },
            Err(_e) => assert!( false, "Error Response received" ),
        };
    }
}

/// ### test_route_to_discharge_patient_save()
/// 
/// Tests:
///   AdmitRoute::route_to_discharge_patient_save()
/// 
#[actix_web::test]
async fn test_route_to_discharge_patient_save(){
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test
    let app_session = EntityFactory::get_mock_app_session().await; // create app session

    let frm_test_1 = DischargeDataForm {
        patient_id: "-1".to_string(),
        encounter_id: "-1".to_string(),
        discharge_notes: "UPDATED UNIT TEST PATIENT".to_string(),
	    ..Default::default()
    };
    
    println!("Test 1: AdmitRoute::route_to_discharge_patient_save");
    let responder = AdmitRoute::route_to_discharge_patient_save( app_session.clone(), user_session.clone(), web::Form( frm_test_1.clone() ) ).await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::SEE_OTHER, "Status not SEE_OTHER");
}