
/*
References:
 https://bitskingdom.com/blog/web-apps-rust-performance-optimization/


#[actix_web::test]
use reqwest;
use tokio;

#[cfg(test)] 

/// ### test_create_encounter_dto()
/// 
/// Tests the ability to create an Encounter DTO and its basic methods:
/// * admit_timestamp_for_display()
/// * to_string() - trait override
/// 
#[test]
async fn test_reqwest() {
    let response = reqwest::get("/")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("Response: {}", response);
}*/