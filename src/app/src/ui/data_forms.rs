//! -------------------------------------------------------------------
//! Defines the Forms which are submitted to routes from the HTML UI of
//!  the application. These forms are basically composite data objects
//!  and are used by actix to assemble/transfer data.
//!
//! These could have been put into routes, but were assembled all together
//! for easier reference, consistency and organization.
//! 
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 
//! -------------------------------------------------------------------

use validator::{Validate, ValidationError};

#[derive(serde::Deserialize)]
pub struct LoginFormData {
    #[serde(rename = "mplUsername")]
    pub username: String,
    #[serde(rename = "mplPassword")]
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct NLPromptFormData {
    #[serde(rename = "prompt")]
    pub prompt: String
}

///
/// A generalized form for 80% of web form submission sitautions, so we dont have a ton of minor forms for one-off uses.
/// 
#[derive(serde::Deserialize)]
pub struct GenericWebFormData {
    pub target_id: String,
}

impl GenericWebFormData {
  pub fn get_uid_as_i64(&self) -> i64{
      let result: i64 = self.target_id.parse().unwrap();
      return result;
  }
}

#[derive(serde::Deserialize)]
pub struct InterventionDataFormBasic {
    pub intervention_type_id: String,
    pub encounter_id: String,
    pub patient_id: String,
}

impl InterventionDataFormBasic {
    pub fn get_patient_id_as_i64(&self) -> i64{
        let result: i64 = self.patient_id.parse().unwrap();
        return result;
    }    
}

#[derive(serde::Deserialize)]
pub struct InterventionDataFormLink {
    pub intervention_id: String,
    pub encounter_id: String,
    pub patient_id: String,
}

#[derive(serde::Deserialize)]
pub struct AdmitFormBasic {
    pub patient_id: String,
    //pub action_flag: String,
    pub user_prompt: String,
}

#[derive(Default, serde::Deserialize, Validate, Clone)]
pub struct DischargeDataForm {
    pub patient_id: String,
    pub encounter_id: String,
    pub discharge_notes: String
}

///
/// A generalized form for 80% of web form submission sitautions, so we dont have a ton of minor forms for one-off uses.
/// 
#[derive(Default, serde::Deserialize, Validate, Clone)]
pub struct AdmitDataForm {
    #[validate(length(min = 0, max = 10, message = "Patient Id invalid"))]
    pub patient_id: String,

    #[validate(length(min = 3, max = 100, message = "First name must be at least 3 characters and can not exceed 100"))]
    pub patient_first_name: String,

    #[validate(length(min = 3, max = 100, message = "Last name must be at least 3 characters and can not exceed 100"))]
    pub patient_last_name: String,

    #[validate(length(min = 0, max = 100, message = "Middle names can not exceed 100 characters"))]
    pub patient_middle_name: String,

    #[validate(length(min = 10, max = 10, message = "PHN must be a 10 digit number"))]
    pub phn: String,

    #[validate(length(min = 11, max = 11, message = "Birthdate must be in YYYY-MON-DD format"))]
    pub birthdate: String,

    //admit_timestamp -> not actually taken as an input
    #[validate(length(min = 0, max = 10, message = "Encounter Id must be a number"))]
    pub encounter_id: String,

    #[validate(length(min = 1, max = 100))]
    pub location_id: String,
    pub action_flag: String,

    #[validate(length(min = 1, max = 2000))]
    pub admit_notes: String,
    pub user_prompt: String,  
    
    #[allow(dead_code)]
    pub form_errors: String, 
}

impl AdmitDataForm {
    pub const ACTION_FLAG_ADMIT: &str = r##"admit"##;
    pub const ACTION_FLAG_DISCHARGE: &str = r##"discharge"##;
    
