use actix_web::error::HttpError;
use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};
use actix_web::http::StatusCode;
use actix_web::cookie::Key;
use actix_cors::Cors;
use actix_files::*;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware}; //, storage::RedisSessionStore} // for user session management: https://docs.rs/actix-session/latest/actix_session/
// TODO: ideally we'd use an external session store, not just cookies. Until the application is largely working, we'll have to leave this for now. //storage::RedisSessionStore}; 

use MapleEMR::webc::web_content::{WebContentFactory, WebContentItem}; 

//use std::sync::Mutex; // needed for thread safety per https://actix.rs/docs/application/

///
/// # Main program executable for the project
///
///      CSM500 Project (April - October 2026)
///         Graham Parker (Student ID: 240120522)
/// 
/// REFERENCES
/// 
/// Refs for Web and DB:
/// [1] B. Gruber, Rust web development: with Warp, Tokio, and Reqwest. Shelter Island, NY: Manning Publications Co, 2023.
/// https://learning.oreilly.com/library/view/rust-web-development/9781617299001/OEBPS/Text/07.htm#sigil_toc_id_85
/// https://github.com/Rust-Web-Development/code
/// 
/// Refs for ML code:
/// [2] S. Lyu and A. Rzeznik, Practical Rust Projects: Build Serverless, AI, Machine Learning, Embedded, Game, and Web Applications. Berkeley, CA: Apress, 2023. doi: DOI:%2010.1007/978-1-4842-9331-7.
/// https://github.com/LukeMathWalker/zero-to-production
///
mod auth_objects;
mod nlp;
mod errors;
mod dto;
mod webc;

// application-wide database string; should come from a configurable parameter file (TODO)
const DB_CONN_STR: &str = "postgres://postgres:csm500@localhost:5432/csm500";

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
    wcf: WebContentFactory,
    app_key: Key,
    //wcf: Mutex<WebContentFactory>,
    //database pool
    //web static content cache
}

///
/// Stores user session variables
/// REF: https://docs.rs/actix-session/latest/actix_session/struct.SessionMiddleware.html
/// 
#[derive(serde::Serialize, serde::Deserialize)]
struct UserSession {
    user_id: String,
    user_display_name: String,
    email: String,
    // current patients
    // department
    // permissions
    // preferences
}

impl UserSession {

  fn get_patients(){
    todo!();
  }

  fn get_department(){
    todo!();
  }

  fn get_permissions(){
    todo!();
  }

  fn get_preferences(){
    todo!();
  }
}

/// performs a natural language prompt using the built in engine
/// 
/// check by going to: http://127.0.0.1:8000/db
/// 
async fn natural_language_prompt(req: web::Form<NLPromptFormData>) -> impl Responder {
    println!("-> /nlprompt Requested; prompt: \"{}\"", req.prompt);

    //println!("Returning: {}", format!("<b>Rust POC WebDBML2! {}</b>", req.prompt));

    let data = vec!["a","b","c","d","e"];
    let head = vec!["ColA","ColB","ColC","ColD","ColE"];

    let mut results_sbuf = String::with_capacity(50); // Single heap allocation
    results_sbuf.push_str("<div><table>\n"); //class=\"data-table\"

    results_sbuf.push_str( &webc::html_formatter::HTMLFormatter::format_row(head, true) );
    results_sbuf.push_str( &webc::html_formatter::HTMLFormatter::format_row(data, false) );
    

    results_sbuf.push_str("</table></div>\n<br>\n");

    //HttpResponse::Ok().body( format!("NL Response: {}", req.prompt) )
    //HttpResponse::Ok().body(format!("<b>Rust POC WebDBML2! {}</b>", req.prompt)) 
    HttpResponse::Ok().body(format!("{}", results_sbuf)) 
}

/// performs an execution of the NLP engine
/// 
/// check by going to: http://127.0.0.1:8000/ml
///                    http://localhost:8000/ml
/// 
async fn machine_learn_test(_req: HttpRequest) -> impl Responder {
  println!("-> /ml Requested");
  let results = nlp::NLP{}.execute();
  
  HttpResponse::Ok().body(format!("<b>machine_learn_test {}</b>", results.await.to_string())) 
}

/// performs a connect to the database
/// 
/// check by going to: http://127.0.0.1:8000/db 
///                    http://localhost:8000/db
/// 
async fn db(_req: HttpRequest) -> impl Responder {
  println!("-> /db Requested");
  // TODO: use a pool instead, as this will block another query/result in multiple connections the DB may not be able to accomodate
  let cur_db_conn = auth_objects::AuthObjects::new(DB_CONN_STR).await;
  let users = cur_db_conn.get_users( Some(10), 0).await.expect( &errors::DatabaseError::NotFoundError.to_string() );

  let mut s = String::new();
  for row in users.iter() {                          // row: &User
      s = s + &row.id.to_string() + " " + &row.username + "; ";
  }
  HttpResponse::Ok().body(format!("Users: {}, #{}", s, users.len()))
}

