use validator::{Validate, ValidationError};
use chrono::NaiveDateTime;
use crate::constants;


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
    pub prompt: String,
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
pub struct AdmitFormBasic {
    pub adm_target_id: String,
}

impl AdmitFormBasic {
  pub fn get_uid_as_i64(&self) -> i64{
      let result: i64 = self.adm_target_id.parse().unwrap();
      return result;
  }
}

///
/// A generalized form for 80% of web form submission sitautions, so we dont have a ton of minor forms for one-off uses.
/// 
#[derive(Default, serde::Deserialize, Validate, Clone)]
pub struct AdmitFormData {
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
    pub birthdate: String,
    //admit_timestamp -> not actually taken as an input
    #[validate(length(min = 0, max = 10, message = "Encounter Id must be a number"))]
    pub encounter_id: String,
    #[validate(length(min = 1, max = 100))]
    pub location_id: String,
    #[validate(length(min = 1, max = 2000))]
    pub admit_notes: String,
    pub form_errors: String,
    // intervention details
   /*  #[validate(length(min = 1, max = 100))]
    pub temperature: String,
     #[validate(length(min = 1, max = 100))]
    pub blood_pressure: String,
     #[validate(length(min = 1, max = 100))]
    pub weight: String,
     #[validate(length(min = 1, max = 2000))]
    pub intervention_notes: String,*/
}

impl AdmitFormData {

    fn valid_date(value: &str) -> Result<(), ValidationError> {
        NaiveDateTime::parse_from_str(value, "%Y-%m-%d")
          .map_err(|_| ValidationError::new("invalid_date_format"))?;
        Ok(())
    }

    pub fn validate_fields(&self) -> Result<(), ValidationError> {
        //let mut result = true;

        if ! (self.patient_id.len() > 0) || ! self.patient_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("Patient Id invalid"));
        }
       
        if ! (self.encounter_id.len() > 0) || ! self.encounter_id.parse::<i64>().is_ok() {
            return Err(ValidationError::new("Encounter Id invalid"));
        }

        if ! (self.phn.len() > 0) || ! self.phn.parse::<i32>().is_ok() {
            return Err(ValidationError::new("PHN must be a 10 digit number"));
        }
        
       /*  match AdmitFormData::valid_date(&self.phn) {
            Ok(_) => {
                println!("All validations passed perfectly!");w 
            }
            Err( e ) => {
                Err(ValidationError::new("Birthdate must be a date in YYYY-MM-DD format"));
            }
        }*/
        // TODO 

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