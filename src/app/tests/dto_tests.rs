///
/// #Unit & Integration tests for the DTO module. Includes all DTO objects
/// 
/// * Encounter
/// * FeeaturePreference
/// * Intervention and InterventionDetails
/// * Patient
/// * Permission
/// * UserAuthorization
/// * User
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 
///  CSM500 Project (April - October 2026)
///  Graham Parker (Student ID: 240120522)
/// -------------------------------------------------------------------

use chrono::{Utc, NaiveDateTime, NaiveDate};
use rand::{RngExt, rng};

use common::data_generator::*; 
use maple_emr::{constants, dto::{encounter::*, feature_preference::*, intervention::*, intervention_detail::*, patient::*, user::*}};
use maple_emr::dto::{user_auth::*, convert_utils::*};
use maple_emr::ui::data_forms::AdmitDataForm;

mod common;

// set up per: https://doc.rust-lang.org/book/ch11-03-test-organization.html

#[cfg(test)] 

/// ### test_create_encounter_dto()
/// 
/// Tests the ability to create an Encounter DTO and its basic methods:
/// * admit_timestamp_for_display()
/// * to_string() - trait override
/// 
#[test]
fn test_create_encounter_dto() {
    let current_time: NaiveDateTime = Utc::now().naive_utc();
    let mut rng = rng();

    let encounter_id: i64  = rng.random();
    let admit_timestamp: NaiveDateTime = current_time;
    let admit_notes: String = DataGenerator::get_lorem_ipsum(2000);
    let discharge_timestamp: Option<NaiveDateTime> = Some( DataGenerator::add_random_seconds(current_time, 3600, 36000) );
    let discharge_notes: String = DataGenerator::get_lorem_ipsum(2000);
    let patient_id: i64 = rng.random();
    let encounter_site_name: String =  DataGenerator::get_lorem_ipsum(16);
    let room_identifier: String = DataGenerator::get_room_identifier(200);
    let is_current_encounter: String = "Y".to_string();

    let obj = Encounter::new(
        encounter_id.clone(),
        admit_notes.clone(),
        admit_timestamp.clone(),
        discharge_notes.clone(), 
        discharge_timestamp.clone(), 
        patient_id.clone(), // patient_id
        encounter_site_name.clone(),
        room_identifier.clone(),
        is_current_encounter.clone()
    );

    assert_eq!(obj.id, encounter_id);
    assert_eq!(obj.admit_timestamp, admit_timestamp);
    assert_eq!(obj.admit_notes, admit_notes);
    assert_eq!(obj.discharge_timestamp, discharge_timestamp);
    assert_eq!(obj.discharge_notes, discharge_notes);
    assert_eq!(obj.patient_id, patient_id);
    assert_eq!(obj.encounter_site_name, encounter_site_name);
    assert_eq!(obj.room_identifier, room_identifier);
    assert_eq!(obj.is_current_encounter, is_current_encounter);

    assert_eq!(obj.admit_timestamp_for_display(), current_time.format("%d/%m/%Y %H:%M:%S").to_string());

    let check_string = "(encounter Id: ".to_owned() + &encounter_id.to_string() + ", admit_timestamp: " + &current_time.format("%d/%m/%Y %H:%M:%S").to_string() + 
                               ", encounter_site_name: " + &encounter_site_name +
                               ", is_current_encounter: " + &is_current_encounter + &")";

    assert_eq!(obj.to_string(), check_string);
}

/// ### test_create_intervention_dto()
/// 
/// Tests the ability to create an Intervention DTO
/// 
#[test]
fn test_create_intervention_dto() {
    let mut rng = rng();

    let intervention_id: i64 = rng.random();
    let encounter_id: i64 = rng.random();
    let description: String = DataGenerator::get_lorem_ipsum(2000);
    let notes: String = DataGenerator::get_lorem_ipsum(2000);
    let location_id: i64 = rng.random();
    let users_id: i64 = rng.random_range(1..3);
    let status_code: String = DataGenerator::get_intv_status_description();
    let intervention_type_id: i64 = rng.random();
    let status_id: i64 = rng.random_range(13..18);
    let intervention_type = DataGenerator::get_intv_type_description();
    let room_identifier = DataGenerator::get_room_identifier(200);
    let current_time: NaiveDateTime = Utc::now().naive_utc();

    // instantiate a DTO to prove it accepts data, but more importantly, detect unexpected changes to it that will break the application
    let obj = Intervention::new(
            intervention_id.clone(),
            encounter_id.clone(),
            description.clone(), //  description,
            notes.to_string(), //  notes,
            location_id.clone(),
            users_id.clone(),
            status_code.clone(),
            intervention_type_id.clone(),
            status_id.clone(),
            intervention_type.clone(),
            room_identifier.clone(),
            Some(current_time), // scheduled_timestamp: Option<NaiveDateTime>,
            Some(current_time)//performed_timestamp: Option<NaiveDateTime>
        );

    assert_eq!(obj.id, intervention_id);
    assert_eq!(obj.encounter_id, encounter_id);
    assert_eq!(obj.description, description);
    assert_eq!(obj.notes, notes);
    assert_eq!(obj.location_id, location_id);

    assert_eq!(obj.users_id, users_id);
    assert_eq!(obj.intervention_type_id, intervention_type_id);
    assert_eq!(obj.status_id, status_id);
    assert_eq!(obj.intervention_type, intervention_type);
    assert_eq!(obj.room_identifier, room_identifier);

    assert_eq!(obj.scheduled_timestamp, Some(current_time));
    assert_eq!(obj.performed_timestamp, Some(current_time));
}

