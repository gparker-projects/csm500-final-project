// from Javascript from Beginner to Professional, Svekis L, Putten M, Percival R; Chapter 10
//
//Aynsc calls:
//   https://www.geeksforgeeks.org/javascript/async-await-function-in-javascript/
//   https://www.w3schools.com/js/js_async_await.asp
//   from Javascript from Beginner to Professional, Svekis L, Putten M, Percival R; Ch13
//REFs: https://www.w3schools.com/js/js_validation.asp
//             https://stackoverflow.com/questions/42803866/form-is-submitting-even-after-validation-function-returning-false
//
// replace with Request/Promise from:
// https://www.digitalocean.com/community/tutorials/how-to-use-the-javascript-fetch-api-to-get-data#fetch-api-vs-ajax-vs-axios
//

async function validateNLPrompt() {
  const errLabel = document.getElementById('errLabel');
  errLabel.textContent = '';

  // pre-pend the context of the prompt before submission. This can be needed when a prompt does not specify a patient, but is
  // requesting a patient-level action such as a transfer.maybe TODO Remove this?
  const patient_id_ctrl = document.getElementById('patient_id');
  let patient_id = -1;
  if (patient_id_ctrl) {
      patient_id = patient_id_ctrl.value;
  }

  let userPrompt = "{patient_id=" +patient_id + "}" + document.getElementById('prompt').value.trim();
  let isValid = true;

  if (userPrompt == '') {
    errLabel.textContent = "Please enter a prompt.";
    errLabel.style = "clinical-emergency-red"; //"color: red";
    isValid = false;
  }

  if (!isValid) {
	  event.preventDefault();       // Stop the form from submitting if there are errors
  }
  else{
	  console.log("Submitting prompt: " + userPrompt);

    var newBody = null;
    try {
      var newBody = await getData(userPrompt);
      console.log("New body: " + newBody);
    }
    catch (error){
      console.log("Error occurred: " + error);
      newBody = "Error occurred: " + error;
    }

    document.getElementById("MapleEMR::NLPCanvas").innerHTML = (newBody);
  }

  // If isValid remains true, the browser automatically proceeds to submit!
  return isValid;
}

function getData(userPrompt){
  const url = "/nlprompt";
  var results = null;

  //console.log("In getData(): " + userPrompt);

  // refs: https://stackoverflow.com/questions/46640024/how-do-i-post-form-data-with-fetch-api
  //       https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch
  //       JS from Beginner to Prof, pg 417
  //
  results = fetch(url, {
  method: "POST",
  headers: {
	   'Content-Type': 'application/x-www-form-urlencoded',
	   'Accept': 'application/json'
  },
	body: new URLSearchParams({ prompt: userPrompt })
  })
  .then((response) => {
	  if (!response.ok){
      console.log("Response error:" + response.status);
      throw new Error(response.status);
	  }
	  return response.text();
   })
   .then((data) => {
      results = data;
      console.log("Response data: " + data);
      return data;
   })
  .catch((error) => console.error(error));

  return results;
}

// based on the natural language action directed, execute it
//
function performNLAction( action_id, patient_id ){
    //console.log("nle.js::performNLAction(): action_id=" + action_id + ", patient_id=" + patient_id);

    const nl_prompt = document.getElementById('prompt');
    //console.log("..v4 prompt=" + nl_prompt.value);

    switch ( action_id ) {
        case 1: // Admit New Patient
            admit_patient_with_prompt( nl_prompt.value );
            break;
        case 2: // Add an intervention
            
            if( patient_id != -1 ){
                if (nl_prompt) {
                    alert ("path 2.1");
                    redirect_to_patient( patient_id, nl_prompt ); // needs a patient_id
                }
                else{
                     alert ("path 2.2");
                    redirect_to_patient( patient_id, "" ); // needs a patient_id
                }
            }
            else {
                alert ("path 2.3");
                const cmd_frm = document.getElementById('nlpCommandForm');
                cmd_frm.action = "/home";
                cmd_frm.submit();
            }
            break;
        case 3: // discharge patient
            quickDischarge( patient_id, nl_prompt.value );
            break;
        default: // all other action_ids are actually intervention_type_id numbers
            //alert("performNLAction(): action_id=" + action_id + " patient_id=" + patient_id);
            if( patient_id != -1 ){
                if (nl_prompt) {
                     alert ("default.1");
                    redirect_to_patient_new_intv( patient_id, action_id, nl_prompt.value ); // needs a patient_id
                }
                else{
                    alert ("default.2");
                    redirect_to_patient_new_intv( patient_id, action_id, "" ); // needs a patient_id
                }
            }
            else {
                alert ("default.3");
                const cmd_frm = document.getElementById('nlpCommandForm');
                cmd_frm.action = "/home";
                cmd_frm.submit();
            }
    }
    return true;
}

