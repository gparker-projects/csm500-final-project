//! # Main program executable for the project
//!
//!      CSM500 Project (April - October 2026)
//!         Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 
//! Refs for Web and DB:
//! [1] B. Gruber, Rust web development: with Warp, Tokio, and Reqwest. Shelter Island, NY: Manning Publications Co, 2023.
//! https://learning.oreilly.com/library/view/rust-web-development/9781617299001/OEBPS/Text/07.htm#sigil_toc_id_85
//! https://github.com/Rust-Web-Development/code
//! 
//! Refs for ML code:
//! [2] S. Lyu and A. Rzeznik, Practical Rust Projects: Build Serverless, AI, Machine Learning, Embedded, Game, and Web Applications. Berkeley, CA: Apress, 2023. doi: DOI:%2010.1007/978-1-4842-9331-7.
//! https://github.com/LukeMathWalker/zero-to-production
//!
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::http::StatusCode;
use actix_web::cookie::Key;
use actix_cors::Cors;
use actix_files::*;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware}; //, storage::RedisSessionStore} // for user session management: https://docs.rs/actix-session/latest/actix_session/

use crate::dto::{user_auth::*, patient::*, encounter::*, intervention::*, intervention_detail::*};
use crate::dao::{encounter_dao::*, patient_dao::*, intervention_dao::*, auth_dao::*};

//use crate::nlp::NLP; 

// TODO: ideally we'd use an external session store, not just cookies. Until the application is largely working, we'll have to leave this for now. //storage::RedisSessionStore}; 
use crate::webc::web_content::{WebContentFactory, WebContentItem}; 

mod constants;
mod dto;
mod webc;
mod dao;
mod nlp;

//use std::sync::Mutex; // needed for thread safety per https://actix.rs/docs/application/

#[derive(serde::Deserialize)]
pub struct LoginFormData {
    #[serde(rename = "mplUsername")]
    username: String,
    #[serde(rename = "mplPassword")]
    password: String,
}

#[derive(serde::Deserialize)]
pub struct NLPromptFormData {
    #[serde(rename = "prompt")]
    prompt: String,
}

///
/// Stores application-wide state/variables
/// REF: https://actix.rs/docs/application/
/// 
struct AppSession {
    app_version: String,
    wcf: WebContentFactory,    //wcf: Mutex<WebContentFactory>,
    app_key: Key,
    //todo: add database pool
}

///
/// Stores user session variables
/// REF: https://docs.rs/actix-session/latest/actix_session/struct.SessionMiddleware.html
/// 
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserSession {
    pub user_id: String,
    pub user_display_name: String,
    pub email: String,
    pub user_authorizations: UserAuthorization,   // for permissions and departments
    // current patients
    // preferences
}