/// ### test_create_intervention_details_dto()
/// 
/// Tests the ability to create an Intervention Details DTO and its basic methods:
/// * entry_timestamp_for_display()
/// * type_name()
/// * to_string() -- override of trait
/// 
#[test]
fn test_create_intervention_details_dto() {
    let mut rng = rng();
    let current_time: NaiveDateTime = Utc::now().naive_utc();

    let intervention_details_id: i64 = rng.random();
    let intervention_id: i64 = rng.random();
    let type_id: i64 = rng.random_range(30..39);
    let value: String = DataGenerator::get_lorem_ipsum(100);
    let notes: String = DataGenerator::get_lorem_ipsum(2000);

    let entry_timestamp: NaiveDateTime = current_time;
    let intervention_type: String = DataGenerator::get_intv_detail_type_description();

    let obj: InterventionDetail = InterventionDetail::new(
              intervention_details_id.clone(),
              intervention_id.clone(),
              type_id.clone(),
              value.clone(),
              notes.clone(),
              entry_timestamp.clone(),
              intervention_type.clone()
    );

    assert_eq!(obj.id, intervention_details_id); 
    assert_eq!(obj.intervention_id, intervention_id); 
    assert_eq!(obj.type_id, type_id); 
    assert_eq!(obj.value, value); 
    assert_eq!(obj.notes, notes); 
    assert_eq!(obj.entry_timestamp, entry_timestamp); 
    assert_eq!(obj.intervention_type, intervention_type); 

    assert_eq!(obj.entry_timestamp_for_display(), entry_timestamp.format("%Y-%b-%d %H:%M").to_string()); 
    assert_eq!(obj.type_name(), intervention_type.clone()); 
    assert_eq!(obj.to_string(), "(InterventionDetail Id: ".to_owned() + &intervention_details_id.to_string() +
                                ", intervention_type: " + &intervention_type + 
                                ", value: " + &value + &")"   ); 
}

