/// -------------------------------------------------------------------
/// #Unit & Integration tests for Intervention routes
/// 
/// ###Includes:
/// * InterventionRoute::route_to_modify_intervention_basic(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<InterventionDataFormLink>)
/// * InterventionRoute::route_to_add_new_intervention(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<InterventionDataFormBasic>)
/// * InterventionRoute::route_to_view_or_modify_intervention(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<InterventionDataForm>)
/// * InterventionRoute::route_to_intervention_save(app_session: web::Data<AppSession>, user_session: Session, mut req: web::Form<InterventionDataForm>) 
/// 
///  CSM500 Project (April - October 2026)
///  Graham Parker (Student ID: 240120522)
/// -------------------------------------------------------------------

mod common;

#[cfg(test)]

use maple_hms::constants;
use maple_hms::route::intervention_route::*; // InterventionRoute
use maple_hms::ui::data_forms::*;

use actix_session::SessionExt;
use actix_web::{body::to_bytes, http::StatusCode, test, web, Responder};
use common::entity_factory::EntityFactory;

/// ### test_route_to_modify_intervention_basic()
/// 
/// Tests:
///   InterventionRoute::route_to_modify_intervention_basic()
///
#[actix_web::test]
async fn test_route_to_modify_intervention_basic(){ 
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test

    
    let app_session = EntityFactory::get_mock_app_session().await; // create app session

    let frm_test_1 = InterventionDataFormLink {
        patient_id: "-1".to_string(),
        intervention_id: "1".to_string(),
        encounter_id: "1".to_string(),
	    //..Default::default()
    };

    const VALIDATION_STRING_TEST_1 : &str  = "<div id=\"MapleHMS::ID=InterventionDetails\"></div>";

    // we're going to loop through them as a vector of tuples. The tuple will be the form and a string to validate
    let all_forms = vec![("Test 1", frm_test_1.clone(), VALIDATION_STRING_TEST_1),];
    for cur_frm in all_forms{
        println!("{} InterventionRoute::route_to_modify_intervention_basic", cur_frm.0);
        let responder = InterventionRoute::route_to_modify_intervention_basic( app_session.clone(), user_session.clone(), web::Form( cur_frm.1.clone() ) ).await;
        let http_resp = responder.respond_to(&req);
        assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");
        match to_bytes(http_resp.into_body()).await{
            Ok(item) => {
                //println!("{} Body = {}", cur_frm.0, String::from_utf8_lossy(&item));
                // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
                assert!( String::from_utf8_lossy(&item).contains( cur_frm.2 ), "Response did not contain expected content"); 
            },
            Err(_e) => assert!( false, "Error Response received" ),
        };
    }

    let cur_frm = ("Test 1", frm_test_1.clone(), VALIDATION_STRING_TEST_1);
    // TEST 3: Edge case where a user (#9) does not have any locations defined for them, but somehow got into the system
    let _ignore_test3 = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("9".to_string())); // we will ignore error as this is a unit test
    // we dont need to change anything else, just the user; re-run prior code outside of loop
    let responder_test3 = InterventionRoute::route_to_modify_intervention_basic( app_session.clone(), user_session.clone(), web::Form( cur_frm.1) ).await;
    let http_resp_test3 = responder_test3.respond_to(&req);
    assert_eq!(http_resp_test3.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp_test3.into_body()).await{
        Ok(item) => {
            //println!("{} Body = {}", cur_frm.0, String::from_utf8_lossy(&item));
            // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
            assert!( String::from_utf8_lossy(&item).contains( cur_frm.2 ), "Response did not contain expected cm ontent"); 
        },
        Err(_e) => assert!( false, "Error Response received" ),
    };
}

