// based on the natural language action directed, execute it
//
function performNLAction( action_id, patient_id ){
    console.log("nle.js::performNLAction(): action_id=" + action_id + ", patient_id=" + patient_id);

    switch (action_id) {
        case 1: // Admit New Patient
            admit_patient();
            break;
        case 2: // Add an intervention
            if( patient_id != -1 ){
                redirect_to_patient( patient_id ); // needs a patient_id
                //redirect_to_intv( 13 );
            }
            else {
                const cmd_frm = document.getElementById('nlpCommandForm');
                cmd_frm.action="/home";
                cmd_frm.submit();
            }
            break;
        case 3: // discharge patient
            quickDischarge( patient_id );
            break;
        default:
            // home screen or do nothing
    }
    return true;
}

function quickDischarge( actual_patient_id ){   // highjack the nlCommand form; we're about to refresh the screen anyway
    console.log("nle.js::quickDischarge()");
    const cmd_frm = document.getElementById('nlpCommandForm');
    const patient_id = document.createElement('input');
    const action_flag = document.createElement('input');    

    action_flag.id = "action_flag";
    action_flag.name = "action_flag";
    action_flag.type = "hidden";
    action_flag.value = "discharge";
    cmd_frm.appendChild(action_flag);
    console.log("nle.js::quickDischarge(): action_id=" + action_flag.value);

    patient_id.id = "patient_id";
    patient_id.name = "patient_id";
    patient_id.type = "hidden";
    patient_id.value = actual_patient_id; //"-1";
    cmd_frm.appendChild(patient_id);
    console.log("nle.js::quickDischarge(): patient_id=" + patient_id.value);

    cmd_frm.action="/discharge";
    cmd_frm.submit();
}