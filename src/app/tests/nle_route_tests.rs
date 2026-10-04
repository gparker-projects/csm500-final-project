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
/*
use maple_hms::constants;
use maple_hms::ui::tile_factory::WebContentFactory;
use maple_hms::route::default_route::*;
use maple_hms::route::login_route::*;
use maple_hms::route::home_route::*;
use maple_hms::route::patient_route::*;

use maple_hms::session::*;
use maple_hms::ui::data_forms::*;
use maple_hms::dto::user_auth::*;

use actix_session::SessionExt;
use actix_web::{body::to_bytes, http::StatusCode, test, web, Responder};
use actix_web::cookie::Key;
use sqlx::postgres::PgPoolOptions;

const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";
const SCREEN_ID_TAG_LOGIN : &str = "<div id=\"MapleHMS::ID=Login\"></div>";
const SCREEN_ID_TAG_HOME : &str = "<div id=\"MapleHMS::ID=Home\"></div>";
//const SCREEN_ID_TAG_INTERVENTION : &str = "<div id=\"MapleHMS::ID=Intervention\"></div>";
const SCREEN_ID_TAG_PATIENT_LIST : &str = "<div id=\"MapleHMS::ID=PatientListTile\"></div>";*/


/// ### test_nle_route_natural_language_prompt()
/// 
/// Tests:
///   NLERoute::natural_language_prompt()
/// 
#[actix_web::test]
async fn test_nle_route_natural_language_prompt(){

    //Test 1: context level 1

    //Test 2: context level 2

    //Test 3: context level 0

    //Test 4a: OTHER_PATIENT_FOUND 

    //Test 4b: TARGET_PATIENT_FOUND  

    //Test 4c: all other cases 
}