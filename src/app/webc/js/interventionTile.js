
function validateForm() {
    const errLabel = document.getElementById('errLabel');
    const desc_field = document.getElementById('description');
    errLabel.textContent = '';
    let isValid = true;
    
    console.log("in validateForm();");

    if (desc_field.value.trim() == '') {
        errLabel.textContent = "Description must be provided";
        isValid = false;
    }

    if ( !hasAcceptableChars( desc_field.value ) ) {
        errLabel.textContent = "Description must only contain letters, numbers or basic punctuation.";
        isValid = false;
    }
    
    if (!isValid) {
        errLabel.style = "color: red";
        event.preventDefault();       // Stop the form from submitting if there are errors
    }
    else{
        intvForm.submit();
    }

    // If isValid remains true, the browser automatically proceeds to submit!
    return isValid;
}

function cancelForm(){
    document.getElementById('intvForm').action = "\patientdtls";
    intvForm.submit();
    return;
}

const btnShowHideNewMeasure = document.getElementById('btnShowHideNewMeasure');

btnShowHideNewMeasure.addEventListener('click', () => {
    event.preventDefault();
    divaddNewMeasure.classList.toggle('hidden');
    event.preventDefault();
});

function validateAddMeasureForm() {
    console.log("validateAddMeasureForm()");
    const addFrm_errors = document.getElementById('addFrm_errors');
    const addMeasureForm = document.getElementById('addMeasureForm');
    const value_field = document.getElementById('addFrm_value');

    addFrm_errors.textContent = '';
    let isValid = true;		

    if ( value_field.value.trim() == '' ) {
        addFrm_errors.textContent = "Value is required.";
        isValid = false;
    }

    if ( !hasAcceptableChars( value_field.value ) ) {
        addFrm_errors.textContent = "Value must only contain letters, numbers or basic punctuation.";
        isValid = false;
    }

    if (!isValid) {
        addFrm_errors.style = "color: red";
        addFrm_errors.classList.toggle('hidden');
        event.preventDefault(); // Stop the form from submitting if there are errors
    }
    else{
        addFrm_intv_dtls_id.value = -1; // make sure we are creating a new entry "on add"
        addMeasureForm.submit();
    }
    return isValid; // If isValid remains true, the browser automatically proceeds to submit!
}

function validateItvDtlsForm(itemId) {
    console.log("validateItvDtlsForm("+itemId+")");
    const formErrors = document.getElementById("itemForm_" + itemId + "_errors");
    const intvDtlsForm = document.getElementById("itemForm_" + itemId);
    const field_value = document.getElementById("intv_dtls_value_" + itemId);
    const field_notes = document.getElementById("intv_dtls_notes_" + itemId);

    formErrors.textContent = '';
    let isValid = true;		

    if ( field_value.value.trim() == '' || !hasAcceptableChars( field_value.value )) {
        formErrors.textContent = "Value is required and can only contain letters, numbers and basic punctuation.";
        isValid = false;
    }

    if  (field_value.value.length > 100 || field_value.value.length < 1 || !hasAcceptableChars( field_value.value )) {
        formErrors.textContent = "Value must be between 1 and 100 characters, and can only contain letters, numbers and basic punctuation.";
        isValid = false;
    }
    
    if (field_notes.value.length > 2000 || !hasAcceptableChars( field_notes.value )) {
        formErrors.textContent = "Notes must be less than 2000 characters, and can only contain letters, numbers and basic punctuation.";
        isValid = false;
    }

    if (!isValid) {
        formErrors.style = "color: red";
        event.preventDefault(); // Stop the form from submitting if there are errors
    }
    else{
        // copy values over to edit form, for clean submission to the params the backend server is expecting
        document.getElementById("addFrm_intv_id").value = document.getElementById("intv_id_" + itemId).value;
        document.getElementById("addFrm_intv_dtls_id").value = document.getElementById("intv_dtls_id_" + itemId).value;
        document.getElementById("addFrm_type_id").value = document.getElementById("type_id_" + itemId).value;
        document.getElementById("addFrm_value").value = field_value.value;
        document.getElementById("addFrm_notes").value = field_notes.value;

        addMeasureForm.submit();
    }
    return isValid; // If isValid remains true, the browser automatically proceeds to submit!
}

// used by the fast action for preferences
function fast_action_add_measure(itemId){
    console.log("prep_add_measure");
    const divaddNewMeasure = document.getElementById('divaddNewMeasure');
    const addFrm_intv_dtls_id = document.getElementById('addFrm_intv_dtls_id');

    divaddNewMeasure.classList.remove('hidden');
    addFrm_intv_dtls_id.value = -1;
    addFrm_type_id.value = itemId;
}


function hasAcceptableChars(str) {
 // https://www.tutorialspoint.com/article/how-to-validate-an-input-is-alphanumeric-or-not-using-javascript
 // checks if a string is an alphanumeric or basic punctuation
 //
  const regex = /^[a-zA-Z0-9\s!"#$%^&*()_+=,\-./:;?[\\\]|]+$/;  
  return regex.test(str);
}