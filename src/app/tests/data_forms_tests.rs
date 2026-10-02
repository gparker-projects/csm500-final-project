///
/// Unit & Integration tests for the data_forms crate of the UI module
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
///  CSM500 Project (April - October 2026)
///  Graham Parker (Student ID: 240120522)
/// -------------------------------------------------------------------

mod common;

#[cfg(test)]
use maple_hms::ui::data_forms::*;

use common::entity_factory::EntityFactory;
use common::data_generator::DataGenerator;

const STRING_200_CHARS_LONG: &str = r##"01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789"##;
const STRING_201_CHARS_LONG: &str = r##"012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567891"##;

/// ### test_generic_web_form_data()
/// 
/// Tests GenericWebFormData struct methods
/// 
#[test]
fn test_generic_web_form_data () {
    let mut frm = GenericWebFormData{
        target_id: "-1".to_string()
    };

    assert_eq!(frm.get_uid_as_i64(), -1);

    frm.target_id = "1".to_string();
    assert_eq!(frm.get_uid_as_i64(), 1);
}

/// ### test_intervention_details_data_form_basic()
/// 
/// Tests InterventionDataFormBasic struct methods
/// 
#[test]
fn test_intervention_data_form_basic() {
    let mut frm = InterventionDataFormBasic {
        intervention_type_id: "-1".to_string(),
        encounter_id: "-1".to_string(),
        patient_id: "-1".to_string(),
    };

    assert_eq!(frm.get_patient_id_as_i64(), -1);

    frm.patient_id = "1".to_string();
    assert_eq!(frm.get_patient_id_as_i64(), 1);
}

/// ### test_intervention_details_data_form_basic()
/// 
/// Tests InterventionDetailsDataForm struct methods
/// 
#[test]
fn test_intervention_details_data_form_basic() {
    let mut frm = InterventionDetailsDataForm {
        type_id: "-1".to_string(),
        ..Default::default()
    };

    assert_eq!(frm.get_type_id_as_i64(), -1);

    frm.type_id = "1".to_string();
    assert_eq!(frm.get_type_id_as_i64(), 1);
}

