///
/// Unit & Integration tests for the DAO module
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 
#[cfg(test)]
mod tests {
  use MapleEMR::dao::patient_dao::PatientDAO;
  use MapleEMR::constants;

  #[tokio::test]
  async fn test_get_assigned_patients() {
    let test_user_id = 2;

    // instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
    let dao = PatientDAO::new( constants::DB_CONN_STR );
    let qry_results = dao.await.get_assigned_patients(test_user_id, false).await;

    match qry_results.unwrap() {
        Some (patient_list) => {
          println!("Retrieved {} patients", patient_list.len());
          assert_eq!(patient_list.len(), 3);
        }
        None => {
          println!("No patients");
          assert!(false);
        }
    }
  }

}