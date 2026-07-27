use chrono::{DateTime, Utc};

mod dto{
    /// -------------------------------------------------------------------
    /// Defines a Data Transfer Object for a (Patient) Encounter, which represents
    /// an event whereby a patient has attended the hospital to have one or 
    /// more interventions applied to them.
    /// -------------------------------------------------------------------
    /// 
    #[derive(serde::Deserialize)]
    pub struct Encounter {
        #[serde(rename = "Id")]
        Id: Integer, // D BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
        #[serde(rename = "AdmitNotes")]
        AdmitNotes: String, // ADMIT_NOTES VARCHAR(2000),
        #[serde(rename = "AuditTimestamp")]
        AuditTimestamp: DateTime, // ADMIT_TIMESTAMP TIMESTAMP DEFAULT NOW(),
        #[serde(rename = "DischargeNotes")]
        DischargeNotes: String, // DISCHARGE_NOTES VARCHAR(2000),  
        #[serde(rename = "DischargeTimestamp")]
        DischargeTimestamp: DateTime, // DISCHARGE_TIMESTAMP TIMESTAMP, 
        #[serde(rename = "PatientId")]
        PatientId: Integer,//       PATIENT_ID BIGINT REFERENCES PATIENT (ID),
        #[serde(rename = "InterventionId")]
        InterventionId: Integer, //   INTERVENTION_ID BIGINT REFERENCES INTERVENTION (ID),
        #[serde(rename = "SiteId")]
        SiteId: Integer //     SITE_ID BIGINT REFERENCES SITE (ID)
    } 

    impl Encounter {
        /// Basic constructor
        /// 
        pub fn new(Id: Integer,
                AdmitNotes: String,
                AuditTimestamp: DateTime,
                DischargeNotes: String,
                DischargeTimestamp: DateTime,
                PatientId: Integer,
                InterventionId: Integer,
                SiteId: Integer
                ) -> Self {
            Self { 
                Id,
                AdmitNotes,
                AuditTimestamp,
                DischargeNotes,
                DischargeTimestamp,
                PatientId,
                InterventionId,
                SiteId
            }
        }
    }
}