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
//use chrono::NaiveDateTime;
//use crate::constants;

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
    pub prompt: String//,
    //pub patient_id: String,
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

#[derive(serde::Deserialize)]
pub struct InterventionDataFormLink {
    pub intervention_id: String,
    pub encounter_id: String,
    pub patient_id: String,
}

#[derive(serde::Deserialize)]
pub struct AdmitFormBasic {
    pub patient_id: String,
    pub action_flag: String,
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

    #[validate(length(min = 10, max = 10, message = "Birthdate must be in YYYY/MM/DD format"))]
    pub birthdate: String,

    //admit_timestamp -> not actually taken as an input
    #[validate(length(min = 0, max = 10, message = "Encounter Id must be a number"))]
    pub encounter_id: String,

    #[validate(length(min = 1, max = 100))]
    pub location_id: String,
    pub action_flag: String,

    #[validate(length(min = 1, max = 2000))]
    pub admit_notes: String,
    pub form_errors: String,
}

impl AdmitDataForm {

    pub fn validate_fields(&self) -> Result<(), ValidationError> {

        if ! (self.patient_id.len() > 0) || ! self.patient_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("Patient Id invalid"));
        }

        if ! (self.patient_first_name.len() > 0) {
            return Err(ValidationError::new("Patient First Name is invalid"));
        }

        if ! (self.patient_last_name.len() > 0) {
            return Err(ValidationError::new("Patient Last Name is invalid"));
        }

        //if ! (self.patient_middle_name.len() > 0) {
        //    return Err(ValidationError::new("Patient Middle Names invalid"));
        //}

        if ! (self.phn.len() > 0) || ! self.phn.parse::<i32>().is_ok() {
            return Err(ValidationError::new("PHN must be a 10 digit number"));
        }
        
        if ! (self.birthdate.len() > 0) {
            return Err(ValidationError::new("Birthdate must be in YYYY/MM/DD format"));
        }
       
        if ! (self.encounter_id.len() > 0) || ! self.encounter_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("Encounter Id invalid"));
        }

        if ! (self.location_id.len() > 0) || ! self.location_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("location Id invalid"));
        }

        if ! (self.admit_notes.len() > 0) {
            return Err(ValidationError::new("Admit notes is invalid"));
        }

        match self.validate() {
            Ok(_) => {
                //println!("All validations passed perfectly!");
                Ok(())
            }
            Err( e ) => {
                println!("Validation failed with errors:\n{}", e);

                Err(ValidationError::new("contains_forbidden_word"))
            }
        }
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

    /*
    fn valid_date(value: &str) -> Result<(), ValidationError> {
        NaiveDateTime::parse_from_str(value, "%Y-%m-%d")
          .map_err(|_| ValidationError::new("invalid_date_format"))?;
        Ok(())
    }
*/
    pub fn validate_fields(&self) -> Result<(), ValidationError> {
        if ! (self.intervention_id.len() > 0) || ! self.intervention_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("Intervention Id invalid"));
        }

        if ! (self.description.len() > 2000) ||  (self.description.len() < 1){
            return Err(ValidationError::new("Description is required and must be less than 2000 characters."));
        }

        if ! (self.notes.len() > 2000) ||  (self.notes.len() < 1){
            return Err(ValidationError::new("Notes are required and must be less than 2000 characters."));
        }

        if ! (self.location_id.len() > 0) || ! self.location_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("Location Id invalid"));
        }

        if ! (self.users_id.len() > 0) || ! self.users_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("User Id invalid"));
        }

        if ! (self.encounter_id.len() > 0) || ! self.encounter_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("Encounter Id invalid"));
        }

        if ! (self.intervention_type_id.len() > 0) || ! self.intervention_type_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("Intervention Type invalid"));
        }

        if ! (self.status_id.len() > 0) || ! self.status_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("Status Id invalid"));
        }

        if ! (self.patient_id.len() > 0) || ! self.patient_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("Patient Id invalid"));
        }

        if ! (self.scheduled_timestamp.len() == 0) && (self.scheduled_timestamp.len() == 16)   {
            return Err(ValidationError::new("Schedule date/time, when provided, must be in YYYY/MM/DD HH:MM format"));
        }

        if ! (self.performed_timestamp.len() == 0) && (self.performed_timestamp.len() == 16)   {
            return Err(ValidationError::new("Performed date/time, when provided, must be in YYYY/MM/DD HH:MM format"));
        }

        match self.validate() {
            Ok(_) => {
                //println!("All validations passed perfectly!");
                Ok(())
            }
            Err( e ) => {
                println!("InterventionDataForm validation failed:\n{}", e);

                Err( ValidationError::new("InterventionDataForm validation failed") )
            }
        }
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

    pub form_errors: String,
}

impl InterventionDetailsDataForm {

    pub fn validate_fields(&self) -> Result<(), ValidationError> {

        if ! (self.intervention_details_id.len() > 0) || ! self.intervention_details_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("Intervention -Details Id invalid"));
        }

        if ! (self.intervention_id.len() > 0) || ! self.intervention_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("Intervention Id invalid"));
        }

        if ! (self.type_id.len() > 0) || ! self.type_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("Type Id is invalid"));
        }

        if ! (self.value.len() > 200) ||  (self.value.len() < 1){
            return Err(ValidationError::new("Description is required and must be less than 200 characters."));
        }

        if ! (self.notes.len() > 200) ||  (self.notes.len() < 1){
            return Err(ValidationError::new("Notes are required and must be less than 200 characters."));
        }

        if ! (self.entry_timestamp.len() > 0) {
            return Err(ValidationError::new("Entry Timestamp invalid"));
        }

        match self.validate() {
            Ok(_) => {
                //println!("All validations passed perfectly!");
                Ok(())
            }
            Err( e ) => {
                println!("InterventionDetailsDataForm validation failed:\n{}", e);

                Err( ValidationError::new("InterventionDetailsDataForm validation failed") )
            }
        }
    }
}

#[derive(serde::Deserialize)]
pub struct InterventionDetailsAddFormBasic {
    pub addFrm_intv_id: String,
    pub addFrm_patient_id: String,
    pub addFrm_type_id: String,
    pub addFrm_value: String,
    pub addFrm_notes: String,
    //pub add_measure_form_errors: String
}