/// ### test_route_to_intervention_save()
/// 
/// Tests:
///   InterventionRoute::route_to_intervention_save()
/// 
#[actix_web::test]
async fn test_route_to_intervention_save(){ 
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test
    let app_session = EntityFactory::get_mock_app_session().await; // create app session

    let frm_test_1 = InterventionDataForm {  // Test 1: constants::INVALID_OTHER_ID
        intervention_id: constants::INVALID_OTHER_ID.to_string(),
        description: "test_route_to_intervention_save()::UNIT TEST1".to_string(),
        notes: "test_route_to_intervention_save()::UNIT TEST1".to_string(),
        location_id: "20".to_string(),
        users_id: "2".to_string(),
        encounter_id: constants::INVALID_OTHER_ID.to_string(), // 188
        intervention_type_id: "100009".to_string(), 
        status_id: "14".to_string(), 
        patient_id: constants::INVALID_OTHER_ID.to_string(), //305
        //scheduled_timestamp: String,
        //performed_timestamp: String, 
        //form_errors: String,
        ..Default::default()
    };

    let frm_test_2 = InterventionDataForm {  // Test 2: valid id
        intervention_id: "150".to_string(),
        description: "test_route_to_intervention_save()::UNIT TEST2".to_string(),
        notes: "test_route_to_intervention_save()::UNIT TEST2".to_string(),
        location_id: "20".to_string(),
        users_id: "2".to_string(),
        encounter_id: "188".to_string(), 
        intervention_type_id: "100009".to_string(),
        status_id: "14".to_string(),
        patient_id: "305".to_string(), 
        ..Default::default()
    };

    let frm_test_3 = InterventionDataForm {  // Test 3: Invalid fields test
        intervention_id: constants::INVALID_OTHER_ID.to_string(),
        description: "test_route_to_intervention_save()::UNIT TEST1".to_string(),
        notes: "test_route_to_intervention_save()::UNIT TEST1".to_string(),
        location_id: "".to_string(),// should be a number
        users_id: "".to_string(), // should be a number
        encounter_id: constants::INVALID_OTHER_ID.to_string(), // 188
        intervention_type_id: "100009".to_string(),
        status_id: "".to_string(), // should be a number
        patient_id: constants::INVALID_OTHER_ID.to_string(), //305
        ..Default::default()
    };

    // invalid tests
    const VALIDATION_STRING_TEST_1A: &str  = "<div id=\"MapleHMS::ID=Intervention\"></div>";
    const VALIDATION_STRING_TEST_1B: &str  = "id=\"patient_id\" value=\"-1";
    const VALIDATION_STRING_TEST_1C: &str  = "id=\"intervention_type_id\" value=\"100009";
    const VALIDATION_STRING_TEST_1D: &str  = "id=\"intervention_id\" value=\"-1";
    const VALIDATION_STRING_TEST_1E: &str  = "id=\"encounter_id\" value=\"-1"; 
    const VALIDATION_STRING_TEST_1F: &str  = "id=\"target_id\" value=\"-1"; 
    const VALIDATION_STRING_TEST_1G: &str  = "id=\"form_errors\" value=\"\""; 

    const VALIDATION_STRING_TEST_3A: &str  = "id=\"form_errors\" value=\"\""; 

    // valid tests
    const VALIDATION_STRING_TEST_2A: &str  = "<div id=\"MapleHMS::ID=Intervention\"></div>";
    const VALIDATION_STRING_TEST_2B: &str  = "id=\"patient_id\" value=\"305";
    const VALIDATION_STRING_TEST_2C: &str  = "id=\"intervention_type_id\" value=\"100009";
    const VALIDATION_STRING_TEST_2D: &str  = "id=\"intervention_id\" value=\"150";
    const VALIDATION_STRING_TEST_2E: &str  = "id=\"encounter_id\" value=\"188"; 
    const VALIDATION_STRING_TEST_2F: &str  = "id=\"target_id\" value=\"305"; 
    const VALIDATION_STRING_TEST_2G: &str  = "id=\"form_errors\" value=\"\""; 
    
    // we're going to loop through them as a vector of tuples. The tuple will be the form and a string to validate
    let all_forms = vec![("Test 1a", frm_test_1.clone(), VALIDATION_STRING_TEST_1A),
                                                               ("Test 1b", frm_test_1.clone(), VALIDATION_STRING_TEST_1B),
                                                               ("Test 1c", frm_test_1.clone(), VALIDATION_STRING_TEST_1C),
                                                               ("Test 1d", frm_test_1.clone(), VALIDATION_STRING_TEST_1D),
                                                               ("Test 1e", frm_test_1.clone(), VALIDATION_STRING_TEST_1E),
                                                               ("Test 1f", frm_test_1.clone(), VALIDATION_STRING_TEST_1F),
                                                               ("Test 1g", frm_test_1.clone(), VALIDATION_STRING_TEST_1G),
                                                               ("Test 2a", frm_test_2.clone(), VALIDATION_STRING_TEST_2A),
                                                               ("Test 2b", frm_test_2.clone(), VALIDATION_STRING_TEST_2B),
                                                               ("Test 2c", frm_test_2.clone(), VALIDATION_STRING_TEST_2C),
                                                               ("Test 2d", frm_test_2.clone(), VALIDATION_STRING_TEST_2D),
                                                               ("Test 2e", frm_test_2.clone(), VALIDATION_STRING_TEST_2E),
                                                               ("Test 2f", frm_test_2.clone(), VALIDATION_STRING_TEST_2F),
                                                               ("Test 2g", frm_test_2.clone(), VALIDATION_STRING_TEST_2G),
                                                               ("Test 3a", frm_test_3.clone(), VALIDATION_STRING_TEST_3A),
                                                      ];
    for cur_frm in all_forms{
        println!("{} InterventionRoute::route_to_add_new_intervention", cur_frm.0);
        let responder = InterventionRoute::route_to_intervention_save( app_session.clone(), user_session.clone(), web::Form( cur_frm.1.clone() ) ).await;
        let http_resp = responder.respond_to(&req);
        assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");
        match to_bytes(http_resp.into_body()).await{
            Ok(item) => {
                //println!("{} Body = {}", cur_frm.0, String::from_utf8_lossy(&item));
                // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
                assert!( String::from_utf8_lossy(&item).contains( cur_frm.2 ), "{} - Response did not contain expected content: {}", cur_frm.0, cur_frm.2); 
            },
            Err(_e) => assert!( false, "Error Response received" ),
        };
    }
}

