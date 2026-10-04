/// -------------------------------------------------------------------
/// #Unit & Integration tests for the Session, SysConfig and Convert Utils module
/// 
/// ###Includes:
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
///  CSM500 Project (April - October 2026)
///  Graham Parker (Student ID: 240120522)
/// -------------------------------------------------------------------

mod common;

#[cfg(test)]

use maple_hms::constants;
use maple_hms::route::nle_route::*;

use maple_hms::ui::data_forms::*;

use actix_session::SessionExt;
use actix_web::{body::to_bytes, http::StatusCode, test, web, Responder};

/* 
use maple_hms::session::*;

use maple_hms::dto::user_auth::*;

use actix_session::SessionExt;
use actix_web::{body::to_bytes, http::StatusCode, test, web, Responder};
use actix_web::cookie::Key;
use sqlx::postgres::PgPoolOptions;

const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";
const SCREEN_ID_TAG_LOGIN : &str = "<div id=\"MapleHMS::ID=Login\"></div>";
const SCREEN_ID_TAG_HOME : &str = "<div id=\"MapleHMS::ID=Home\"></div>";
//const SCREEN_ID_TAG_INTERVENTION : &str = "<div id=\"MapleHMS::ID=Intervention\"></div>";
const SCREEN_ID_TAG_PATIENT_LIST : &str = "<div id=\"MapleHMS::ID=PatientListTile\"></div>";
*/

//const SCREEN_ID_TAG_NLE_PROMPT : &str = "TBD";
const VALIDATION_STRING_NO_CONTEXT_1 : &str  = "<div id=\"MapleHMS::NLPCanvas\">";
const VALIDATION_STRING_NO_CONTEXT_2 : &str  = "nlp_action_2";
const VALIDATION_STRING_INVALID_PATIENT : &str  = "<div id=\"MapleHMS::NLPCanvas\">";
const VALIDATION_STRING_CTXTLVL_0 : &str  = "<div id=\"MapleHMS::NLPCanvas\">";
const VALIDATION_STRING_CTXTLVL_1 : &str  = VALIDATION_STRING_NO_CONTEXT_2;
const VALIDATION_STRING_CTXTLVL_2 : &str  = "Your prompt did not result in any options";
const VALIDATION_STRING_CTXTLVL_0B : &str  = VALIDATION_STRING_NO_CONTEXT_2;
const VALIDATION_STRING_CTXTLVL_1B : &str  = VALIDATION_STRING_NO_CONTEXT_2;
const VALIDATION_STRING_OTHER_2: &str  = "Update Denny DoNotDischarge";
const VALIDATION_STRING_MALFORMED: &str  = "value='Update Patient Information' onclick=\"performNLAction(2,-1);";


use common::entity_factory::EntityFactory;


/// ### test_nle_route_natural_language_prompt()
/// 
/// Tests:
///   NLERoute::natural_language_prompt()
/// 
#[actix_web::test]
async fn test_nle_route_natural_language_prompt(){
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test

    let static_base_path = std::env::current_dir().expect("Base path to executable could not be found").display().to_string() + "\\data\\";
    let app_session = EntityFactory::get_mock_nle_app_session( static_base_path ).await; // create app session

    // make some forms
    let frm_no_context = NLPromptFormData { prompt: "no context prompt".to_string() };
    let frm_invalid_patient = NLPromptFormData { prompt: "{patient_id=-1} invalid patient prompt".to_string() };
    let frm_ctxtlvl_0 = NLPromptFormData { prompt: "{ctxtlvl=0} context level 0 prompt".to_string() };
    let frm_ctxtlvl_1 = NLPromptFormData { prompt: "{ctxtlvl=1} context level 1 prompt".to_string() };
    let frm_ctxtlvl_2 = NLPromptFormData { prompt: "{ctxtlvl=2} context level 2 prompt".to_string() };

    let frm_ctxtlvl_0b = NLPromptFormData { prompt: "{patient_id=1}{ctxtlvl=0} context level 0 prompt".to_string() };
    let frm_ctxtlvl_1b = NLPromptFormData { prompt: "{patient_id=1}{ctxtlvl=1} context level 1 prompt".to_string() };
    let frm_ctxtlvl_2b = NLPromptFormData { prompt: "{patient_id=1}{ctxtlvl=2} context level 2 prompt".to_string() };
    let frm_other_1 = NLPromptFormData { prompt: "{patient_id=1}{ctxtlvl=2}{other} context level 2 prompt".to_string() };
    let frm_other_2 = NLPromptFormData { prompt: "{patient_id=1}{ctxtlvl=2} Denny DoNotDischarge".to_string() };
    
    // we're going to loop through them as a vector of tuples. The tuple will be the form and a string to validate
    let all_forms = vec![
                                                        (frm_no_context.clone(), VALIDATION_STRING_NO_CONTEXT_1),
                                                        (frm_no_context.clone(), VALIDATION_STRING_NO_CONTEXT_2),
                                                        (frm_invalid_patient, VALIDATION_STRING_INVALID_PATIENT),
                                                        (frm_ctxtlvl_0,  VALIDATION_STRING_CTXTLVL_0),
                                                        (frm_ctxtlvl_1,  VALIDATION_STRING_CTXTLVL_1),
                                                        (frm_ctxtlvl_2, VALIDATION_STRING_CTXTLVL_2),
                                                        (frm_ctxtlvl_0b, VALIDATION_STRING_CTXTLVL_0B),
                                                        (frm_ctxtlvl_1b, VALIDATION_STRING_CTXTLVL_1B),
                                                        (frm_ctxtlvl_2b, VALIDATION_STRING_NO_CONTEXT_2),
                                                        (frm_other_1, VALIDATION_STRING_NO_CONTEXT_2),
                                                        (frm_other_2, VALIDATION_STRING_OTHER_2)
                                                ];
    for nl_frm in all_forms{
        let responder = NLERoute::natural_language_prompt(app_session.clone(), user_session.clone(), web::Form( nl_frm.0.clone() ) ).await;
        let http_resp = responder.respond_to(&req);
        assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");
        match to_bytes(http_resp.into_body()).await{
            Ok(item) => {
                println!("Body = {}", String::from_utf8_lossy(&item));
                // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
                assert!( String::from_utf8_lossy(&item).contains( nl_frm.1 ), "Response did not contain expected content"); 
            },
            Err(_e) => assert!( false, "Error Response received for: {}", nl_frm.0.prompt ),
        };
    }
}

