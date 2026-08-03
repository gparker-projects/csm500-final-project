pub mod dto{
    use chrono::{DateTime, Local};

    /// -------------------------------------------------------------------
    /// Defines a Data Transfer Object for a (Patient) Encounter, which represents
    /// an event whereby a patient has attended the hospital to have one or 
    /// more interventions applied to them.
    /// -------------------------------------------------------------------
    /// 
    #[derive(serde::Deserialize)]
    pub struct Encounter {
        #[serde(rename = "id")]
        id: u32, // D BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
        #[serde(rename = "AdmitNotes")]
        admit_notes: String, // ADMIT_NOTES VARCHAR(2000),
        #[serde(rename = "AuditTimestamp")]
        audit_timestamp: DateTime<Local>, // ADMIT_TIMESTAMP TIMESTAMP DEFAULT NOW(),
        #[serde(rename = "DischargeNotes")]
        discharge_notes: String, // DISCHARGE_NOTES VARCHAR(2000),  
        #[serde(rename = "DischargeTimestamp")]
        discharge_timestamp: DateTime<Local>, // DISCHARGE_TIMESTAMP TIMESTAMP, 
        #[serde(rename = "PatientId")]
        patient_id: u32,//       PATIENT_ID BIGINT REFERENCES PATIENT (ID),
        #[serde(rename = "InterventionId")]
        intervention_id: u32, //   INTERVENTION_ID BIGINT REFERENCES INTERVENTION (ID),
        #[serde(rename = "SiteId")]
        site_id: u32 //     SITE_ID BIGINT REFERENCES SITE (ID)
    } 

    impl Encounter{
        /// Basic constructor
        /// 
        pub fn new(id: u32,
                admit_notes: String,
                audit_timestamp: DateTime<Local>,
                discharge_notes: String,
                discharge_timestamp: DateTime<Local>,
                patient_id: u32,
                intervention_id: u32,
                site_id: u32
                ) -> Self {
            Self { 
                id,
                admit_notes,
                audit_timestamp,
                discharge_notes,
                discharge_timestamp,
                patient_id,
                intervention_id,
                site_id
            }
        }
    }
}