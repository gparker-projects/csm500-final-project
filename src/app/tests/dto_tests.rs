///
/// Unit & Integration tests for the DTO module
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 
/// 

//use MapleEMR::dto::patient::Patient;
//use MapleEMR::dto::intervention::Intervention;
use chrono::{Utc, NaiveDateTime};
use rand::{Rng, RngExt, rng};
use MapleEMR::{constants, dto::{encounter::*, intervention::*, intervention_detail::*, patient::*, user::*}};

use MapleEMR::dto::user_auth::*;

mod common; // set up per: https://doc.rust-lang.org/book/ch11-03-test-organization.html

use common::test_utils::*; 


#[cfg(test)] 

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
  }

  ///
  /// Prove the Intervention DTO works/continues to work
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

    //DataGenerator::get_location_short_name(16);

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
            room_identifier.clone()
        );

    assert_eq!(obj.id, intervention_id);
    assert_eq!(obj.encounter_id, encounter_id);
    assert_eq!(obj.description, description);
    assert_eq!(obj.notes, notes);
    assert_eq!(obj.location_id, location_id);

    assert_eq!(users_id, users_id);
    assert_eq!(intervention_type_id, intervention_type_id);
    assert_eq!(status_id, status_id);
    assert_eq!(intervention_type, intervention_type);
    assert_eq!(room_identifier, room_identifier);
  }

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
  }

  ///
  /// Prove the Patient DTO works/continues to work; includes the Encounter table details
  /// 
  /// REF: Random number gerneration for tests: Zero-to-prod, page 159; now deprecated apparently.
  /// 
  #[test]
  fn test_create_patient_dto() {
    let current_time: NaiveDateTime = Utc::now().naive_utc();
    let mut rng = rng();

    let patient_id: i64 = rng.random();
    let legal_first_name: String = DataGenerator::get_first_name(100);
    let legal_last_name: String = DataGenerator::get_last_name(100);
    let legal_middle_names: String = DataGenerator::get_middle_names(100);
    let phn: i64 = DataGenerator::get_phn();
    let birth_date: NaiveDateTime = DataGenerator::get_date();
    let location_id: i64 = rng.random();
    let location_short_name: String =  DataGenerator::get_lorem_ipsum(16);
    let admit_timestamp: NaiveDateTime = current_time;
    let admit_notes: String = DataGenerator::get_lorem_ipsum(2000);
    let discharge_timestamp: Option<NaiveDateTime> = Some( DataGenerator::add_random_seconds(current_time, 3600, 36000) );
    let discharge_notes: String = DataGenerator::get_lorem_ipsum(2000);

    // instantiate a DTO to prove it accepts data, but more importantly, detect unexpected changes to it that will break the application
     let obj = Patient::new(
        patient_id.clone(), // patient_id
        constants::NOT_SPECIFIED_ID, //encounter_id, 
        legal_first_name.clone(), //legal_first_name, 
        legal_last_name.clone(), //legal_last_name, 
        legal_middle_names.clone(), //legal_middle_names, 
        phn.clone(), //phn, 
        birth_date.clone(), //birth_date,
        location_id.clone(), //location_id,
        location_short_name.clone(), //location_short_name, 
        admit_timestamp.clone(), //discharge_timestamp,
        admit_notes.clone(), //admit_note
        discharge_timestamp.clone(),
        discharge_notes.clone()//discharge_notes
    );

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
  }

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

    #[test]
  fn test_create_user_auth_and_permission_dto() {
    let mut basic_perms= [(1, 2), (2, 1)];

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
        granted_permissions: perms
    };
    
    assert!(result.has_permission(1));
    assert!(!result.has_permission(9999));

    // test department level permissions
    assert!(result.has_permission_for_dept(1,2));    // should succeed
    assert!(!result.has_permission_for_dept(1,999)); // should fail
    assert!(!result.has_permission_for_dept(999,1)); // should fail
  }