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

    return isValid; // If isValid remains true, the browser automatically proceeds to submit!
}

// refs: https://stackoverflow.com/questions/46640024/how-do-i-post-form-data-with-fetch-api
//       https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch
//       JS from Beginner to Prof, pg 417
//
function getData(userPrompt){
    const url = "/nlprompt";
    var results = null;
    
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
    const nl_prompt = document.getElementById('prompt');
    switch ( action_id ) {
        case 1: // Admit New Patient
            //alert ("case 1 + prompt");
            admit_patient_with_prompt( nl_prompt.value );
            break;
        case 2: // Add an intervention
            if( patient_id != -1 ){
                if (nl_prompt) {
                    //alert ("case 2 + actual patient id and an NL prompt present");
                    redirect_to_patient( patient_id, nl_prompt ); // needs a patient_id
                }
                else{
                    //alert ("case 2 + actual patient id");
                    redirect_to_patient( patient_id, "" ); // needs a patient_id
                }
            }
            else {
                //alert ("path 2.3");
                const cmd_frm = document.getElementById('nlpActionCmdForm'); //nlpCommandForm');
                cmd_frm.action = "/home";
                cmd_frm.submit();
            }
            break;
        case 3: // discharge patient
            //alert ("case 3 + prompt");
            quickDischarge( patient_id, nl_prompt.value );
            break;
        default: // all other action_ids are actually intervention_type_id numbers
            //alert("performNLAction(): action_id=" + action_id + " patient_id=" + patient_id);
            if( patient_id != -1 ){
                if (nl_prompt) {
                    //alert ("default + actual patient id and an NL prompt present");
                    redirect_to_patient_new_intv( patient_id, action_id, nl_prompt.value ); // needs a patient_id
                }
                else{
                    //alert ("default + actual patient id");
                    redirect_to_patient_new_intv( patient_id, action_id, "" ); // needs a patient_id
                }
            }
            else {
                //alert ("default + patient id = -1");
                const cmd_frm = document.getElementById('nlpActionCmdForm'); //'nlpCommandForm');
                cmd_frm.action = "/home";
                cmd_frm.submit();
            }
    }
    return true;
}

async function redirect_to_patient_new_intv( patient_id, intervention_type_id, nl_prompt ){
    console.log("redirect_to_patient_new_intv().v_Sep_06_1628:");
    //alert("redirect_to_patient_new_intv v_Sep_06_1022() ..patient_id=" + patient_id + " \n..intv_type_id=" + action_id +" \n..nl_prompt=" + nl_prompt);

    let cmd_frm = document.getElementById('nlpActionCmdForm'); //'nlpCommandForm');
    cmd_frm.action = "/intvnew";

    // We often end up with duplicates of the fields needed to submit from the NLE prompt box, so the best thing to do is to remove them
    // this could cause problems if the user uses the browser back button... too bad. For web applications, back buttons are often disabled
    // or at a minimum, there is no guarantee they will work.
    //
    // ref: https://stackoverflow.com/questions/13125817/how-to-remove-elements-that-were-fetched-using-queryselectorall
    document.querySelectorAll('[name="encounter_id"]').forEach(item => item.remove());
    document.querySelectorAll('[name="intervention_type_id"]').forEach(item => item.remove());
    document.querySelectorAll('[name="patient_id"]').forEach(item => item.remove());
    
    let enc_id = document.createElement('input');
    enc_id.type = "hidden";
    enc_id.name = enc_id.id = "encounter_id"; // we won't have this normally, but it is still required by the route //= enc_id.id
    enc_id.value = -1;    
    cmd_frm.appendChild(enc_id);
    
    let tmp_intv_type_id = document.createElement('input');
    tmp_intv_type_id.type = "hidden";
    tmp_intv_type_id.name = tmp_intv_type_id.id = "intervention_type_id"; //
    cmd_frm.appendChild(tmp_intv_type_id);
    tmp_intv_type_id.value = intervention_type_id;

    let tmp_patient_id = document.createElement('input');
    tmp_patient_id.type = "hidden";
    tmp_patient_id.name  = "patient_id"; //= tmp_patient_id.id
    cmd_frm.appendChild(tmp_patient_id);
    tmp_patient_id.value = patient_id;

    let tmp_user_prompt = document.getElementById('user_prompt'); // see comment above
    if (! tmp_user_prompt ){
        tmp_user_prompt = document.createElement('input');
        tmp_user_prompt.type = "hidden";
        tmp_user_prompt.name = "user_prompt"; //= tmp_user_prompt.id 
        cmd_frm.appendChild(tmp_user_prompt);
    }
    tmp_user_prompt.value = nl_prompt;

    //alert("redirect_to_patient_new_intv().v_Sep_06_1628:\n..submitting nlpCommandForm to "+ cmd_frm.action + " " + enc_id.id + "=" +enc_id.value+ " " + tmp_intv_type_id.id + " =" +tmp_intv_type_id.value);
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