impl UserSession {
  fn get_userid_as_i64(&self) -> i64{
      let result: i64 = self.user_id.parse().unwrap();
      return result;
  }
}
/// performs a natural language prompt using the built in engine
/// 
/// check by going to: http://127.0.0.1:8000/db
/// 
//async fn natural_language_prompt(req: web::Form<NLPromptFormData>) -> impl Responder {
async fn natural_language_prompt(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<NLPromptFormData>) -> impl Responder {
  
    println!("-> /nlprompt Requested; prompt: \"{}\"", req.prompt);

    let mut results_sbuf = String::with_capacity(50); // Single heap allocation
    results_sbuf.push_str("<b>PLACEHOLDER CONTENT/b>\n");

    if req.prompt.contains("discharge") {
        //actix_web::web::Redirect::to("/admdis").using_status_code(StatusCode::SEE_OTHER)
        HttpResponse::Ok().body(format!( r##"{{"action": "discharge", "prompt": "{}",}}"##, req.prompt)) 
    }
    else if req.prompt.contains("admit")  {
        HttpResponse::Ok().body(format!( r##"{{"action": "admit", "prompt": "{}",}}"##, req.prompt))
    }
    else {
        HttpResponse::Ok().body(format!(r##"{{"action": "other", "prompt": "{}",}}"##, req.prompt)) 
    }

    //actix_web::web::Redirect::to("/home").using_status_code(StatusCode::SEE_OTHER)
   
    // these are the ACTUAL execution from the POC
      //let results = nlp::NLP{}.execute();
      //HttpResponse::Ok().body(format!("<b>machine_learn_test {}</b>", results.await.to_string())) 

    //HttpResponse::Ok().body(format!("{}", results_sbuf)) 
}

/// performs a connect to the database
/// 
/// REF: https://stackoverflow.com/questions/75369137/rust-actix-web-how-to-change-method-when-using-actix-webwebredirecttou
///      https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Redirections#temporary_redirections
///      
/// check by going to: http://127.0.0.1:8000/db
/// 
async fn login(user_session: Session, req: web::Form<LoginFormData>, _app_session: web::Data<AppSession>, ) -> impl Responder { // Box<dyn Responder<>> { //
  println!("-> /login Requested");

  let cur_db_conn = AuthDAO::new(constants::DB_CONN_STR).await;
  let user_can_login = cur_db_conn.can_user_login(req.username.clone(), req.password.clone()).await.expect( constants::DATABASE_ERROR_NOT_FOUND );

  match user_can_login {
    Some (current_user) => {
      println!("User can login: {} redirect to /home", req.username.clone());

      let uid: i64 = current_user.id;
      let user_perms = cur_db_conn.get_user_permissions( uid ).await.expect( constants::DATABASE_ERROR_NOT_FOUND ).unwrap();
      // TODO: catch this for users without data

      // initialize user session (this is the only location it can occur), for an authenticated user
      //  ref: https://docs.rs/actix-admin/latest/actix_admin/prelude/struct.Session.html
      // copy values from the db into the session; will use a different type of object than the DTO.user
      user_session.insert(constants::USER_SESSION, UserSession {
        user_id: current_user.id.to_string(), 
        user_display_name: current_user.name + " (" + &current_user.user_name + ")",
        email: current_user.email,
        user_authorizations: user_perms,
      }).expect("User Session could not be constructed");
    
      actix_web::web::Redirect::to("/home").using_status_code(StatusCode::SEE_OTHER) 
    }
    None => {
      println!("Login denied for {} redirect back to /<default route>", req.username.clone()); // must use the user from the session as DB was not successful

      // do not PURGE before this; it will trash the session including this new key
      let _ignore = user_session.insert(constants::VALIDATION_ERRORS, "Invalid user or password. Please try again.");
      actix_web::web::Redirect::to("/").using_status_code(StatusCode::SEE_OTHER)
    }
  }
}

///
/// Allows a monitoring services to perform a basic "is the application up?" check
/// 
async fn is_it_up() -> impl Responder {
  println!("-> /isItUp Requested");
  HttpResponse::Ok().body("MapleEMR is Up")
}

///
/// default route when nothing else is specified by the user
///
async fn default_route(app_session: web::Data<AppSession>, user_session: Session) -> impl Responder {
  println!("-> /default_route Requested");
 
  let wcf = &app_session.wcf; 
  println!("Checking session for Validation errors");
  
  match user_session.get::<String>(constants::VALIDATION_ERRORS){
    Ok(Some(validation_errors))=> {
       println!("Ok(Some()) Validation errors present in session: {}", &validation_errors);
       // if the login form had validation errors, then we need to show them in the regenerated page.

       let mut content = wcf.get_tile(WebContentItem::WCTypeLoginTile); // retrieve the page base content

       // construct alternate content for the page
       let alt_content = "<label id=\"errLabel\" style=\"color: red\"><b>".to_owned() + &validation_errors + "</b>"; //.expect("User session invalid")
       content = content.replace("<label id=\"errLabel\">", &alt_content);   // retrieve validation errors; they are just raw text for now

       user_session.purge(); // minimize attack vectors by purging the session 

       HttpResponse::Ok().body( content )
    },
    Ok( None )=> {
      //println!("Ok( None ) No errors present in session");
      println!("Ok( None ) No active session");
      HttpResponse::Ok().body( wcf.get_tile(WebContentItem::WCTypeLoginTile) )
    },
    Err(_)=> {
      //println!("Ok( None ) No errors present in session");
      println!("User session does not exist");
      HttpResponse::Ok().body( wcf.get_tile(WebContentItem::WCTypeLoginTile) )
    },
  }
}

///
/// Main workspace page of the application, to be supplemented with lots of Javascript, CSS and API calls
/// 
async fn route_to_home(app_session: web::Data<AppSession>, user_session: Session) -> impl Responder {
  println!("-> /home Route Requested");

  let user_session: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info

  let wcf = &app_session.wcf; // https://actix.rs/docs/application/
  let mut content = wcf.get_home_tile(); // retrieve the page base content

  // get patients at the user's facility, for display
  let dao = PatientDAO::new(constants::DB_CONN_STR).await;
  let idao = InterventionDAO::new(constants::DB_CONN_STR).await;
  let edao = EncounterDAO::new(constants::DB_CONN_STR).await;

  let qry_results = dao.get_patients_at_users_site_no_discharge(user_session.get_userid_as_i64(), false).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
  match qry_results {
    Some (patient_list) => {
       //println!("Retrieved {} patients:", patient_list.len());
       let mut pwrap: Vec<PatientWrapper> = Vec::new();

       for p in patient_list.clone(){
          let cur_enc: Encounter = edao.get_current_encounter(p.id.clone()).await.clone();
          let cur_intv: Intervention = idao.get_most_recent_vitals(cur_enc.id.clone()).await.clone(); //get the current encounter for each patient
          let cur_idtls = idao.get_all_intervention_details(cur_intv.id, constants::NOT_SPECIFIED_ID).await.expect(constants::DATABASE_ERROR_NOT_FOUND).clone(); //.clone();

          //let tmp_p = p.clone();
          //let tmp_e = cur_enc.clone();
          //let tmp_i = cur_intv.clone();

          pwrap.push( PatientWrapper{
                  patient: p.clone(),
                  current_encounter: cur_enc.clone(),
                  most_recent_intervention: cur_intv.clone(),
                  intervention_detail: cur_idtls.clone().expect(constants::GENERAL_ERROR_NOT_FOUND)
              }
          );
          //print!(">> DEBUG Added pid={} e={} i={}", tmp_p, tmp_e, tmp_i);
       }
       
       let patient_list_html = wcf.get_home_route_summary_of_patients_tile_using_wrapper(pwrap.clone()); 
       content = content.replace(constants::BODY_TILE_CONTENT_TAG, &patient_list_html);  // replace default string

       // todo: offload this to the tile generator; should not be repeated
       let std_menu_html = wcf.get_legacy_menu(patient_list.clone()); 
       content = content.replace(constants::LEGACY_MENU_TILE_TAG, &std_menu_html);  // replace default string       
    }
    None => {
      println!("No patients found");
    }
  }
  // and adjust the menu

  // add the user's identity
  content = content.replace(constants::USER_IDENTITY_TILE_TAG, &&user_session.user_display_name); 

  //change to get_home_tile_with_user_identity(&&user_session.user_display_name);

  HttpResponse::Ok().body( content )
}


///
/// A generalized form for 80% of web form submission sitautions, so we dont have a ton of minor forms for one-off uses.
/// 
#[derive(serde::Deserialize)]
pub struct GenericWebFormData {
    target_id: String,
}

impl GenericWebFormData {
  fn get_uid_as_i64(&self) -> i64{
      let result: i64 = self.target_id.parse().unwrap();
      return result;
  }
}

///
/// A generalized form for 80% of web form submission sitautions, so we dont have a ton of minor forms for one-off uses.
/// 
#[derive(Default, serde::Deserialize)]
pub struct AdmitFormData {
    pub patient_id: String,
    pub patient_first_name: String,
    pub patient_last_name: String,
    pub patient_middle_name: String,
    pub phn: String,
    pub birthdate: String,
    //admit_timestamp -> not actually taken as an input
    pub encounter_id: String,
    pub location: String,
    pub admit_notes: String,
    // intervention details
    pub temperature: String,
    pub blood_pressure: String,
    pub weight: String,
    pub intervention_notes: String,
}

impl AdmitFormData {

  pub fn validate() -> String{
      let results: String = "Valid".to_string();

      //if patient_id

      results // return results, whatever they may be
  }

 /*() pub fn to_admit_object_tuple(&self) -> (Option<Patient>, Option<Encounter>, Option<Intervention>){
    let p = Patient {
      id: &self.patient_id,
      legal_first_name: &self.legal_first_name,
      legal_last_name: &self.legal_last_name,
      legal_middle_names: &self.legal_middle_names,
      sin: &self.sin,
      phn: &self.phn,
      birth_date: &self.birth_date,
    };

    /*let e: Encounter = {};
    let o: Intervention = {};


    e.encounter_id = encounter_id;
    e.location_id = location_id;
    e.location_short_name = location_short_name;

    e.admit_timestamp = admit_timestamp;
    e.admit_notes = admit_notes;
    e.discharge_timestamp = discharge_timestamp;
    e.discharge_notes = discharge_notes;*/

    ( Some(p), Some(e), Some(o) )
  }*/
}

///
/// Wrapper route for the menu option to admit a patient without having any web form to pass data in from
/// 
async fn route_to_admit_new_no_patient(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<GenericWebFormData>) -> impl Responder {
    println!("-> /route_to_admit_new_no_patient Route Requested");

    route_to_admit_discharge(app_session, user_session, web::Form(
        AdmitFormData {
            patient_id: req.0.target_id.clone(),
            ..Default::default()
        }
    )).await
}

///
/// Route for New patient admit, or existing patient discharge page
/// 
async fn route_to_admit_discharge(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<AdmitFormData>) -> impl Responder {
  println!("-> /admit_discharge Route Requested");

  let user_session: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
  let wcf = &app_session.wcf;

  let patient_id: i64 = req.patient_id.parse().unwrap();
  let dao = PatientDAO::new(constants::DB_CONN_STR).await;


  println!(">> route_to_admit_discharge() called");

  let existing_patient: Option<dto::patient::Patient> = if patient_id == constants::NOT_SPECIFIED_ID {
      println!(">> no Patient specified: create a new Patient and Encounter");
      
      //let results = dao.update_patient_details(user_id, p); 
      None
  }
  else{
      println!(">> Patient exists: update Patient and Encounter");
      dao.get_patient_details( user_session.get_userid_as_i64(), patient_id).await.expect( constants::DATABASE_ERROR_NOT_FOUND ) 
  };




  // TODO: pull the legacy menu code out into a common method
  // get_legacy_menu(dao, user_id, patient_id)
  //
  let legacy_menu_results = dao.get_patients_at_users_site_no_discharge(user_session.get_userid_as_i64(), false).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
  let legacy_menu = match legacy_menu_results {
      Some (patients_for_menu_lst) => {
          wcf.get_legacy_menu_with_patient(patients_for_menu_lst.clone(), patient_id)
      }
      None => {
          println!("No patients found for legacy menu");
          constants::LEGACY_MENU_ON_ERROR.to_string() // when no patient, return default error-expected menu
      }
  };

  let content = wcf.get_admit_discharge_tile(user_session.user_display_name, existing_patient, legacy_menu); // retrieve the page base content

  HttpResponse::Ok().body( content ) //"TODO : route_to_admit_discharge()" ) 
}

///
/// Route for adding new, or modifying existing Interventions of a patient
/// 
async fn route_to_modify_intervention(app_session: web::Data<AppSession>, user_session: Session) -> impl Responder {
  println!("-> /modify_intervention Route Requested");

  let user_session: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info

  let wcf = &app_session.wcf; 
  //let mut content = wcf.get_modify_intervention_tile(); // retrieve the page base content

  //pub fn get_modify_intervention_tile(&self, current_intervention: Intervention) -> String {



  HttpResponse::Ok().body( "CONTENT TODO: route_to_modify_intervention()" )  //content )
}

///
/// Route to View Patient details; expects a GenerialWebFormData to have been submitted to reach the route
///
async fn route_to_patient_details(user_session: Session, app_session: web::Data<AppSession>, req: web::Form<GenericWebFormData>) -> impl Responder {
  println!("-> /patientdtls Route Requested");

  //todo: this should direct to a standard error or login screen when session is lost
  let user_session: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session info
  let wcf: &WebContentFactory = &app_session.wcf; 

  let patient_id: i64 = req.get_uid_as_i64();

  // get base patient data
  let dao = PatientDAO::new(constants::DB_CONN_STR).await;
  let idao = InterventionDAO::new(constants::DB_CONN_STR).await;
  let edao = EncounterDAO::new(constants::DB_CONN_STR).await;

  let mut current_encounter: String = "No Encounters found".to_owned();

  // get encounters for the patient
  let enc_results = edao.get_encounters(patient_id, false).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
  let enc_section = match enc_results {
      Some (encounters) => {
         //println!("Patient details obtained");

         // pull out the current Encounter and get alternate summary tile for it
         let (current_enc, all_other_encs): (Vec<_>, Vec<_>) = encounters.into_iter().partition(|item| item.is_current_encounter == "Y");
         if let Some(enc) = current_enc.first(){
             current_encounter = wcf.get_single_encounter_summary_tile(enc.clone());
         }

         wcf.get_encounter_list_tile(all_other_encs)
      }
      None =>{
         //println!("No Encounters found");
         "No Encounters found".to_owned()
      } 
  };

  // get all interventions for the patient
  let intv_results = idao.get_interventions(patient_id, false).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
  let intv_section = match intv_results {
      Some (intvs) => {
         //println!("Patient details obtained");
         wcf.get_intervention_list_tile(intvs)
      }
      None =>{
         //println!("No Encounters found");
         "No Interventions found".to_owned()
      } 
  };

  // get patient encounter history

  let patient_results = dao.get_patient_details( user_session.get_userid_as_i64(), patient_id).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
  let patient_header = match patient_results {
      Some (patient_details) => {
         //println!("Patient details obtained"); //: {}", &tile_content);
         wcf.get_patient_details_tile(patient_details.clone())
      }
      None =>{
         //println!("No patients found");
         "No patients found".to_owned()
      } 
  };

  // refresh the patients in the menu (only)
  let legacy_menu_results = dao.get_patients_at_users_site_no_discharge(user_session.get_userid_as_i64(), false).await.expect( constants::DATABASE_ERROR_NOT_FOUND );
  let legacy_menu = match legacy_menu_results {
    Some (patients_for_menu_lst) => {
        wcf.get_legacy_menu_with_patient(patients_for_menu_lst.clone(), patient_id)
    }
    None => {
        println!("No patients found for legacy menu");
        constants::LEGACY_MENU_ON_ERROR.to_string()
    }
  };

  let consolidated_content = wcf.get_patient_details_full_tile(patient_header, current_encounter, enc_section,
                                                  user_session.user_display_name, legacy_menu, intv_section); //, enc_history);

  HttpResponse::Ok().body( consolidated_content )
}

///
/// Helper function: obtains the web static path base, which is used to retrieve many sources of static content
/// TODO: if this is not being used anywhere other than WebContentFactory, can we remove it?
/// 
fn get_static_path_base() -> String{
   //let path = see below
   //println!("Default Route base dir: {}", path.clone());
   return std::env::current_dir().expect("Base path to executable could not be found").display().to_string() + "\\webc\\static\\";
}

///
/// Provides the secret key for the application, usually from a config file (TODO)
/// REF: https://docs.rs/actix-web/latest/actix_web/cookie/struct.Key.html
/// 
fn get_application_secret_key() -> Key {
    println!(">get_application_secret_key()");

    actix_web::cookie::Key::from(
    std::env::var("SESSION_KEY")
        .unwrap_or_else(|_| "this_is_a_new_system_key_to_prevent_regeneration_of_a_key_every_time_the_app_starts".to_string())
        .as_bytes()
    )
}

/// # Main program
/// 
/// Loads the NLP engine and adds handlers for key paths of the web application
/// 
/// Ref: Add CORS headers to allow javascript connectivity
///      ->  https://docs.rs/actix-cors/latest/actix_cors/struct.Cors.html 
/// 
/// Returns std::io::Result<()> for 
#[tokio::main]
async fn main() -> std::io::Result<()> {

//  //nlp::NLP{}.execute();
// https://docs.rs/actix-cors/latest/actix_cors/struct.Cors.html 

  println!("MapleEMR is running! Access via: http://127.0.0.1:8000");

  // use the Builder pattern to add one route at a time
  HttpServer::new(|| {

      let tmp_app_key = get_application_secret_key(); // create within the enclosure to make sure it is available and consistent for the two uses below

      App::new()
          .wrap(
            Cors::default()
                //.allowed_origin("http://localhost:8000") // Restrict to specific origin
                .allow_any_origin() // not great... will have to do for now
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
                .allow_any_header()
                .max_age(3600),
        )
        .app_data(  // this enclosure allows the session state to be created and made available to all routes. actix_web magic.
            web::Data::new( AppSession {
                app_version: "v1.0".to_string(),
                //wcf: Mutex::new( WebContentFactory::new(&get_static_path_base()) )
                wcf: WebContentFactory::new(&get_static_path_base()),
                app_key: tmp_app_key.clone()
              }
            ) 
        )
        .wrap(SessionMiddleware::new(CookieSessionStore::default(), tmp_app_key.clone())) // for user session
        .route("/", web::get().to( default_route ))
        .route("/login", web::post().to( login ))
        .route("/home", web::get().to( route_to_home )) // main workspace
        .route("/patientdtls", web::post().to( route_to_patient_details ))
        .route("/nlprompt", web::post().to( natural_language_prompt ))
        .route("/admit", web::post().to( route_to_admit_discharge ))
        .route("/admitnew", web::post().to( route_to_admit_new_no_patient ))
        .route("/discharge", web::post().to( route_to_admit_discharge ))
        .route("/modintv", web::post().to( route_to_modify_intervention ))
        //.route("/ml", web::get().to( machine_learn_test ))
        .route("/isItUp", web::get().to( is_it_up ))
        .service(Files::new("/webc/", "./webc"))  // ref: ttps://actix.rs/docs/static-files/
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}