/// ### test_admit_data_form()
/// 
/// Tests AdmitDataForm struct methods
/// 
#[test]
fn test_admit_data_form() {
    let p = EntityFactory::create_patient();
    let mut frm = AdmitDataForm { // start with a form full of invalid data
        patient_id: String::new(),
        patient_first_name: p.clone().legal_first_name,
        patient_last_name: p.clone().legal_last_name,
        patient_middle_name: p.clone().legal_middle_names,
        phn: p.phn.to_string(),
        birthdate: p.birth_date_for_display(),
        encounter_id: String::new(),  //p.encounter_id.to_string(),
        location_id: String::new(),   //p.location_id.to_string(),
        action_flag: AdmitDataForm::ACTION_FLAG_ADMIT.to_string(),
        admit_notes: String::new(),   //DataGenerator::get_lorem_ipsum(200),
        user_prompt: DataGenerator::get_lorem_ipsum(100),
        form_errors: DataGenerator::get_lorem_ipsum(100)
    };
    // as the validator is sequential, we must "correct" the fields in sequence such that the fields resolve correctly

    // Test 1: Patient Id EMPTY
    match frm.validate_fields() {
        Err (e) => println!("Test 1 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 1: Data was invalid"),
    }; 

    // Test 2: Patient Id negative
    frm.patient_id = "-1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 2 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 2: Data was invalid"),
    };

    // Test 3: Patient Id valid
    frm.patient_id = "1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 3 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 3: Data was invalid"),
    };

    // Test 4: Patient Name too long
    frm.patient_first_name = STRING_201_CHARS_LONG.to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 4 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 4: Data was invalid"),
    };

    // Test 5: Patient Name valid
    frm.patient_first_name = "Normal".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 5 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 5: Data was invalid"),
    };

    // Test 6: Patient Name exactly right sized
    frm.patient_first_name = STRING_200_CHARS_LONG.to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 6 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 6: Data was invalid"),
    };

    // Test 7: Patient Last Name too long
    frm.patient_last_name = STRING_201_CHARS_LONG.to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 7 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 7: Data was invalid"),
    };

    // Test 8: Patient Last Name valid
    frm.patient_last_name = "Normal".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 8 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 8: Data was invalid"),
    };

    // Test 9: Patient Last Name exactly right sized
    frm.patient_last_name = STRING_200_CHARS_LONG.to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 9 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 9: Data was invalid"),
    };

    // Test 10: Patient Middle Name oversized
    frm.patient_middle_name = STRING_201_CHARS_LONG.to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 10 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 10: Data was invalid"),
    };
    
    // Test 11: Patient Middle Name exactly sized
    frm.patient_middle_name = STRING_200_CHARS_LONG.to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 11 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 11: Data was invalid"),
    };

    // Test 12: PHN must be 10 characters long
    frm.phn = String::new();
    match frm.validate_fields() {
        Err (e) => println!("Test 12 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 12: Data was invalid"),
    };

    // Test 13: PHN must be 10 characters long
    frm.phn = "91234567890".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 13 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 13: Data was invalid"),
    };
    frm.phn = "9123456789".to_string();

    // Test 14: birthdate must be greater than 0 length
    frm.birthdate = String::new();
    match frm.validate_fields() {
        Err (e) => println!("Test 14 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 14: Data was invalid"),
    };

    // Test 15: birthdate must be 10 characters long, and it is
    frm.birthdate = "2026-01-01".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 15 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 15: Data was invalid"),
    };

    // Test 16: encounter_id must be > 0
    frm.encounter_id = "1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 16 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 16: Data was invalid"),
    };

    // Test 17: location_id must be > 0
    frm.location_id = "1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 17 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 17: Data was invalid"),
    };

    // Test 18: admit_notes must be > 0; CURRENTLY String::new()
    match frm.validate_fields() {
        Err (e) => println!("Test 18 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 18: Data was invalid"),
    };

    // Test 18: admit_notes must be > 0
    frm.admit_notes = "ADMIT NOTES".to_string();
    match frm.validate_fields() {
        Err (e) => assert!(false, "Test 19: Data was invalid: {}", e),
        _ => println!("Test 19 success, received next error as expected"),
    };
}

