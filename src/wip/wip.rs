discharge mary mallon immediately, she is infecting everyone with the plague


https://docs.rs/rust-bert/latest/rust_bert/


///
/// 
/// 
/// https://docs.rs/rust-bert/latest/rust_bert/pipelines/index.html
///

/*use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};

pub struct BERTEngine {}

impl BERTEngine {

    pub fn QAExample() -> String{
        let qa_model = QuestionAnsweringModel::new(Default::default())?;

        let question = String::from("Where does Amy live ?");
        let context = String::from("Amy lives in Amsterdam");

        let answers = qa_model.predict(&[QaInput { question, context }], 1, 32);

        return answers.to_string();
    }
}*/

----------------------------------


    /// ### get_patients_at_users_site_no_discharge()
    ///    Finds and returns any patients that are currently assigned to the user
    /// 
    /// #### Returns:
    /// * Option< Vec<Patient>: a vector of Patients, if found
    /// * sqlx::Error: An error, if applicable
    /// 
    /// #### Refs
    /// * https://docs.rs/sqlx/latest/sqlx/fn.query_as.html
    /// * https://stackoverflow.com/questions/67243108/mapping-nm-relations-into-vec-using-sqlx
    /// * https://doc.rust-lang.org/std/io/struct.Error.html - for return Error
    /// 
    
    /// ### PatientDAO::new()
    ///    Creates a new Patient Data Access Object, with a database pool for use by other calls
    /// 
    /// #### Parameters:
    /// * db_connection (PgPool): a PgPool for establishing a database connection
    /// 
    /// #### Returns:
    /// * PatientDAO: the PatientDAO object that was created
    /// 
    
    /// ### get_patient_details()
    ///    Finds and returns the data for a specific patient, as a Patient struct
    /// 
    /// #### Parameters:
    /// * _user_id (i64): the id of the user making the data request, for audit purposes
    /// * patient_id (i64): the id of the patient to be obtained
    /// 
    /// #### Returns:
    /// * Option<Patient>: the Patient, if found
    /// * sqlx::Error: An error, if applicable
    /// 
    
    
    /// #### Parameters:
    /// * form (AdmitDataForm): an AdmitDataForm object describing the data to be used for the update
    /// * _audit_user_id (i64): the id of the user making the data request, for audit purposes
    /// 