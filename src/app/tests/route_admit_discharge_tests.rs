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
use maple_hms::route::patient_route::*;
use maple_hms::session::*;
use maple_hms::ui::data_forms::*;
use maple_hms::dto::user_auth::*;

use actix_session::SessionExt;
use actix_web::{body::to_bytes, http::StatusCode, test, web, Responder};
use common::entity_factory::EntityFactory;

const SCREEN_ID_TAG_PATIENT_LIST : &str = "<div id=\"MapleHMS::ID=PatientListTile\"></div>";

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

    let mut frm = GenericWebFormData { target_id: "1".to_string() };

    // Test 1: HomeRoute::route_to_home, standard call with a clinical user
    println!("Test 1: PatientRoute::route_to_patient_details with clinical data (only) user");
    let responder = PatientRoute::route_to_patient_details( user_session.clone(), app_session.clone(), web::Form( frm.clone() ) ).await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp.into_body()).await{
        Ok(item) => {
            // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
            assert!( String::from_utf8_lossy(&item).contains( SCREEN_ID_TAG_PATIENT_LIST ), "Response did not contain expected content"); 
        },
        Err(_e) => assert!( false, "Error Response received" ),
    };


    //AdmitRoute::route_to_admit_new_no_patient(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<AdmitFormBasic>)

}

async fn test_route_to_discharge_patient(){
    //AdmitRoute::route_to_discharge_patient(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<AdmitFormBasic>)

}

async fn test_route_to_admit_discharge(){

    //AdmitRoute::route_to_admit_discharge(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<AdmitDataForm>)

}

async fn test_route_to_discharge_patient_save(){

    //AdmitRoute::route_to_discharge_patient_save(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<DischargeDataForm>)

}
async fn test_route_to_admit_save(){

     //AdmitRoute::route_to_admit_save(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<AdmitDataForm>)

}
    
   