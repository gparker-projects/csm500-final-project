//! -------------------------------------------------------------------
//! Struct and implementation for creating html that formats Inverventions
//!   and Intervention Details.
//! -------------------------------------------------------------------
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 
//! -------------------------------------------------------------------

use crate::dto::{encounter::*, intervention::*};
use crate::dao::patient_dao::PatientWrapper;

use crate::ui::common_fmt::CommonFormatter;

pub struct SimpleFormatter{}

impl SimpleFormatter {

    ///
    /// Provide rendering of a list of patients, as a screen tile
    /// 
    pub fn get_home_route_summary_of_patients_tile_using_wrapper(patient_list: Vec<PatientWrapper>) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        let mut counter: i8 = 1;

        results_sbuf.push_str(&CommonFormatter::get_hidden_form("patientdtls".to_owned(), "patientDtlsFrm".to_owned()) );

        for pwrap in patient_list{
            results_sbuf.push_str("<a href=\"#\" onclick=\"redirect_to_patient("  ); 
            results_sbuf.push_str( &pwrap.patient.id.to_string() ); 
            results_sbuf.push_str("); return false;\">");
            results_sbuf.push_str(&SimpleFormatter::get_single_patient_summary( pwrap, counter));
            results_sbuf.push_str("</a><p></p>");
            counter = counter + 1;
        }

