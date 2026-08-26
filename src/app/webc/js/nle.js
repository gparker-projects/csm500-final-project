function performNLAction(action_id){
    console.log("nle.js::performNLAction(): action_id=" + action_id);

    //const data = document.getElementById('intervention_id');
    //data.value = intv_id;

    //const frm = document.getElementById('addIntvForm');
    //frm.action = "/intvlink";

    performNLSubAction(action_id);
}

function performNLSubAction(action_id){
	console.log("nle.js::performNLSubAction(): action_id=" + action_id);

    //nlpActionCmdForm.onSubmit = "\"event.preventDefault(); return performNLSubAction(" + action_id + ")\"";
    //nlpActionCmdForm.submit();
}