/// ### test_route_to_add_new_intervention()
/// 
/// Tests:
///   InterventionRoute::route_to_add_new_intervention()
/// 
#[actix_web::test]
async fn route_to_add_new_intervention(){ 
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test
    let app_session = EntityFactory::get_mock_app_session().await; // create app session

    let frm_test_1 = InterventionDataFormBasic {  // Test 1: constants::INVALID_OTHER_ID
        patient_id: constants::INVALID_OTHER_ID.to_string(),
        encounter_id: constants::INVALID_OTHER_ID.to_string(),
        intervention_type_id: "100009".to_string(), // invalid, PHN is not 10 digits
    };

    let frm_test_2 = InterventionDataFormBasic {  // Test 2: valid id
        patient_id: "27".to_string(),
        encounter_id: "13".to_string(),
        intervention_type_id: "100003".to_string(), // invalid, PHN is not 10 digits
    };

    const VALIDATION_STRING_TEST_1A: &str  = "<div id=\"MapleHMS::ID=Home\"></div>";
    const VALIDATION_STRING_TEST_1B: &str  = "id=\"patient_id\" value=\"-1";
    const VALIDATION_STRING_TEST_1C: &str  = "id=\"intervention_type_id\" value=\"100009";
    const VALIDATION_STRING_TEST_1D: &str  = "id=\"encounter_id\" value=\"0"; // actually default is to 0, not -1

    const VALIDATION_STRING_TEST_2A: &str  = "<div id=\"MapleHMS::ID=Home\"></div>";
    const VALIDATION_STRING_TEST_2B: &str  = "id=\"patient_id\" value=\"27";
    const VALIDATION_STRING_TEST_2C: &str  = "id=\"intervention_type_id\" value=\"100003";
    const VALIDATION_STRING_TEST_2D: &str  = "id=\"encounter_id\" value=\"13";
    
    // we're going to loop through them as a vector of tuples. The tuple will be the form and a string to validate
    let all_forms = vec![("Test 1a", frm_test_1.clone(), VALIDATION_STRING_TEST_1A),
                                                                    ("Test 1b", frm_test_1.clone(), VALIDATION_STRING_TEST_1B),
                                                                    ("Test 1c", frm_test_1.clone(), VALIDATION_STRING_TEST_1C),
                                                                    ("Test 1d", frm_test_1.clone(), VALIDATION_STRING_TEST_1D),

                                                                    ("Test 2a", frm_test_2.clone(), VALIDATION_STRING_TEST_2A),
                                                                    ("Test 2b", frm_test_2.clone(), VALIDATION_STRING_TEST_2B),
                                                                    ("Test 2c", frm_test_2.clone(), VALIDATION_STRING_TEST_2C),
                                                                    ("Test 2d", frm_test_2.clone(), VALIDATION_STRING_TEST_2D),
                                                      ];
    for cur_frm in all_forms{
        println!("{} InterventionRoute::route_to_add_new_intervention", cur_frm.0);
        let responder = InterventionRoute::route_to_add_new_intervention( app_session.clone(), user_session.clone(), web::Form( cur_frm.1.clone() ) ).await;
        let http_resp = responder.respond_to(&req);
        assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");
        match to_bytes(http_resp.into_body()).await{
            Ok(item) => {
                //println!("{} Body = {}", cur_frm.0, String::from_utf8_lossy(&item));
                // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
                assert!( String::from_utf8_lossy(&item).contains( cur_frm.2 ), "{} - Response did not contain expected content: {}", cur_frm.0, cur_frm.2); 
            },
            Err(_e) => assert!( false, "Error Response received" ),
        };
    }
}