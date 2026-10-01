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

//use actix_web::cookie::time::format_description::modifier::End;
use sqlx::postgres::{PgPoolOptions}; 

use maple_emr::constants;
use maple_emr::dao::patient_dao::PatientDAO;

use maple_emr::nle::controller::CommandController;
use maple_emr::ui::nle_command_fmt::NLECommandFormatter;
use maple_emr::nle::nle::*;

use maple_emr::dto::user_auth::Permission;
use maple_emr::dto::user_auth::UserAuthorization;

use common::entity_factory::EntityFactory;

//use crate::common::test_utils;
use crate::common::test_utils::DataGenerator;

pub const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

/// ### test_fmt_get_nle_options_content()
/// 
/// Tests the get_nle_options_content() method of the NLECommandFormatter struct
/// 
#[tokio::test]
async fn test_fmt_get_nle_options_content() {
    let tmp_patient = EntityFactory::create_patient();
    let p_id = tmp_patient.id; //constants::INVALID_OTHER_ID;
    let static_base_path = std::env::current_dir().expect("Base path to executable could not be found").display().to_string() + "\\data\\";
    let option_limit: usize = 3;

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
    let html_result1 = NLECommandFormatter::get_nle_options_content(test_prompts_and_matches.clone(), test_cmd, p_id, Some(tmp_patient), option_limit);
    assert_ne!(html_result1, String::new(), "Test 3: No HTML returned by NLECommandFormatter::get_nle_options_content");

    // Test 4: NLECommandFormatter::get_nle_options_content; no patient provided
    let test_nle2 = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let test_cmd2: CommandController = CommandController::new(&mapping_file_path, test_nle2);
    let html_result1 = NLECommandFormatter::get_nle_options_content(test_prompts_and_matches.clone(), test_cmd2, p_id, None, option_limit);
    assert_ne!(html_result1, String::new(), "Test 4: No HTML returned by NLECommandFormatter::get_nle_options_content");

    // Test 5: NLECommandFormatter::get_nle_options_content; no patient provided, patient_id = -1
    let test_nle3 = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let test_cmd3: CommandController = CommandController::new(&mapping_file_path, test_nle3);
    let html_result1 = NLECommandFormatter::get_nle_options_content(test_prompts_and_matches.clone(), test_cmd3, constants::INVALID_PATIENT_ID, None, option_limit);
    assert_ne!(html_result1, String::new(), "Test 5: No HTML returned by NLECommandFormatter::get_nle_options_content");

    // Test 5b: NLECommandFormatter::get_nle_options_content; no patient provided, patient_id = -1
    let test_nle5b = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let test_cmd5b: CommandController = CommandController::new(&mapping_file_path, test_nle5b);
    let html_result1 = NLECommandFormatter::get_nle_options_content(test_prompts_and_matches.clone(), test_cmd5b, 100000, None, option_limit);
    assert_ne!(html_result1, String::new(), "Test 5b: No HTML returned by NLECommandFormatter::get_nle_options_content");

    // Test 6: NLECommandFormatter::get_nle_options_content; no patient provided, patient_id = -1; NO ITEMS
    let test_nle4 = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let test_cmd4: CommandController = CommandController::new(&mapping_file_path, test_nle4);
    let html_result1 = NLECommandFormatter::get_nle_options_content(Vec::new(), test_cmd4, constants::INVALID_PATIENT_ID, None, option_limit);
    assert_ne!(html_result1, String::new(), "Test 6: No HTML returned by NLECommandFormatter::get_nle_options_content");

    // Test 7: Test the results of is_command_allowed_at_context_level() permission id, current context, required context, expected_true_value
    let truth_variations = vec![(4, 0, true),  (12, 0,  true),  (9999, 0,  true),  (100000, 0,  true), // row is for current level=CONTEXT_LEVEL_NO_PATIENT_REQUIRED
                                                      (4, 1, false), (12, 1,  true),  (9999, 1,  false), (100000, 1,  true), // row is for current level=CONTEXT_LEVEL_REQUIRES_PATIENT
                                                      (4, 2, false), (12, 2,  false), (9999, 2,  false), (100000, 2,  true), // row is for current level=CONTEXT_LEVEL_REQUIRES_PATIENT_INTERVENTION
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

    let user_prompt: String = "transfer john smith".to_string();

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
}


/// ### test_nle_get_referenced_patient()
/// 
/// Tests the get_referenced_patient() method of the CommandController struct
/// 
#[tokio::test]
async fn test_nle_get_referenced_patient() {
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
    let mut user_prompt: String = "transfer john smith".to_string();
    let patients_list = {PatientDAO::new( db_pool.clone() ).await}.get_patients_at_users_site_no_discharge( test_user_id ).await.expect( constants::DATABASE_ERROR_NOT_FOUND ).unwrap();

    // Test 9a: Patient should NOT be found
    let mut referenced_patient: (i8, Option<maple_emr::dto::patient::Patient>) = CommandController::get_referenced_patient(patients_list.clone(), test_user_id, user_prompt.clone() ).await; // perform a basic search within the prompt for any of the current patients
    assert_eq!(referenced_patient.0, CommandController::OTHER_PATIENT_FOUND, "No patient was expected, an alternative was returned");
    assert!(Some(referenced_patient.1).is_some(), "Test 9a: Target Patient not returned");
    
    // Test 9b: Patient SHOULD be found
    user_prompt = "transfer kate beaton".to_string();
    referenced_patient = CommandController::get_referenced_patient(patients_list.clone(), test_user_id, user_prompt.clone() ).await; // perform a basic search within the prompt for any of the current patients
    assert_eq!(referenced_patient.0, CommandController::TARGET_PATIENT_FOUND, "A known patient was expected, but none was returned");
    assert!(Some(referenced_patient.1).is_some(), "Test 9b: A patient was returned, when none were expected");
}


/// ### test_validate_prompt_results_clinical()
/// 
/// Tests the NL prompt with all levels of permissions for a wide array of commands that could be executed, against 2 possible phrases for each command
/// 
///    cargo test --test nle_tests test_validate_prompt_results_clinical -- --exact --nocapture
/// 
#[tokio::test]
async fn test_validate_prompt_results_clinical() {
    let static_base_path = std::env::current_dir().expect("Base path to executable could not be found").display().to_string() + "\\data\\";
    let minimlm_model_file_path = static_base_path.clone() + &"all-MiniLM-L6-v2.onnx".to_string();
    let tokenizer_file_path = static_base_path.clone() + &"tokenizer.json".to_string();
    let mapping_file_path = static_base_path.clone() + &"command_mapping.csv".to_string();

    // populate the prompt with permissions a clinical user would have
    //   clinical users are ids: 1, 2, 6, 7
    //    non-clinical: 3, 4 and also 5 is very limited
    //    admin: 5
    let clinical_permissions = vec![1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12,
                                              100002, 100003, 100004, 100005, 100006, 100007, 100008, 
                                              100009, 100010, 100011, 100012, 100038, 100040, 100041, 
                                              100042, 100043, 100044, 100045, 100046, 100047, 100048   ];
    let mut ua: UserAuthorization = UserAuthorization::new(Vec::new());

    for perm in clinical_permissions{
        ua.granted_permissions.push( Permission::new(1, perm) );
    }

    let valid_prompts  = vec![
                "high fall risk on admission", "bed alarm activated and signage posted at bedside", // 100040 Alerts/CCI/SPI"
                "cci flag for aggressive behaviour", "patient has hemophilia", //  100040	"Alerts/CCI/SPI"
                "patient has severe alergy to penicillin", "allergy band applied", // 100043	"Allergies"
                "follow-up with dermatologist booked for next wednesday at 10:00", "patient reminded to bring medication and given appointment card", // 100041	"Appointments"
                "electrolytes and cbc drawn from right antecubital at 1600", "specimen labelled sent to lab", // 100002	"Collect Specimen: Bloodwork"
                "suspected dehydration, iv fluids started", "urine output reasssessment in four hours", // 2	"create-clinical-intervention"
                "interpreter services requeted for family", "social worker notified for children at home", // 3	"create-non-clinical-intervention"
                "patient admitted to medical unit from ed for acute poisoning", "admission ordered", // 4	"create-update-admit"
                "primary diagnosis confirmed as community-acquired covid-19", "secondary diagnosis of pneumonia noted", // 100042	"Diagnoses"
                "signed consent form scanned added to chart", "discharge summary uploaded for general physician", // 100046	"Documents"
                "influenza vaccine administered to right deltoid", "no reaction observed to influenza vaccine after 15 minutes", // 100045	"Immunizations"
                "incorrect permissions at log in", "password reset and access restored", // 1	"login"
                "all done", "end of shift log out", // 1	"logout"
                "patient reports low mood and poor sleep for two weeks", "referral made to the mental health liaison team", // 100047	"Mental Health and Wellness"
                "family requested to speak with attending physician", "message left for physician on call", // 100012	"Other"
                "patient ported to ed with a walker and standby assist", "daily gait training to continue as part of physio", // 100048	"Physiotherapy"
                "patient returned with minimal chest drainage", "chest tubes in place following cabg", // 100008	"Procedure: (Cardiovascular) Open Heart Surgery"
                "2 mg morphine administered iv for pain rated 6/10", "pain reassessed at 3/10 pain after 60 minutes", // 100005	"Procedure: Administer Medication"
                "one unit of type ab started at 1400", "vitals stable with no signs of reaction to transfusion", // 100010	"Procedure: Blood Transfusion"
                "bp 144/90, rr 17, hr 60, spo2 96% on room air", "temperature 37.2 degrees and patient was afebrile", // 100038	"Procedure: Collect Vitals"
                "ct head completed, no contrast", "abnormal acute intracranial reported", // 100003	"Procedure: CT Scan"
                "3cm laceration to left forearm closed with four sutures", "patient to return in 10 days for removal", // 100009	"Procedure: General Suture"
                "mri lumbar spine completed this morning", "radiology report review pending", // 100004	"Procedure: Magnetic Resonance Imaging (MRI)"
                "portable chest x-ray done at bedside", "findings show left lower lobe dispersement", // 100011	"Procedure: X-Ray"
                "worsening oxygen levels, patient transfer to icu requested", "bed confirmed and report given to receiving porter", // 100006	"Support Request: Patient Transfer"
                "referral sent to nephrology for declining creatinine", "consult expected by 23:00", // 100007	"Support Request: Physician Referral"
                "iv fluid rate increased to 100 ml/hr", "patient now managing oral fluids well", // 6	"update-clinical-intervention"
                "interpreter meeting rescheduled to tomorrow afternoon", "patient agreed to the new time", // 7	"update-non-clinical-intervention"
                "review admission records prior to rounds", "patient admitt for copd exacerbation", // 8	"view-admit"
                "review recent labs and vitals trend", "potassium remains low and needs replacement", // 9	"view-any-clinical-data"
                "check active changes to care plan", "wound dressing change at 1700", // 10	"view-clinical-intervention"
                "discharge summary agrees with previous admission", "patient was sent home on oral antibiotics and referral to gp", // 11	"view-discharge"
                "check patient orders", "update next of kin address" // 12	"view-non-clinical-intervention" 
            ];

    println!("This is test test_validate_prompt_results_clinical()");
    println!("  Providing supporting evidence of executability of the NL module integrated into MapleHMS\n");
    println!("  Supporting Use Case: 7.0 Natural Language (NL) Prompt\n");
    println!("  Execute via DOS: cargo test --test nle_tests test_validate_prompt_results_clinical -- --exact --nocapture > nle_test_results_2026-MM-DD_HHMM.log");

    let test_nle = NaturalLanguageEngine::new(&minimlm_model_file_path, &tokenizer_file_path).await;
    let mut test_cmd: CommandController = CommandController::new(&mapping_file_path, test_nle);

    let start_time = DataGenerator::now();

    let mut num_of_runs = 0;
    let expected_iteration_count = valid_prompts.len();

    // run every prompt for every context level
    for pmp in valid_prompts{
        for context_level in 0..=2 {
            let rankings:  Vec<(String, f32)> = test_cmd.get_filtered_classifier_rankings(pmp.to_string(), ua.clone(), context_level).await; // level 0

            println!("..Prompt: {} @ Level: {}", pmp, context_level);
            for item in rankings.clone() {
                println!("....'{}': ({:.1}%)", item.0, item.1 * 100.);
            }
            if rankings.clone().len() == 0 {
                assert!(false);
            }
            num_of_runs = num_of_runs + 1;
        }
    }
    let end_time = DataGenerator::now();
    println!("\nStart time: {}", start_time);
    println!("End time: {}", end_time);
    println!("No. of Runs where results were produced: {}", num_of_runs);
    assert_eq!(num_of_runs, expected_iteration_count, "Number of successful runs ({}) did not match expected number ({})", num_of_runs, expected_iteration_count);
}