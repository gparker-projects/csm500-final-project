/// -------------------------------------------------------------------
/// #Unit & Integration tests for Intervention Details routes
/// 
/// ###Includes:
/// * InterventionDetailsRoute::route_to_add_intervention_detail()
/// * InterventionDetailsRoute::route_to_intervention_detail_save()
/// 
///  CSM500 Project (April - October 2026)
///  Graham Parker (Student ID: 240120522)
/// -------------------------------------------------------------------

mod common;

#[cfg(test)]

use maple_hms::constants;
use maple_hms::route::intervention_details_route::*; // InterventionRoute
use maple_hms::ui::data_forms::*;

use actix_session::SessionExt;
use actix_web::{body::to_bytes, http::StatusCode, test, web, Responder};
use common::entity_factory::EntityFactory;

/// ### test_route_to_add_intervention_detail()
/// 
/// Tests:
///   InterventionRoute::route_to_add_intervention_detail()
/// 
#[actix_web::test]
async fn test_route_to_add_intervention_detail(){
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test
    let app_session = EntityFactory::get_mock_app_session().await; // create app session

    let frm_test_1 = InterventionDetailsAddFormBasic { // Test 1, invalid data
        addFrm_intv_id: constants::INVALID_OTHER_ID.to_string(),
        addFrm_intv_dtls_id: "1".to_string(),
        addFrm_patient_id: constants::INVALID_PATIENT_ID.to_string(),
        addFrm_type_id: "1".to_string(),
        addFrm_value: "1".to_string(),
        addFrm_notes: "".to_string(),
    };

    let frm_test_2 = InterventionDetailsAddFormBasic { // Test 1, invalid data
        addFrm_intv_id: "147".to_string(),
        addFrm_intv_dtls_id: "241".to_string(),
        addFrm_patient_id: "27".to_string(),
        addFrm_type_id: "44".to_string(),
        addFrm_value: "UPDATED - PASS".to_string(),
        addFrm_notes: "InterventionRoute::route_to_add_intervention_detail() - UNIT TEST".to_string(),
    };

    const VALIDATION_STRING_TEST_1A: &str  = "<div id=\"MapleHMS::ID=Intervention\"></div>";
    const VALIDATION_STRING_TEST_1B: &str  = "id=\"addFrm_patient_id\" value=\"-1";
    const VALIDATION_STRING_TEST_1C: &str  = "id=\"addFrm_intv_id\" value=\"-1";
    const VALIDATION_STRING_TEST_1D: &str  = "id=\"addFrm_intv_dtls_id\" value=\"-1"; // new entries will always have a -1

    const VALIDATION_STRING_TEST_2A: &str  = VALIDATION_STRING_TEST_1A;
    const VALIDATION_STRING_TEST_2B: &str  = "id=\"addFrm_patient_id\" value=\"27";
    const VALIDATION_STRING_TEST_2C: &str  = "id=\"addFrm_intv_id\" value=\"147";
    const VALIDATION_STRING_TEST_2D: &str  = VALIDATION_STRING_TEST_1D; // new entries will always have a -1
    
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
        println!("{} InterventionDetailsRoute::route_to_add_intervention_detail", cur_frm.0);
 
        // * route_to_add_intervention_detail(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<InterventionDetailsAddFormBasic>)
        let responder = InterventionDetailsRoute::route_to_add_intervention_detail( app_session.clone(), user_session.clone(), web::Form( cur_frm.1.clone() ) ).await;
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


//route_to_intervention_detail_save(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<InterventionDetailsDataForm>)

//    const VALIDATION_STRING_TEST_2B: &str  = "id=\"addFrm_patient_id\" value=\"27";
//    const VALIDATION_STRING_TEST_2C: &str  = "id=\"intv_id_241\" value=\"147";
//    const VALIDATION_STRING_TEST_2D: &str  = "id=\"intv_dtls_id_241\" value=\"241";