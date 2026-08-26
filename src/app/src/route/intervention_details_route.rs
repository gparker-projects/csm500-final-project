//! # Intervention Details and related routes
//!
//!      CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 
use tracing;

use crate::dto::intervention::Intervention;

use crate::dao::{ patient_dao::*, intervention_dao::*, common_dao::*, auth_dao::*}; 
use crate::webc::{data_forms::*, menu_tile::*};
use crate::webc::data_forms::InterventionDataForm;

use crate::session::{AppSession, UserSession};

use crate::constants;

pub struct InterventionDetailsRoute{}

impl InterventionDetailsRoute{

  ///
  /// Route that will update the intervention and then redirect back to the modify screen
  /// 
  pub async fn route_to_intervention_detail_save(app_session: web::Data<AppSession>, user_session: Session, mut req: web::Form<InterventionDataForm>) -> impl Responder {
      tracing::debug!("-> Route Requested: /route_to_discharge_patient_save ");

      HttpResponse::Ok().body(  content )
  }


}