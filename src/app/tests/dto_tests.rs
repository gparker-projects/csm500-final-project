///
/// Unit & Integration tests for the DTO module
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 
#[cfg(test)]
mod tests {

  use MapleEMR::{dto::*, dto::{self, intervention}};

  #[test]
  fn test_dtos() {

    // instantiate a DTO to prove it accepts data, but more importantly, detect unexpected changes to it that will break the application
    let obj = MapleEMR::dto::intervention::Intervention::new( 0,//  id,
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

}