///
/// Unit & Integration tests for the DTO module
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 
#[cfg(test)]
mod tests {

  use MapleEMR::dto::intervention::dto::Intervention;
  use MapleEMR::dto::patient::dto::Patient;
  use chrono::{Utc, NaiveDateTime};

    ///
  /// Prove the Intervention DTO works/continues to work
  /// 
  #[test]
  fn test_create_intervention_dto() {

    // instantiate a DTO to prove it accepts data, but more importantly, detect unexpected changes to it that will break the application
    let obj = Intervention::new( 0,//  id,
              "intervention_code".to_string(), //  intervention_code,
              "description".to_string(), //  description,
              "notes".to_string(), //  notes,
              1,//  location_id,
              2//   users_id
            );

    assert_eq!(obj.id, 0); 
    assert_eq!(obj.intervention_code, "intervention_code".to_string()); 
    assert_eq!(obj.description, "description".to_string()); 
    assert_eq!(obj.notes, "notes".to_string()); 
    assert_eq!(obj.location_id, 1); 
    assert_eq!(obj.users_id, 2); 
  }

  ///
  /// Prove the Patient DTO works/continues to work; includes the Encounter table details
  /// 
  #[test]
  fn test_create_patient_dto() {

    let current_time: NaiveDateTime = Utc::now().naive_utc();

    // instantiate a DTO to prove it accepts data, but more importantly, detect unexpected changes to it that will break the application
    let obj = Patient::new(
        0, // patient_id
        1, //encounter_id, 
        "DUMMY1".to_string(), //legal_first_name, 
        "DUMMY2".to_string(), //legal_last_name, 
        "DUMMY3".to_string(), //legal_middle_names, 
        11111111, //sin, 
        current_time, //birth_date,
        1, //location_id,
        current_time, //admit_timestamp, 
        "DUMMY4".to_string(), //admit_notes,
        Some(current_time), //discharge_timestamp,
        "DUMMY5".to_string() //discharge_notes
    );
    assert_eq!(obj.id, 0); 
    assert_eq!(obj.encounter_id, 1); 
    assert_eq!(obj.legal_first_name, "DUMMY1".to_string()); 
    assert_eq!(obj.legal_last_name, "DUMMY2".to_string()); 
    assert_eq!(obj.legal_middle_names, "DUMMY3".to_string());
    assert_eq!(obj.admit_notes, "DUMMY4".to_string()); 
    assert_eq!(obj.discharge_notes, "DUMMY5".to_string()); 
    assert_eq!(obj.sin, 11111111); 
    assert_eq!(obj.birth_date, current_time);
    assert_eq!(obj.admit_timestamp, current_time);
    assert_eq!(obj.discharge_timestamp, Some(current_time));
    assert_eq!(obj.location_id, 1); 
  }

}