

async function redirect_to_patient( p_id ){
    const data = document.getElementById('target_id');
    data.value = p_id;

    const frm = document.getElementById('patientDtlsFrm');
    frm.submit();
}

// submit form to admit patient; assumes no prompt provided
async function admit_patient(){ 
	  console.log('admit_patient()');
    const frm = document.getElementById('admitFrm');

    frm.submit();
}

// submit form to admit patient, using prompt provided by the NLE prompt box
async function admit_patient_with_prompt( prompt ){
	  console.log('admit_patient_with_prompt()');
    const frm = document.getElementById('admitFrm');

    // copy from "prompt" of the NLE form, to the admit form's "user_prompt"
    const user_prompt = document.getElementById('prompt');
    user_prompt.value = prompt;
    frm.submit();
}


async function logout(){
    const data = document.getElementById('target_id');
    data.value = enc_id;

    const frm = document.getElementById('encHistoryFrm');
    frm.submit();
}