/// ### test_intervention_data_form()
/// 
/// Tests InterventionDataForm struct methods
/// 
#[test]
fn test_intervention_data_form_standard() {
    let mut frm = InterventionDataForm {
        patient_id: "-1".to_string(),
        intervention_type_id: "-1".to_string(),
        ..Default::default()
    };

    // basic checks up front
    assert_eq!(frm.get_intervention_type_as_i64(), -1);
    assert_eq!(frm.get_patient_id_as_i64(), -1);

    // prep for the rest
    frm.patient_id = String::new();
    frm.intervention_type_id = String::new();

    // Test 1: intervention_id EMPTY
    match frm.validate_fields() {
        Err (e) => println!("Test 1 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 1: Data was invalid"),
    }; 

    // Test 2: intervention_id negative
    frm.intervention_id = "-1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 2 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 2: Data was invalid"),
    };

    // Test 3: intervention_id valid
    frm.intervention_id = "1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 3 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 3: Data was invalid"),
    };

    //------------------------------------------------------------------------
    // Test 4: description must be between 0 and 2000
    frm.description = DataGenerator::get_lorem_ipsum(2001);
    match frm.validate_fields() {
        Err (e) => println!("Test 4 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 4: Data was invalid"),
    };
    frm.description ="Normal Length Description".to_string();

    // Test 5: description must be between 0 and 2000
    frm.notes = DataGenerator::get_lorem_ipsum(2001);
    match frm.validate_fields() {
        Err (e) => println!("Test 5 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 5: Data was invalid"),
    };
    frm.notes ="Normal Length Description".to_string();

    //------------------------------------------------------------------------
    // Test 6: Location Id EMPTY
    match frm.validate_fields() {
        Err (e) => println!("Test 6 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 6: Data was invalid"),
    }; 

    // Test 7: Location Id negative
    frm.location_id = "-1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 7 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 7: Data was invalid"),
    };

    // Test 8: Location Id valid
    frm.location_id = "1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 8 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 8: Data was invalid"),
    };

    //------------------------------------------------------------------------
    // Test 9: users_id EMPTY
    match frm.validate_fields() {
        Err (e) => println!("Test 9 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 9: Data was invalid"),
    }; 

    // Test 10: users_id negative
    frm.users_id = "-1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 10 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 10: Data was invalid"),
    };

    // Test 11: users_id valid
    frm.users_id = "1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 11 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 11: Data was invalid"),
    };

    //------------------------------------------------------------------------
    // Test 12: encounter_id EMPTY
    match frm.validate_fields() {
           Err (e) => println!("Test 12 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 12: Data was invalid"),
    }; 

    // Test 13: encounter_id negative
    frm.encounter_id = "-1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 13 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 13: Data was invalid"),
    };

    // Test 14: encounter_id valid
    frm.encounter_id = "1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 14 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 14: Data was invalid"),
    };

    //------------------------------------------------------------------------
    // Test 15: intervention_type_id EMPTY
    match frm.validate_fields() {
        Err (e) => println!("Test 15 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 15: Data was invalid"),
    }; 

    // Test 16: intervention_type_id negative
    frm.intervention_type_id = "-1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 16 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 16: Data was invalid"),
    };

    // Test 17: intervention_type_id valid
    frm.intervention_type_id = "1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 17 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 17: Data was invalid"),
    };

    //------------------------------------------------------------------------
    // Test 18: status_id EMPTY
    match frm.validate_fields() {
        Err (e) => println!("Test 18 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 18: Data was invalid"),
    }; 

    // Test 19: status_id negative
    frm.status_id = "-1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 19 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 19: Data was invalid"),
    };

    // Test 20: status_id valid
    frm.status_id = "1".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 20 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 20: Data was invalid"),
    };

    //------------------------------------------------------------------------
    // Test 21: patient_id EMPTY
    match frm.validate_fields() {
        Err (e) => println!("Test 21 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 21: Data was invalid"),
    }; 

    // Test 22: patient_id negative
    frm.patient_id = "-1".to_string();
    match frm.validate_fields() {
        Err (e) => assert!(false, "Test 22: Data was valid: {}", e),
        _ => println!("Test 22 success, received next error as expected"),
    };

    // if ! (self.scheduled_timestamp.len() == 0) && (self.scheduled_timestamp.len() == 16)   {

    // Test 23: scheduled_timestamp INVALID length
    frm.scheduled_timestamp = "BAD_TIMESTAMP".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 23 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 23: scheduled_timestamp was invalid"),
    };
    frm.scheduled_timestamp = "0123456789123456".to_string();

    // Test 24: scheduled_timestamp INVALID length
    frm.performed_timestamp = "BAD_TIMESTAMP".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 24 success, received next error as expected: {}", e),
        _ => assert!(false, "Test 24: scheduled_timestamp was invalid"),
    };

    // Test 25: the last two cases are both positive
    frm.performed_timestamp = String::new();
    match frm.validate_fields() {
        Err (e) => println!("Test 25 success, received next error as expected: {}", e),//assert!(false, "Test 25: scheduled_timestamp was invalid: {}", e),
        _ => assert!(false, "Test 25: performed_timestamp  was invalid"),// println!("Test 25 success, received next error as expected"),
    };

    // Test 26: positive
    frm.performed_timestamp = "0123456789123456".to_string();
    match frm.validate_fields() {
        Err (e) => println!("Test 25 success, received next error as expected: {}", e),//assert!(false, "Test 25: scheduled_timestamp was invalid: {}", e),
        _ => assert!(false, "Test 25: performed_timestamp was invalid"),// println!("Test 25 success, received next error as expected"),
    };
}