/// ### test_create_patient_dto()
/// 
/// Tests the ability to create a User DTO and prove the Patient DTO works/continues to work; includes the Encounter table details
/// 
/// REF: Random number gerneration for tests: Zero-to-prod, page 159; now deprecated apparently.
/// 
#[test]
fn test_create_patient_dto() {
      let current_time: NaiveDateTime = Utc::now().naive_utc();
      let mut rng = rng();

      let fixed_birth_date: NaiveDateTime = NaiveDate::from_ymd_opt(1909, 9, 21).unwrap().and_hms_opt(0, 0, 0).unwrap();
      let fixed_discharge_timestamp: NaiveDateTime = NaiveDate::from_ymd_opt(2026, 9, 19).unwrap().and_hms_opt(0, 0, 0).unwrap();

      let patient_id: i64 = rng.random();
      let legal_first_name: String = DataGenerator::get_first_name(100);
      let legal_last_name: String = DataGenerator::get_last_name(100);
      let legal_middle_names: String = DataGenerator::get_middle_names(100);
      let phn: i64 = DataGenerator::get_phn();
      let birth_date = fixed_birth_date; //NaiveDateTime = DataGenerator::get_date();
      let location_id: i64 = rng.random();
      let location_short_name: String =  DataGenerator::get_lorem_ipsum(16);
      let admit_timestamp: NaiveDateTime = current_time;
      let admit_notes: String = DataGenerator::get_lorem_ipsum(2000);

      let discharge_timestamp: Option<NaiveDateTime> = Some( fixed_discharge_timestamp ); // DataGenerator::add_random_seconds(current_time, 3600, 36000) );

      let discharge_notes: String = DataGenerator::get_lorem_ipsum(2000);

      // instantiate a DTO to prove it accepts data, but more importantly, detect unexpected changes to it that will break the application
      let mut obj = Patient::new(
          patient_id.clone(), // patient_id
          constants::NOT_SPECIFIED_ID, //encounter_id, 
          legal_first_name.clone(), //legal_first_name, 
          legal_last_name.clone(), //legal_last_name, 
          legal_middle_names.clone(), //legal_middle_names, 
          phn.clone(), //phn, 
          birth_date.clone(), //birth_date,
          location_id.clone(), //location_id,
          location_short_name.clone(), //location_short_name, 
          admit_timestamp.clone(), //admit_timestamp,
          admit_notes.clone(), //admit_note
          discharge_timestamp.clone(),
          discharge_notes.clone()//discharge_notes
      );

      println!("...fixed_birth_date: {}", fixed_birth_date);
      println!("...birth_date: {}", birth_date);
      println!("...obj.birth_date: {}", obj.birth_date);

      // basic tests
      assert_eq!(obj.id, patient_id); 
      assert_eq!(obj.encounter_id, constants::NOT_SPECIFIED_ID);
      assert_eq!(obj.legal_first_name, legal_first_name );
      assert_eq!(obj.legal_last_name, legal_last_name);
      assert_eq!(obj.legal_middle_names, legal_middle_names);
      assert_eq!(obj.phn, phn);
      assert_eq!(obj.birth_date, birth_date);
      assert_eq!(obj.location_id,location_id);
      assert_eq!(obj.location_short_name,location_short_name);  
      assert_eq!(obj.admit_timestamp, admit_timestamp);
      assert_eq!(obj.admit_notes, admit_notes);
      assert_eq!(obj.discharge_timestamp, discharge_timestamp);
      assert_eq!(obj.discharge_notes, discharge_notes);

      let tmp_prompt = "unit test";
      let form_errors = "unit test";

      // quick win: test copy from an AdmitDataForm
      let frm = AdmitDataForm{
          patient_id: patient_id.to_string(),
          patient_first_name: legal_first_name.clone(),
          patient_last_name: legal_last_name.clone(),
          patient_middle_name: legal_middle_names.clone(),
          phn: phn.to_string(),
          birthdate: birth_date.to_string(),
          encounter_id: constants::NOT_SPECIFIED_ID.to_string(),
          location_id: location_id.to_string(),
          action_flag: "admit".to_string(),
          admit_notes: admit_notes.clone(),
          user_prompt: tmp_prompt.to_string(),  
          form_errors: form_errors.to_string(), 
      };

      let p = Patient::to_patient(frm.clone());
      println!("...birth_date.to_string(): {}", birth_date.to_string());
      println!("...frm.birth_date: {}", frm.birthdate);
      println!("...p.birth_date: {}", p.birth_date);

      // NaiveDateTime.to_string() uses the system date time format, which we've overridden for the system
      assert!(ConvertUtils::is_equal_to_yyyy_mm_dd_hh_mm_ss(p.birth_date, birth_date), "Birth dates do not match: {} <> {}", p.birth_date, birth_date);

      assert_eq!(p.admit_notes, admit_notes.clone());
      assert_eq!(p.id, patient_id); 
      assert_eq!(p.encounter_id, constants::NOT_SPECIFIED_ID);
      assert_eq!(p.legal_first_name, legal_first_name );
      assert_eq!(p.legal_last_name, legal_last_name);
      assert_eq!(p.legal_middle_names, legal_middle_names);
      assert_eq!(p.phn, phn);      
      assert!(p.location_id == location_id, "Location Ids do not match");
      //assert!(p.location_short_name == location_short_name, "location short_namea do not match: {} <> {}", p.location_short_name, location_short_name);

      assert!(ConvertUtils::is_equal_to_yyyy_mm_dd_hh_mm_ss(p.admit_timestamp, admit_timestamp), "admit_timestamps do not match: {} <> {}", p.admit_timestamp, admit_timestamp);

      assert!(p.admit_notes == admit_notes, "admit_notes do not match");

      match p.discharge_timestamp {
          Some( ts) => {
              match discharge_timestamp {
                  Some( ts2) => {
                      assert!(ConvertUtils::is_equal_to_yyyy_mm_dd_hh_mm_ss(ts, ts2), "discharge_timestamps do not match: {} <> {}", ts, ts2);
                  }
                  None => {
                      assert!(false, "Test Fail: discharge_timestamp is nothing, but p.discharge_timestamp exists");
                  }
              }
          }
          None => {
              match discharge_timestamp {
                  Some( _ts3) => {
                      //assert!(false, "Test Fail: p.discharge_timestamp is nothing, but discharge_timestamp exists");
                      assert!(true, "This is okay because AdmitDataForm does not have a Discharge Timestamp");
                  }
                  None => {
                      assert!(true, "Both dates are nothing");
                  }
              }
          }
      };

      // Empty frm.discharge_notes is okay because AdmitDataForm does not have a Discharge Notes
      assert!(p.discharge_notes == "".to_string(), "discharge_notes do not match: ({}) ({})", p.discharge_notes, discharge_notes);


      // quick win: test copy from an AdmitDataForm
      let mut frm2 = frm.clone();
      frm2.birthdate = "INVALID DATE FOR TEST".to_string();

      let tmp_current_datetime = Utc::now().naive_utc();
      let p2 = Patient::to_patient(frm2);
      assert!(ConvertUtils::is_equal_to_yyyy_mm_dd_hh_mm_ss(p2.birth_date, tmp_current_datetime), "Invalid Birth Date not coerced correctly: {} <> {}", p2.birth_date, tmp_current_datetime);


      assert_eq!(obj.birth_date_for_display(), birth_date.format("%Y-%b-%d").to_string());

      // check invalid PHN output
      assert_eq!(obj.phn_to_string(), phn.to_string()); // test before with a valid PHN
      obj.phn = constants::NOT_SPECIFIED_ID;
      assert_eq!(obj.phn_to_string(), "".to_string()); // test after

      assert_eq!(obj.admit_timestamp_for_display(), admit_timestamp.format("%Y-%b-%d %H:%M").to_string());
      

      obj.birth_date = fixed_birth_date;
      assert_eq!(obj.age(), ((Utc::now().naive_utc() - fixed_birth_date).num_days() / 365).to_string());

      obj.id = 1;
      obj.legal_first_name = "UNIT TEST".to_string();
      obj.legal_last_name = "UNIT TEST".to_string();
      let tmp_patient_to_string = "(patient Id: 1\nlegal_first_name: UNIT TEST\nlegal_last_name: UNIT TEST)".to_string(); 
      assert_eq!(obj.to_string(), tmp_patient_to_string);
}

