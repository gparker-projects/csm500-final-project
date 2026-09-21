///
/// Unit & Integration tests for the data_forms crate of the UI module
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 

mod common;

use std::ops::IndexMut;

#[cfg(test)]
use maple_emr::constants;
use maple_emr::dto::encounter::Encounter;
use maple_emr::dto::user_auth::Permission;

use maple_emr::ui::data_forms::*;

use common::entity_factory::EntityFactory;
use common::test_utils::DataGenerator;

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

///
/// Tests InterventionDataFormBasic struct methods
/// 
#[test]
fn test_intervention_data_form_basic  () {
    let mut frm = InterventionDataFormBasic {
        intervention_type_id: "-1".to_string(),
        encounter_id: "-1".to_string(),
        patient_id: "-1".to_string(),
    };

    assert_eq!(frm.get_patient_id_as_i64(), -1);

    frm.patient_id = "1".to_string();
    assert_eq!(frm.get_patient_id_as_i64(), 1);
}

///
/// Tests InterventionDetailsDataForm struct methods
/// 
#[test]
fn test_intervention_details_data_form_basic  () {
    let mut frm = InterventionDetailsDataForm {
        type_id: "-1".to_string(),
        ..Default::default()
    };

    assert_eq!(frm.get_type_id_as_i64(), -1);

    frm.type_id = "1".to_string();
    assert_eq!(frm.get_type_id_as_i64(), 1);
}