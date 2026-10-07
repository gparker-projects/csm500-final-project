//! -------------------------------------------------------------------
//! Intervention Details and related routes
//!
//!  CSM500 Project (April - October 2026)
//!  Graham Parker (Student ID: 240120522)

use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session}; 
use tracing;

use crate::constants;
use crate::dao::feature_preference_dao::FeaturePreferenceDAO;
use crate::dao::intervention_dao::InterventionDAO;
use crate::route::intervention_route::InterventionRoute;
use crate::session::{AppSession, UserSession};
use crate::ui::data_forms::*;

pub struct InterventionDetailsRoute{}

impl InterventionDetailsRoute{
    /// ### InterventionDetailsRoute::route_to_add_intervention_detail()
    ///    Route that will update the intervention and then redirect back to the modify screen
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * user_session (actix_session::Session): the user's session
    /// * req: web::Form<DischargeDataForm>: the user's request, encapsulated in a InterventionDetailsAddFormBasic
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    pub async fn route_to_add_intervention_detail(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<InterventionDetailsAddFormBasic>) -> impl Responder {
        tracing::debug!("-> Route Requested: /route_to_add_intervention_detail ");
        
        InterventionDetailsRoute::route_to_intervention_detail_save(       // translate the InterventionDetailsAddFormBasic into a InterventionDetailsDataForm
                app_session.clone(),
                user_session.clone(),
                web::Form(
                    InterventionDetailsDataForm {
                        intervention_id: req.addFrm_intv_id.clone(),
                        intervention_details_id: req.addFrm_intv_dtls_id.clone(),//constants::INVALID_OTHER_ID.to_string(),
                        type_id: req.addFrm_type_id.clone(),
                        value: req.addFrm_value.clone(),
                        notes: req.addFrm_notes.clone(),
                        ..Default::default()
                    }
                )
        ).await;
        
        // route back to main form again
        InterventionRoute::route_to_view_or_modify_intervention(
                    app_session,
                    user_session,
                    web::Form(
                    InterventionDataForm {
                        intervention_id: req.addFrm_intv_id.clone(),
                        patient_id: req.addFrm_patient_id.clone(),
                        intervention_type_id: "100040".to_string(), // every user should have access to the "Alerts/CCI/SPI" intervention type, as it is for staff safety
                        ..Default::default()
                    }
                )
        ).await
    }

    /// ### InterventionDetailsRoute::route_to_intervention_detail_save()
    ///    Route that will update the intervention and then redirect back to the modify screen
    /// 
    /// #### Parameters:
    /// * app_session (web::Data<session::AppSession>): the application session
    /// * user_session (actix_session::Session): the user's session
    /// * req: web::Form<DischargeDataForm>: the user's request, encapsulated in a InterventionDetailsDataForm
    /// 
    /// #### Returns:
    /// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
    /// 
    pub async fn route_to_intervention_detail_save(app_session: web::Data<AppSession>, user_session: Session, req: web::Form<InterventionDetailsDataForm>) -> impl Responder {
        tracing::debug!("-> Route Requested: /route_to_intervention_detail_save ");
        println!("-> Route Requested: /route_to_intervention_detail_save ");
        let mut results: String = constants::INVALID_OTHER_ID.to_string();
        let user_session_details: UserSession = user_session.get(constants::USER_SESSION).unwrap().expect( constants::SESSION_ERROR_INVALID ); // retrieve user session inf

        let idao: InterventionDAO = InterventionDAO::new( app_session.get_db_connection() ).await;
        let intv_dtls_id = idao.upsert_intervention_details_from_intv_form(req.clone(), user_session_details.get_userid_as_i64()).await.unwrap();
        if intv_dtls_id != constants::INVALID_OTHER_ID { // upsert_intervention_details_from_intv_form never returns an error, only a wrapped constants::INVALID_OTHER_ID 
            //tracing::debug!("..Intervention Details (id={intv_dtls_id})] created/updated");
            //println!("..Intervention Details (id={intv_dtls_id})] created/updated");
            results = intv_dtls_id.to_string();
            let type_id = req.clone().get_type_id_as_i64(); // extract type for updating the user's feature preference

            // if save successful, record a feature preference as well
            let fpdao = FeaturePreferenceDAO::new( app_session.get_db_connection() ).await;
            let _ignore = fpdao.upsert_feature_preference( user_session_details.clone().get_userid_as_i64() , type_id).await.unwrap();
        }

        // route back to main form again
        //InterventionRoute::route_to_view_or_modify_intervention( app_session, user_session, req ).await
        HttpResponse::Ok().body( results )
    }
}