/// ### test_create_user_dto()
/// 
/// Tests the ability to create a User DTO
///
#[test]
fn test_create_user_dto() {
      let current_time: NaiveDateTime = Utc::now().naive_utc();
      let mut rng = rng();

      let user_id: i64 = rng.random();
      let name: String = DataGenerator::get_first_name(100);
      let user_name: String = DataGenerator::get_last_name(20);
      let email: String = DataGenerator::get_last_name(80) + &"@maple.com";
      let created_timestamp: NaiveDateTime = current_time;
      let password: String = DataGenerator::get_last_name(80) + &"!abcde"; // terribly poor actual security practise; good enough for basic DAO testing, at this time

      let obj = User::new (
        user_id.clone(), // user_id
        name.clone(),
        user_name.clone(),
        email.clone(),
        created_timestamp.clone(),
        password.clone()
      );

      assert_eq!(obj.id, user_id); 
      assert_eq!(obj.name, name); 
      assert_eq!(obj.user_name, user_name); 
      assert_eq!(obj.email, email); 
      assert_eq!(obj.created_timestamp, created_timestamp); 
      assert_eq!(obj.password, password); 
}

/// ### test_create_user_auth_and_permission_dto()
/// 
/// Tests the ability to create a UserAuthorization DTO and its basid methods:
/// * has_permission()
/// * has_permission_for_dept()
///
#[test]
fn test_create_user_auth_and_permission_dto() {
    let basic_perms= [(1, 2), (2, 1)];

    let mut perms: Vec<Permission> = Vec::with_capacity( basic_perms.len() );
    for p in basic_perms {
        perms.push(
            Permission {
                department_id: p.0,
                permission_id: p.1,
            }
        );
    }

    let result = UserAuthorization {
        granted_permissions: perms.clone()
    };
    
    assert!(result.granted_permissions.len() == 2, "UserAuthorization should have 2 permissions");

    assert!(result.has_permission(1));
    assert!(!result.has_permission(9999));

    // test department level permissions
    assert!(result.has_permission_for_dept(1,2));    // should succeed
    assert!(!result.has_permission_for_dept(1,999)); // should fail
    assert!(!result.has_permission_for_dept(999,1)); // should fail

    let ua: UserAuthorization = UserAuthorization::new(perms);
    assert!(ua.granted_permissions.len() > 0);
}