async function redirect_to_patient_new_intv(  patient_id, action_id, nl_prompt ){
    console.log("redirect_to_patient_new_intv v_Sep_06_1022()");
    //alert("redirect_to_patient_new_intv v_Sep_06_1022() ..patient_id=" + patient_id + " \n..intv_type_id=" + action_id +" \n..nl_prompt=" + nl_prompt);

    const cmd_frm = document.getElementById('nlpCommandForm');
    cmd_frm.action = "/intvnew";

    let enc_id = document.getElementById('encounter_id');
    if (! enc_id ){ // add the element only if it does not exist already, otherwise we end up with duplicates that will break form submission
        enc_id = document.createElement('input');
        enc_id.type = "hidden";
        enc_id.id = "encounter_id"; // we won't have this normally, but it is still required by the route
        enc_id.name = "encounter_id"; 
        enc_id.value = -1;    
        cmd_frm.appendChild(enc_id);
    }

    let tmp_intv_type_id = document.getElementById('intervention_type_id'); // see comment above
    if (! tmp_intv_type_id ){
        tmp_intv_type_id = document.createElement('input');
        tmp_intv_type_id.type = "hidden";
        tmp_intv_type_id.id = "intervention_type_id";
        tmp_intv_type_id.name = "intervention_type_id";
        tmp_intv_type_id.value = action_id;    
        cmd_frm.appendChild(tmp_intv_type_id);
    }
    tmp_intv_type_id.value = action_id;

    let tmp_patient_id = document.getElementById('patient_id'); // see comment above
    if (! tmp_patient_id ){
        tmp_patient_id = document.createElement('input');
        tmp_patient_id.type = "hidden";
        tmp_patient_id.id = "patient_id";
        tmp_patient_id.name = "patient_id";
        cmd_frm.appendChild(tmp_patient_id);
    }
    tmp_patient_id.value = patient_id;

    let tmp_user_prompt = document.getElementById('user_prompt'); // see comment above
    if (! tmp_user_prompt ){
        tmp_user_prompt = document.createElement('input');
        tmp_user_prompt.type = "hidden";
        tmp_user_prompt.id = "user_prompt";
        tmp_user_prompt.name = "user_prompt";
        cmd_frm.appendChild(tmp_user_prompt);
    }
    tmp_user_prompt.value = nl_prompt;

    alert("..submitting nlpCommandForm to "+ cmd_frm.action + " enc_id=" +enc_id.value+ " tmp_intv_type_id=" +tmp_intv_type_id.value);
    //console.log("..submitting nlpCommandForm to "+ cmd_frm.action + " enc_id=" +enc_id.value);

    cmd_frm.submit();
}

function quickDischarge( actual_patient_id, prompt ){   // highjack the nlCommand form; we're about to refresh the screen anyway
    console.log("nle.js::quickDischarge()");
    const cmd_frm = document.getElementById('nlpCommandForm');
    const patient_id = document.createElement('input');
    const action_flag = document.createElement('input');    
    const user_prompt = document.createElement('input'); 

    user_prompt.id = "user_prompt";
    user_prompt.name = "user_prompt";
    user_prompt.type = "hidden";
    user_prompt.value = prompt;
    cmd_frm.appendChild(user_prompt);
    console.log(".. user_prompt=" + user_prompt.value);

    action_flag.id = "action_flag";
    action_flag.name = "action_flag";
    action_flag.type = "hidden";
    action_flag.value = "discharge";
    cmd_frm.appendChild(action_flag);
    console.log(".. action_id=" + action_flag.value);

    patient_id.id = "patient_id";
    patient_id.name = "patient_id";
    patient_id.type = "hidden";
    patient_id.value = actual_patient_id; //"-1";
    cmd_frm.appendChild(patient_id);
    console.log(".. patient_id=" + patient_id.value);

    cmd_frm.action="/discharge";
    cmd_frm.submit();
}