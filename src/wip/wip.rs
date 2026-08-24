


admit Keenan Waynes for broken foot and blood loss. age 25, 6ft 10, 125kg

Mary Medical AssistantOne
Mattie Medical AssistantTwo

Peter Porter

Drake Ramoray, neurosurgeon




 let ht7 = &ht6.replace(constants::INTERVENTION_TYPE_DROP_DOWN_CONTROL_TAG, &self.get_location_dropdown( location_list, constants::NOT_SPECIFIED_ID));


<!--tr><td>
	<h2>Baseline Vitals</h2>
	<table>
	    <tr class="data-label">
		<td><div>Temp (C)</div><input type="text" name="temperature" id="temperature" class="data-field-rw" value="<!--{temperature}-->"></div></td>
		<td><div>Blood Pressure</div><input type="text" name="blood_pressure" id="blood_pressure" class="data-field-rw" value="<!--{blood_pressure}-->"></div></td>
		<td><div>Weight (Kg)</div><input type="text" name="weight" id="weight" class="data-field-rw" value="<!--{weight}-->"></div></td>
	    </tr>
	    <tr class="data-label">
		<td colspan=3>
		<div>Additional Observations </div><textarea id="intervention_notes" name="intervention_notes" rows="4" cols="50" class="data-field-rw"><!--{intervention_notes}--></textarea>
		</td>
	    </tr>
	</table>

</td></tr-->


//println!("Retrieved {} patients:", patient_list.len());
       let mut pwrap: Vec<PatientWrapper> = Vec::new();

       for p in patient_list.clone(){
          let cur_enc: Encounter = edao.get_current_encounter(p.id.clone()).await.clone(); //get the current encounter for each patient
          let cur_intv = idao.get_most_recent_vitals(cur_enc.id.clone()).await.expect(constants::DATABASE_ERROR_NOT_FOUND); 

          let cur_idtls: Result< Option< Vec<InterventionDetail> >, std::io::Error> = match cur_intv.clone() {
           Some( intv ) => {
                Ok( Some(
                  idao.get_all_intervention_details(intv.id, constants::NOT_SPECIFIED_ID).await.expect(constants::EMPTY_DATASET).clone().unwrap()
                ) )
            }
            None => {
                println!("No intervention found for encounter id = {}", cur_enc.id);
                Ok( None )
            }
          };

          pwrap.push( PatientWrapper{
                  patient: p.clone(),
                  current_encounter: cur_enc.clone(),
                  most_recent_intervention: cur_intv,
                  intervention_detail: cur_idtls.expect(constants::EMPTY_DATASET)
              }
          );
          //print!(">> DEBUG Added pid={} e={} i={}", tmp_p, tmp_e, tmp_i);
       }