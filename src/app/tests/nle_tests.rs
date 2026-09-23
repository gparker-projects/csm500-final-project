//! -------------------------------------------------------------------
//! Unit & Integration tests for NLECommandFormatter module. Includes:
//!
//! * NLECommandFormatter::get_nle_options_content()
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 
//! -------------------------------------------------------------------

#[cfg(test)]

mod common;

use sqlx::postgres::{PgPoolOptions}; 

use maple_emr::constants;
use maple_emr::dao::patient_dao::PatientDAO;

use maple_emr::nle::controller::CommandController;
use maple_emr::ui::nle_command_fmt::NLECommandFormatter;
use maple_emr::nle::nle::*;

use maple_emr::dto::user_auth::Permission;
use maple_emr::dto::user_auth::UserAuthorization;

use common::entity_factory::EntityFactory;

pub const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

#[tokio::test]
async fn test_fmt_get_nle_options_content() {
    let tmp_patient = EntityFactory::create_patient();
    let p_id = tmp_patient.id; //constants::INVALID_OTHER_ID;
    let static_base_path = std::env::current_dir().expect("Base path to executable could not be found").display().to_string() + "\\data\\";

    let mut test_prompts_and_matches: Vec<(String, f32)> = Vec::new();
    test_prompts_and_matches.push (("Weak Match".to_string(),      0.10));
    test_prompts_and_matches.push (("Poor Match".to_string(),      0.40));
    test_prompts_and_matches.push (("Okay Match".to_string(),      0.60));
    test_prompts_and_matches.push (("Good Match".to_string(),      0.80));
    test_prompts_and_matches.push (("Excellent Match".to_string(), 0.99));

    let minimlm_model_file_path = static_base_path.clone() + &"all-MiniLM-L6-v2.onnx".to_string();
    let tokenizer_file_path = static_base_path.clone() + &"tokenizer.json".to_string();
    let mapping_file_path = static_base_path.clone() + &"command_mapping.csv".to_string();

    // Test 1: init an NaturalLanguageEngine
    let test_nle_throwaway = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    assert!( Some(test_nle_throwaway).is_some(), "Test 1: NaturalLanguageEngine was not initialized");

    // Test 2: init a CommandController
    let test_nle_throwaway2 = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let test_cmd_throwaway: CommandController = CommandController::new(&mapping_file_path, test_nle_throwaway2);
    assert!( Some(test_cmd_throwaway).is_some(), "Test 2: CommandController was not initialized");

    let test_nle = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let test_cmd: CommandController = CommandController::new(&mapping_file_path, test_nle);

    // Test 2b: bad path to mapping file
    let bad_path = mapping_file_path.to_string() + &"_bad".to_string();
    let test_nle_2b = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let _test_cmd_2b: CommandController = CommandController::new(&bad_path, test_nle_2b);

    // Test 3: NLECommandFormatter::get_nle_options_content; patient provided
    let html_result1 = NLECommandFormatter::get_nle_options_content(test_prompts_and_matches.clone(), test_cmd, p_id, Some(tmp_patient));
    assert_ne!(html_result1, String::new(), "Test 3: No HTML returned by NLECommandFormatter::get_nle_options_content");

    // Test 4: NLECommandFormatter::get_nle_options_content; no patient provided
    let test_nle2 = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let test_cmd2: CommandController = CommandController::new(&mapping_file_path, test_nle2);
    let html_result1 = NLECommandFormatter::get_nle_options_content(test_prompts_and_matches.clone(), test_cmd2, p_id, None);
    assert_ne!(html_result1, String::new(), "Test 4: No HTML returned by NLECommandFormatter::get_nle_options_content");

    // Test 5: NLECommandFormatter::get_nle_options_content; no patient provided, patient_id = -1
    let test_nle3 = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let test_cmd3: CommandController = CommandController::new(&mapping_file_path, test_nle3);
    let html_result1 = NLECommandFormatter::get_nle_options_content(test_prompts_and_matches.clone(), test_cmd3, constants::INVALID_PATIENT_ID, None);
    assert_ne!(html_result1, String::new(), "Test 5: No HTML returned by NLECommandFormatter::get_nle_options_content");

    // Test 5b: NLECommandFormatter::get_nle_options_content; no patient provided, patient_id = -1
    let test_nle5b = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let test_cmd5b: CommandController = CommandController::new(&mapping_file_path, test_nle5b);
    let html_result1 = NLECommandFormatter::get_nle_options_content(test_prompts_and_matches.clone(), test_cmd5b, 100000, None);
    assert_ne!(html_result1, String::new(), "Test 5b: No HTML returned by NLECommandFormatter::get_nle_options_content");

    // Test 6: NLECommandFormatter::get_nle_options_content; no patient provided, patient_id = -1; NO ITEMS
    let test_nle4 = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let test_cmd4: CommandController = CommandController::new(&mapping_file_path, test_nle4);
    let html_result1 = NLECommandFormatter::get_nle_options_content(Vec::new(), test_cmd4, constants::INVALID_PATIENT_ID, None);
    assert_ne!(html_result1, String::new(), "Test 6: No HTML returned by NLECommandFormatter::get_nle_options_content");

    // Test 7: Test the results of is_command_allowed_at_context_level() permission id, current context, required context, expected_true_value
    let truth_variations = vec![(4, 0, true), (12, 0,  true),  (9999, 0,  true), (100000, 0,  true), // row is for current level=CONTEXT_LEVEL_NO_PATIENT_REQUIRED
                                (4, 1, true), (12, 1,  true),  (9999, 1,  true), (100000, 1,  true), // row is for current level=CONTEXT_LEVEL_REQUIRES_PATIENT
                                (4, 2, true), (12, 2,  false), (9999, 2,  true), (100000, 2,  true), // row is for current level=CONTEXT_LEVEL_REQUIRES_PATIENT_INTERVENTION
                               ];
    for item in truth_variations {
        let tmp_result = CommandController::is_command_allowed_at_context_level (item.0, item.1);
        assert_eq!(tmp_result, item.2, "Permission mismatch: perm ({}) + cur_ctxt ({}) = {} (expected)", item.0, item.1, item.2);
    }

    // Test 8: valid call
    let test_nle5 = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let test_cmd5: CommandController = CommandController::new(&mapping_file_path, test_nle5);
    let perm_label: (i64, String) = test_cmd5.get_permission_and_label_for_operation ( "update contact information".to_string() );
    assert_eq!(perm_label.0, 2, "Test 8.1: Retrieved wrong permission; 2 was expected");
    assert_eq!(perm_label.1, "Update Patient Information".to_string(), "Test 8.2: Retrieved wrong description, 'Update Patient Information was' expected");

    // Test 8b: invalid call
    let test_nle6 = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let test_cmd6: CommandController = CommandController::new(&mapping_file_path, test_nle6);
    let perm_label: (i64, String) = test_cmd6.get_permission_and_label_for_operation ( "".to_string() );
    assert_eq!(perm_label.0, constants::INVALID_OTHER_ID, "Test 8b.1: Retrieved wrong permission; -1 was expected");
    assert_eq!(perm_label.1, "".to_string(), "Test 8b.2: Empty string expected");

    // Test 8: get_filtered_classifier_rankings()
    //   Step 1: requires a permission-populated UserAuthorization

    let perm: Permission = Permission::new(1, 100006);
    let perm2: Permission = Permission::new(1, 100002);
    let perm3: Permission = Permission::new(1, 12);
    let perm4: Permission = Permission::new(1, 9999);
    let perm5: Permission = Permission::new(1, Permission::ALLOW_CREATE_UPDATE_ADMIT);
    let ua = UserAuthorization {
        granted_permissions: vec![perm.clone(), perm2.clone(), perm3.clone(), perm4.clone(), perm5.clone()]
    };

    let mut user_prompt: String = "transfer john smith".to_string();

    let test_nle7 = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let mut test_cmd7: CommandController = CommandController::new(&mapping_file_path, test_nle7);
    let rankings:  Vec<(String, f32)> = test_cmd7.get_filtered_classifier_rankings(user_prompt.clone() , ua.clone(), CommandController::CONTEXT_LEVEL_NO_PATIENT_REQUIRED).await; // level 0
    assert_ne!(rankings.len(), 0, "Test 7a: No rankings returned");

    let test_nle7b = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let mut test_cmd7b: CommandController = CommandController::new(&mapping_file_path, test_nle7b);
    let rankings:  Vec<(String, f32)> = test_cmd7b.get_filtered_classifier_rankings(user_prompt.clone() , ua.clone(), CommandController::CONTEXT_LEVEL_REQUIRES_PATIENT).await; // level 1
    assert_ne!(rankings.len(), 0, "Test 7b: No rankings returned");

    let test_nle7c = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let mut test_cmd7c: CommandController = CommandController::new(&mapping_file_path, test_nle7c);
    let rankings:  Vec<(String, f32)> = test_cmd7c.get_filtered_classifier_rankings(user_prompt.clone() , ua.clone(), CommandController::CONTEXT_LEVEL_REQUIRES_PATIENT_INTERVENTION).await; // level 2
    assert_ne!(rankings.len(), 0, "Test 7c: No rankings returned");

    // create with an empty command hashset... NOTE: this should never really happen as it means a user logged in without any permissions, but will trigger the unit test.
    let test_nle7d = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let mut test_cmd7d: CommandController = CommandController::new(&mapping_file_path, test_nle7d);
    test_cmd7d.unit_test_invalidate_command_hashset();
    let rankings:  Vec<(String, f32)> = test_cmd7d.get_filtered_classifier_rankings(user_prompt.clone() , ua.clone(), CommandController::CONTEXT_LEVEL_REQUIRES_PATIENT_INTERVENTION).await; // level 2
    assert_eq!(rankings.len(), 0, "Test 7d: No rankings expected to be returned");

    // Step 9: ---------------------------------  get_referenced_patient()  ---------------------------------
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_CONN_STR)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            println!("{}", e);
            assert!(false);
            panic!("{}", e)
        },
    };

    let test_user_id = 2;
    let patients_list = {PatientDAO::new( db_pool.clone() ).await}.get_patients_at_users_site_no_discharge( test_user_id ).await.expect( constants::DATABASE_ERROR_NOT_FOUND ).unwrap();

    // Test 9a: Patient should NOT be found
    let mut referenced_patient: (i8, Option<maple_emr::dto::patient::Patient>) = CommandController::get_referenced_patient(patients_list.clone(), test_user_id, user_prompt.clone() ).await; // perform a basic search within the prompt for any of the current patients
    assert_eq!(referenced_patient.0, CommandController::NO_PATIENT_FOUND, "No patient was expected, but one was returned");
    assert!(Some(referenced_patient.1).is_some(), "Test 9a: Default Patient not returned");
    
    // Test 9b: Patient SHOULD be found
    user_prompt = "transfer kate beaton".to_string();
    referenced_patient = CommandController::get_referenced_patient(patients_list.clone(), test_user_id, user_prompt.clone() ).await; // perform a basic search within the prompt for any of the current patients
    assert_eq!(referenced_patient.0, CommandController::KNOWN_PATIENT_FOUND, "A known patient was expected, but none was returned");
    assert!(Some(referenced_patient.1).is_some(), "Test 9b: A patient was returned, when none were expected");
}


