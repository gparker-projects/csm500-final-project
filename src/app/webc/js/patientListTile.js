function submitDischargeForm() {
    if ( window.confirm("Are you sure you want to discharge the patient?") ) {
        dischargeForm.submit();
        return true;
    }
    return false;
}

function submitAddIntvForm() {
    addIntvForm.submit();
    return true;
}

function redirect_to_intv(intv_id){
    const data = document.getElementById('intervention_id');
    data.value = intv_id;

    const frm = document.getElementById('addIntvForm');
    frm.action = "/intvlink";
    frm.onSubmit = "";
    frm.submit();
}

function fast_action_add_intv(type_id){
    const frm_type = document.getElementById('intervention_type_id');
    const frm = document.getElementById('addIntvForm');

    frm_type.value = type_id;
    frm.submit();
}