        return results_sbuf;
    }


    // -----------------------------------------------------------------------------------
    // Encounter formatters
    // -----------------------------------------------------------------------------------
    pub fn get_single_encounter_summary_tile( encounter: Encounter) -> String {
        let mut results_sbuf = String::with_capacity(100);

        results_sbuf.push_str("<table <tr><th>Admit Reason</th><th>Site/Facility</th></tr>"); 

        results_sbuf.push_str("  <tr>");
        results_sbuf.push_str("<td><a href=\"#\" onclick=\"redirect_to_enc("  ); 
        results_sbuf.push_str( &encounter.to_string() ); 
        results_sbuf.push_str("); return false;\">"); 
        results_sbuf.push_str( &encounter.admit_timestamp_for_display()); 
        results_sbuf.push_str("</a></td><td>"); 
        results_sbuf.push_str( &encounter.encounter_site_name );
        results_sbuf.push_str("</td>"); 
        results_sbuf.push_str("  </tr>\n");

        results_sbuf.push_str("  <tr><td><b>Admit Reason:<\\b>&nbsp;");
        results_sbuf.push_str( &encounter.admit_notes );
        results_sbuf.push_str("<\\td>\n  </tr>");
        
        results_sbuf.push_str("</table>");


        return results_sbuf;
    }

    ///
    /// Provide HTML for all of a Patient's encounters
    /// 
    pub fn get_encounter_list_tile(encounter_list: Vec<Encounter>) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        tracing::debug!(">get_encounter_list_tile()");

        results_sbuf.push_str(&CommonFormatter::get_hidden_form("encounterDtls".to_owned(), "encounterDtlsFrm".to_owned()) );

        results_sbuf.push_str("<table <tr><th>Admit Date</th><th>Site/Facility</th></tr>"); 

        for row in encounter_list{
            results_sbuf.push_str("  <tr>");
            results_sbuf.push_str("<td><a href=\"#\" onclick=\"redirect_to_enc("  ); 
            results_sbuf.push_str( &row.id.to_string() ); 
            results_sbuf.push_str("); return false;\">"); 
            results_sbuf.push_str( &row.admit_timestamp_for_display()); 
            results_sbuf.push_str("</a></td><td>"); 
            results_sbuf.push_str( &row.encounter_site_name );
            results_sbuf.push_str("</td>"); 
            results_sbuf.push_str("  </tr>\n");
        }
        results_sbuf.push_str("</table>");

        return results_sbuf;
    }

     pub fn get_single_patient_summary(pwrap: PatientWrapper, index: i8) -> String {
        let mut results_sbuf = String::with_capacity(500); 
	    let p = pwrap.patient;
	    let e = pwrap.current_encounter;
	    let i = pwrap.most_recent_intervention;

	    results_sbuf.push_str("<table class=\"hover-table\"><tr><td>"); 

        if index != -1 {
            let idx = index.to_string();
            results_sbuf.push_str(&idx);
            results_sbuf.push_str(")&nbsp;");
        }
	    results_sbuf.push_str("<b>");
	    results_sbuf.push_str( &p.legal_last_name ); 
	    results_sbuf.push_str(", "); 
	    results_sbuf.push_str( &p.legal_first_name );

	    results_sbuf.push_str("</b>&nbsp;PHN:<i>&nbsp;"); 
	    results_sbuf.push_str( &p.phn_to_string() );
	    results_sbuf.push_str("</i>&nbsp;");

	    results_sbuf.push_str("&nbsp;<div class='clinical-electric-blue'>DOB:<b>&nbsp;"); 
	    results_sbuf.push_str( &p.birth_date_for_display() );

	    results_sbuf.push_str("</b></div>&nbsp;[");

	    results_sbuf.push_str( &p.age() );
	    results_sbuf.push_str("yrs]&nbsp;@");

	    results_sbuf.push_str( &e.room_identifier );
	    results_sbuf.push_str("</td></tr>");

	    results_sbuf.push_str("<tr><td>");
	    results_sbuf.push_str("Admitted: ");
	    results_sbuf.push_str(&p.admit_timestamp_for_display() );

	    if i.is_some() {
	       let tmp_intv = i.unwrap();

	       results_sbuf.push_str("&nbsp;");
	       results_sbuf.push_str( &tmp_intv.intervention_type );
	       results_sbuf.push_str("&nbsp;@&nbsp;");
	       results_sbuf.push_str( &tmp_intv.scheduled_timestamp_for_display() );

	       results_sbuf.push_str("&nbsp;(");
	       results_sbuf.push_str( &tmp_intv.status_code );
	       results_sbuf.push_str(")");
	    }
	    results_sbuf.push_str("</td></tr>");

	    results_sbuf.push_str("</table>");

        return results_sbuf;
    }

    ///
    /// Provide HTML for all of a (Patient's) Encounter's Interventions
    /// 
    pub fn get_intervention_list_for_patient_details_tile(intervention_list: Vec<Intervention>, user_can_view_clinical_data: bool) -> String {
        let mut results_sbuf = String::with_capacity(100); 
        tracing::debug!(">get_intervention_list_tile()");

        results_sbuf.push_str(&CommonFormatter::get_hidden_form("intvDtls".to_owned(), "intvDtlsFrm".to_owned()) );

        results_sbuf.push_str("<table <tr><th>Description</th><th>Date Performed</th><th>Date Scheduled</th><th>State</th></tr>"); 

        for row in intervention_list{
            if row.is_alert() { // add red formatting if the intervention is an alert
                results_sbuf.push_str("  <tr><td class='clinical-emergency-red'>");
            }
            else{
                results_sbuf.push_str("  <tr><td>");
            }
            let clinical_row = row.is_clinical();

            if user_can_view_clinical_data || (! clinical_row ) {
                results_sbuf.push_str("<a href=\"#\" onclick=\"redirect_to_intv("  ); 
                results_sbuf.push_str( &row.id.to_string() ); 
                results_sbuf.push_str("); return false;\">"); 
            }
            results_sbuf.push_str( &row.type_description_for_display()); 
            results_sbuf.push_str("</a></td><td>"); // the orphaned, closing </a> will be ignored by the browser
            results_sbuf.push_str( &row.performed_timestamp_for_display() );
            results_sbuf.push_str("</td>"); 
            results_sbuf.push_str("<td>"); 
            results_sbuf.push_str( &row.scheduled_timestamp_for_display() );
            results_sbuf.push_str("</td>"); 
            results_sbuf.push_str("<td>"); 
            results_sbuf.push_str( &row.status_for_display() );
            results_sbuf.push_str("</td>"); 
            results_sbuf.push_str("  </tr>\n");
        }
        results_sbuf.push_str("</table>");

        return results_sbuf;
    }

}