    pub fn validate_fields(&self) -> Result<(), ValidationError> {
        let mut error = ValidationError::new("AdmitDataFormError");

        if ! (self.patient_id.len() > 0) || ! self.patient_id.parse::<i64>().is_ok() {
            error.message = Some("Patient Id invalid".into());
            //println!("AdmitDataForm::validate_fields() Error: {} [{}]", error.message.as_ref().unwrap(), self.patient_id);
            return Err(error);
        }

        if ! (self.patient_first_name.len() >= 1) && (self.patient_first_name.len() <= 200){
            error.message = Some("Patient First Name is invalid".into());
            //println!("AdmitDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }

        if ! (self.patient_last_name.len() >= 1) && (self.patient_last_name.len() <= 200) {
            error.message = Some("Patient Last Name is invalid".into());
            //println!("AdmitDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }

        // can't really validate the presence of a middle name, as some people do not have them
        if self.patient_middle_name.len() > 200{
            //println!("AdmitDataForm::validate_fields()");
            //println!(".. patient_middle_name >{}<", self.patient_middle_name);
            error.message = Some("Patient Middle Names invalid".into());
            //println!("AdmitDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }        

        if ! (self.phn.len() == 10) || ! self.phn.parse::<i64>().is_ok() {
            let err_msg = "PHN ".to_owned() + &self.phn + "must be a 10 digit number".into();
            error.message = Some(err_msg.into());
            //println!("AdmitDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }
        
        if ! (self.birthdate.len() > 0) {
            error.message = Some("Birthdate must be in YYYY/MM/DD format".into());
            //println!("AdmitDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }
       
        if ! (self.encounter_id.len() > 0) || ! self.encounter_id.parse::<i64>().is_ok() {
            error.message = Some("Encounter Id invalid".into());
            //println!("AdmitDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }

        if ! (self.location_id.len() > 0) || ! self.location_id.parse::<i64>().is_ok() {
            error.message = Some("Location Id invalid".into());
            //println!("AdmitDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }

        if ! (self.admit_notes.len() > 1) && (self.admit_notes.len() < 2000){
            error.message = Some("Admit notes is invalid".into());
            //println!("AdmitDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }
        //println!("AdmitDataForm::validate_fields(): PASSED");
        Ok(())
    }
}

///
/// InterventionDataForm, for saving full-data Interventions
/// 
#[derive(Default, serde::Deserialize, Validate, Clone)]
pub struct InterventionDataForm {
    #[validate(length(min = 1, max = 10, message = "Intervention Id invalid"))]
    pub intervention_id: String,

    #[validate(length(min = 1, max = 2000, message = "Description is required and must be less than 2000 characters."))]
    pub description: String, 

    #[validate(length(min = 1, max = 2000, message = "Notes is required and must be less than 2000 characters."))]
    pub notes: String,

    #[validate(length(min = 1, max = 1000000000, message = "Location Id must be a number"))]
    pub location_id: String,

    #[validate(length(min = 1, max = 1000000000, message = "Users Id must be a number"))]
    pub users_id: String,
   
    #[validate(length(min = 1, max = 1000000000, message = "Encounter Id must be a number"))]
    pub encounter_id: String,

    #[validate(length(min = 1, max = 1000000000, message = "Intervention Type must be a number"))]
    pub intervention_type_id: String, 

    #[validate(length(min = 1, max = 1000000000, message = "Status Id must be a number"))]
    pub status_id: String, 

    #[validate(length(min = 1, max = 1000000000, message = "Patient Id must be a number"))]
    pub patient_id: String, 

    //#[validate(length(min = 16, max = 16, message = "Schedule date/time must be in YYYY/MM/DD HH:MM format"))]
    pub scheduled_timestamp: String, // optional field, can not be validated this easily

    //#[validate(length(min = 16, max = 16, message = "Schedule date/time must be in YYYY/MM/DD HH:MM format"))]
    pub performed_timestamp: String, // optional field, can not be validated this easily

    pub form_errors: String,
}

impl InterventionDataForm {

    pub fn get_patient_id_as_i64(&self) -> i64{
        let result: i64 = self.patient_id.parse().unwrap();
        return result;
    }

    pub fn get_intervention_type_as_i64(&self) -> i64{
        let result: i64 = self.intervention_type_id.parse().unwrap();
        return result;
    }

    pub fn validate_fields(&self) -> Result<(), ValidationError> {
        let mut error = ValidationError::new("InterventionDataForm Error");

        if ! (self.intervention_id.len() > 0) || ! self.intervention_id.parse::<i64>().is_ok() {
            error.message = Some("Intervention Id invalid".into());
            println!("InterventionDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }

        if  self.description.len() > 2000 {
            error.message = Some("Description is required and must be less than 2000 characters.".into());
            println!("InterventionDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            println!("...value={}", self.description);
            return Err(error);
        }

        if  self.notes.len() > 2000 {
            error.message = Some("Notes must be less than 2000 characters.".into());
            println!("InterventionDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }

        if ! (self.location_id.len() > 0) || ! self.location_id.parse::<i64>().is_ok() {
            error.message = Some("Location Id invalid".into());
            println!("InterventionDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }

        if ! (self.users_id.len() > 0) || ! self.users_id.parse::<i64>().is_ok() {
            error.message = Some("User Id invalid".into());
            println!("InterventionDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }

        if ! (self.encounter_id.len() > 0) || ! self.encounter_id.parse::<i64>().is_ok() {
            error.message = Some("Encounter Id invalid".into());
            println!("InterventionDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }

        if ! (self.intervention_type_id.len() > 0) || ! self.intervention_type_id.parse::<i64>().is_ok() {
            error.message = Some("Intervention Type invalid".into());
            println!("InterventionDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }

        if ! (self.status_id.len() > 0) || ! self.status_id.parse::<i64>().is_ok() {
            error.message = Some("Status Id invalid".into());
            println!("InterventionDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }

        if ! (self.patient_id.len() > 0) || ! self.patient_id.parse::<i64>().is_ok() {
            error.message = Some("Patient Id invalid".into());
            println!("InterventionDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }

        if ! (self.scheduled_timestamp.len() == 0 || self.scheduled_timestamp.len() == 16)   {
            error.message = Some("Schedule date/time, when provided, must be in YYYY/MM/DD HH:MM format".into());
            println!("InterventionDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }

        if ! (self.performed_timestamp.len() == 0 || self.performed_timestamp.len() == 16)   {
            error.message = Some("Performed date/time, when provided, must be in YYYY/MM/DD HH:MM format".into());
            println!("InterventionDataForm::validate_fields() Error: {}", error.message.as_ref().unwrap());
            return Err(error);
        }

        Ok(())
    }
}

///
/// InterventionDataForm, for saving full-data Interventions
/// 
#[derive(Default, serde::Deserialize, Validate, Clone)]
pub struct InterventionDetailsDataForm {
    #[validate(length(min = 1, max = 10, message = "Intervention-Details Id invalid"))]
    pub intervention_details_id: String,

    #[validate(length(min = 1, max = 10, message = "Intervention Id invalid"))]
    pub intervention_id: String,
    
    #[validate(length(min = 1, max = 1000000000, message = "Type must be a number"))]
    pub type_id: String, 

    #[validate(length(min = 1, max = 2000, message = "Value is required and must be less than 2000 characters."))]
    pub value: String,

    #[validate(length(min = 1, max = 2000, message = "Notes is required and must be less than 2000 characters."))]
    pub notes: String,

    #[validate(length(min = 1, max = 2000, message = "Entry Timestamp is required."))]
    pub entry_timestamp: String,

    #[allow(dead_code)]
    pub form_errors: String,
}

impl InterventionDetailsDataForm {

    pub fn get_type_id_as_i64(&self) -> i64{
        let result: i64 = self.type_id.parse().unwrap();
        return result;
    }    
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize)]
 // these are the names in the forms in Javascript; not worth the time to correct from a warning, given it is dynamically generated and hard to debug
pub struct InterventionDetailsAddFormBasic {
    pub addFrm_intv_id: String,
    pub addFrm_intv_dtls_id: String,
    pub addFrm_patient_id: String,
    pub addFrm_type_id: String,
    pub addFrm_value: String,
    pub addFrm_notes: String,    
    //pub add_measure_form_errors: String
}