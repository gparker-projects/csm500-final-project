// based on the natural language action directed, execute it
//
function performNLAction(action_id){
    console.log("nle.js::performNLAction(): action_id=" + action_id);

    switch (action_id) {
    case 1: // Admit New Patient
        admit_patient();
        break;
    case 2: // Add an intervention
        redirect_to_patient(-1); // needs a patient_id
        redirect_to_intv(13);
        break;
    case 3: // discharge patient
        submitDischargeForm();
        break;
    default:
        // home screen or do nothing
    }
    //const data = document.getElementById('intervention_id');
    //data.value = intv_id;

    //const frm = document.getElementById('addIntvForm');
    //frm.action = "/intvlink";

    //performNLSubAction(action_id);
    return true;
}

function performNLSubAction(action_id){
	console.log("nle.js::performNLSubAction(): action_id=" + action_id);

    //nlpActionCmdForm.onSubmit = "\"event.preventDefault(); return performNLSubAction(" + action_id + ")\"";
    //nlpActionCmdForm.submit();
}