/// ### test_encounter_dto()
/// 
/// Tests the ability to create an Encounter DTO
///
#[test]
fn test_encounter_dto(){   
    let current_time: NaiveDateTime = Utc::now().naive_utc();
    let mut rng = rng();
    let tmp_id: i64 = rng.random();

    let tmp_admit_notes : String = DataGenerator::get_first_name(2000);
    let tmp_admit_timestamp : NaiveDateTime = current_time;
    let tmp_discharge_notes:  String = DataGenerator::get_first_name(2000);
    let tmp_discharge_timestamp: NaiveDateTime = Utc::now().naive_utc();
    let tmp_patient_id = rng.random();
    let tmp_encounter_site_name = "TEST SITE NAME".to_owned();
    let tmp_room_identifier = "R00-999".to_owned();
    let tmp_is_current_encounter: String = "Y".to_owned();

    let obj = Encounter{
      id: tmp_id,
      admit_notes: tmp_admit_notes.clone(),
      admit_timestamp: tmp_admit_timestamp,
      discharge_notes: tmp_discharge_notes.clone(),
      discharge_timestamp: Some(tmp_discharge_timestamp),
      patient_id: tmp_patient_id,
      encounter_site_name: tmp_encounter_site_name.clone(),
      room_identifier: tmp_room_identifier.clone(),
      is_current_encounter: tmp_is_current_encounter.clone()
    };

    assert_eq!(obj.id, tmp_id); 
    assert_eq!(obj.admit_notes, tmp_admit_notes); 
    assert_eq!(obj.admit_timestamp, tmp_admit_timestamp); 
    assert_eq!(obj.discharge_notes, tmp_discharge_notes); 
    assert_eq!(obj.discharge_timestamp,  Some(tmp_discharge_timestamp)); 
    assert_eq!(obj.patient_id, tmp_patient_id); 
    assert_eq!(obj.encounter_site_name, tmp_encounter_site_name); 
    assert_eq!(obj.room_identifier, tmp_room_identifier); 
    assert_eq!(obj.is_current_encounter, tmp_is_current_encounter); 
  }

/// ### test_feature_preference_dto()
/// 
/// Tests the ability to create a FeaturePreference DTO
///
  #[test]
fn test_feature_preference_dto(){
let mut rng = rng();

let tmp_id: i64 = rng.random();
let tmp_display_order: i64 = 99999999;
let tmp_weight: i64 = 0;
let tmp_calculation_date: NaiveDateTime = Utc::now().naive_utc();
let tmp_users_id: i64 = rng.random();
let tmp_department_id: i64 = rng.random();
let tmp_feature_id: i64 = rng.random();
let tmp_ref_group_id: i64 = rng.random();
let tmp_ref_name: String = "REFERENCE NAME".to_string();

let obj = FeaturePreference {
    id: tmp_id, 
    display_order: tmp_display_order,
    weight: tmp_weight,
    calculation_date: tmp_calculation_date,
    users_id: tmp_users_id,
    department_id: tmp_department_id,
    feature_id: tmp_feature_id,
    ref_group_id: tmp_ref_group_id,
    ref_name: tmp_ref_name.clone()
};

assert_eq!(obj.id, tmp_id); 
assert_eq!(obj.display_order, tmp_display_order); 
assert_eq!(obj.weight, tmp_weight); 
assert_eq!(obj.calculation_date, tmp_calculation_date); 
assert_eq!(obj.users_id, tmp_users_id); 
assert_eq!(obj.department_id, tmp_department_id); 
assert_eq!(obj.feature_id, tmp_feature_id); 
assert_eq!(obj.ref_group_id, tmp_ref_group_id); 
assert_eq!(obj.ref_name, tmp_ref_name); 
}

/// ### test_permissions_dto()
/// 
/// Tests the ability to create a Permission DTO
///
 #[test]
fn test_permissions_dto() {
    let mut rng = rng();
    let tmp_department_id: i64 = rng.random();
    let tmp_permission_id: i64 = rng.random();

    let obj = Permission {
        department_id: tmp_department_id,
        permission_id: tmp_permission_id
    };

    assert_eq!(obj.department_id, tmp_department_id); 
    assert_eq!(obj.permission_id, tmp_permission_id); 

    let p : Permission = Permission::new( constants::INVALID_OTHER_ID, constants::INVALID_OTHER_ID);
    assert_eq!(p.department_id, constants::INVALID_OTHER_ID);
    assert_eq!(p.permission_id, constants::INVALID_OTHER_ID);
}