/// performs a connect to the database
/// 
/// REF: https://stackoverflow.com/questions/75369137/rust-actix-web-how-to-change-method-when-using-actix-webwebredirecttou
///      https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Redirections#temporary_redirections
///      
/// check by going to: http://127.0.0.1:8000/db
/// 
async fn login(session: Session, req: web::Form<LoginFormData>, data: web::Data<AppSession>, ) -> impl Responder { // Box<dyn Responder<>> { //
  println!("-> /login Requested");

  let cur_db_conn = auth_objects::AuthObjects::new(DB_CONN_STR).await;
  let user_can_login = cur_db_conn.can_user_login(req.username.clone(), req.password.clone()).await.expect( &errors::DatabaseError::NotFoundError.to_string() );

  if user_can_login {
    println!("User can login: {} redirect to /home", req.username.clone());

    // initialize user session (this is the only location it can occur), for an authenticated user
    //  ref: https://docs.rs/actix-admin/latest/actix_admin/prelude/struct.Session.html
    session.insert("USER_SESSION", UserSession {
       user_id: req.username.clone(),
       user_display_name: "User, Fake".to_string(),
       email: "fake@email.com".to_string(),
    }).expect("User Session could not be constructed");
  
     actix_web::web::Redirect::to("/home").using_status_code(StatusCode::SEE_OTHER) // Box::new()
  }
  else{
    println!("Login denied for {} redirect back to /<default route>", req.username.clone());

    // do not PURGE before this; it will trash the session including this new key
    let _ignore = session.insert("VALIDATION_ERRORS".to_string(), "Invalid user or password. Please try again.");
    actix_web::web::Redirect::to("/").using_status_code(StatusCode::SEE_OTHER)
  }
}


/// Allows a monitoring services to perform a basic "is the application up?" check
/// 
async fn is_it_up() -> impl Responder {
  println!("-> /isItUp Requested");
  HttpResponse::Ok().body("MapleEMR is Up")
}

///
/// default route when nothing else is specified by the user
///
//async fn default_route() -> impl Responder {
async fn default_route(data: web::Data<AppSession>, session: Session) -> impl Responder {
  println!("-> /default_route Requested");
  //let redirect_page = WebContentFactory::new(&get_static_path_base()).get_tile(WebContentItem::WCTypeLoginTile);
 // HttpResponse::Ok().body(redirect_page)
 
  let wcf = &data.wcf; 
  println!("Checking session for Validation errors");
  
  match session.get::<String>("VALIDATION_ERRORS"){
    Ok(Some(validation_errors))=> {
       println!("Ok(Some()) Validation errors present in session: {}", &validation_errors);
       // if the login form had validation errors, then we need to show them in the regenerated page.

       let mut content = wcf.get_tile(WebContentItem::WCTypeLoginTile); // retrieve the page base content

       // construct alternate content for the page
       let alt_content = "<label id=\"errLabel\" style=\"color: red\"><b>".to_owned() + &validation_errors + "</b>"; //.expect("User session invalid")

       content = content.replace("<label id=\"errLabel\">", &alt_content);   // retrieve validation errors; they are just raw text for now

       session.purge(); // minimize attack vectors by purging the session 

       HttpResponse::Ok().body( content )
    },
    Ok( None )=> {
      //println!("Ok( None ) No errors present in session");
      println!("Ok( None ) No active session");
      HttpResponse::Ok().body( wcf.get_tile(WebContentItem::WCTypeLoginTile) )
    },
    Err(_)=> {
      //println!("Ok( None ) No errors present in session");
      println!("Session does not exist");
      HttpResponse::Ok().body( wcf.get_tile(WebContentItem::WCTypeLoginTile) )
    },
  }
}


///
/// Main workspace page of the application, to be supplemented with lots of Javascript, CSS and API calls
/// 
async fn workspace(data: web::Data<AppSession>, session: Session) -> impl Responder {
  println!("-> /maple Requested");

  let user_session: UserSession = session.get("USER_SESSION").unwrap().expect("User session invalid"); // retrieve user session info
  let wcf = &data.wcf; // https://actix.rs/docs/application/
  let mut content = wcf.get_tile(WebContentItem::WCTypeWorkspacePage); // retrieve the page base content

  // construct some alternate content for the page
  let user_identity_string = "<label id=\"userIdentityLbl\"><b>".to_owned() + &user_session.user_display_name + "</b>";
  content = content.replace("<label id=\"userIdentityLbl\">", &user_identity_string);  // replace default string

  HttpResponse::Ok().body( content )
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
   Key::generate() // TODO: change this to pull from a config file instead
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
        .route("/home", web::get().to( workspace )) // main workspace
        .route("/nlprompt", web::post().to( natural_language_prompt ))
        .route("/ml", web::get().to( machine_learn_test ))
        .route("/db", web::get().to( db ))
        .route("/isItUp", web::get().to( is_it_up ))
        .service(Files::new("/webc/", "./webc"))  // ref: ttps://actix.rs/docs/static-files/
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}