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