/// ### test_nle_route_natural_language_prompt_site_2()
/// 
/// Tests:
///   NLERoute::natural_language_prompt(), for user 9 which is Site 2 (no patients)
/// 
#[actix_web::test]
async fn test_nle_route_natural_language_prompt_site_2(){
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("9".to_string())); // we will ignore error as this is a unit test

    let static_base_path = std::env::current_dir().expect("Base path to executable could not be found").display().to_string() + "\\data\\";
    let app_session = EntityFactory::get_mock_nle_app_session( static_base_path ).await; // create app session

    // make some forms
    let frm_no_context = NLPromptFormData { prompt: "no context prompt".to_string() };
    let frm_invalid_patient = NLPromptFormData { prompt: "{patient_id=-1} invalid patient prompt".to_string() };
    let frm_ctxtlvl_0 = NLPromptFormData { prompt: "{ctxtlvl=0} context level 0 prompt".to_string() };
    let frm_ctxtlvl_1 = NLPromptFormData { prompt: "{ctxtlvl=1} context level 1 prompt".to_string() };
    let frm_ctxtlvl_2 = NLPromptFormData { prompt: "{ctxtlvl=2} context level 2 prompt".to_string() };

    let frm_ctxtlvl_0b = NLPromptFormData { prompt: "{patient_id=1}{ctxtlvl=0} context level 0 prompt".to_string() };
    let frm_ctxtlvl_1b = NLPromptFormData { prompt: "{patient_id=1}{ctxtlvl=1} context level 1 prompt".to_string() };
    let frm_ctxtlvl_2b = NLPromptFormData { prompt: "{patient_id=1}{ctxtlvl=2} context level 2 prompt".to_string() };
    let frm_other_1 = NLPromptFormData { prompt: "{patient_id=1}{ctxtlvl=2}{other} context level 2 prompt".to_string() };
    let frm_other_2 = NLPromptFormData { prompt: "{patient_id=1}{ctxtlvl=2} Denny DoNotDischarge".to_string() };
    let frm_malformed = NLPromptFormData { prompt: "{patient_id=1{ctxtlvl=2 Denny DoNotDischarge".to_string() };
    
    const USER_9_VALIDATION_STRING_OTHER_2: &str = "value='Update Patient Information' onclick=\"performNLAction(2,1)";

    // we're going to loop through them as a vector of tuples. The tuple will be the form and a string to validate
    let all_forms = vec![
                                                        (frm_no_context.clone(), VALIDATION_STRING_NO_CONTEXT_1),
                                                        (frm_no_context.clone(), VALIDATION_STRING_NO_CONTEXT_2),
                                                        (frm_invalid_patient, VALIDATION_STRING_INVALID_PATIENT),
                                                        (frm_ctxtlvl_0,  VALIDATION_STRING_CTXTLVL_0),
                                                        (frm_ctxtlvl_1,  VALIDATION_STRING_CTXTLVL_1),
                                                        (frm_ctxtlvl_2, VALIDATION_STRING_CTXTLVL_2),
                                                        (frm_ctxtlvl_0b, VALIDATION_STRING_CTXTLVL_0B),
                                                        (frm_ctxtlvl_1b, VALIDATION_STRING_CTXTLVL_1B),
                                                        (frm_ctxtlvl_2b, VALIDATION_STRING_NO_CONTEXT_2),
                                                        (frm_other_1, VALIDATION_STRING_NO_CONTEXT_2),
                                                        (frm_other_2, USER_9_VALIDATION_STRING_OTHER_2),
                                                        (frm_malformed, VALIDATION_STRING_MALFORMED) // malformed
                                                ];
    for nl_frm in all_forms{
        let responder = NLERoute::natural_language_prompt(app_session.clone(), user_session.clone(), web::Form( nl_frm.0.clone() ) ).await;
        let http_resp = responder.respond_to(&req);
        assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");
        match to_bytes(http_resp.into_body()).await{
            Ok(item) => {
                println!("Body = {}", String::from_utf8_lossy(&item));
                // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
                assert!( String::from_utf8_lossy(&item).contains( nl_frm.1 ), "Response did not contain expected content"); 
            },
            Err(_e) => assert!( false, "Error Response received for: {}", nl_frm.0.prompt ),
        };
    }
}