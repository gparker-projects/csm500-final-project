///
/// Unit & Integration tests for the DAO module
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 
#[cfg(test)]
mod tests {

  use MapleEMR::{dao::*, dao::{self, patientdao}};
  use MapleEMR::{dto::*, dto::{self, patient}};
  //use MapleEMR::dao::patientdao::dao::PatientDAO;

  const DB_CONN_STR: &str = "postgres://postgres:csm500@localhost:5432/csm500";

  #[test]
  pub fn test_daos() {
    let test_user_id = 2;

    // instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
    let dao = MapleEMR::dao::patientdao::dao::PatientDAO::new( DB_CONN_STR );
  
    //let qry_results = dao.get_assigned_patients(test_user_id, false).await.expect( &errors::DatabaseError::NotFoundError.to_string() );
    let qry_results: Option< Vec<dto::patient::dto::Patient> > = dao.get_assigned_patients(test_user_id, false);

    match qry_results {
        Some (patient_list) => {
          println!("Retrieved SOME patients");
          assert!(true);
        }
        None => {
          println!("No patients");
          assert!(false);
        }
    }

    //assert_eq!(obj.notes, "notes".to_string()); 
  }

}