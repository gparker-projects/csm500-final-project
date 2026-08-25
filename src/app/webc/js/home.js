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
  userPrompt = document.getElementById('prompt').value.trim();
  let isValid = true;

  if (userPrompt == '') {
	errLabel.textContent = "Please enter a prompt.";
	errLabel.style = "color: red";
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

	//document.getElementById("mainContentArea").innerHTML = (newBody);

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

async function redirect_to_patient(p_id){
    const data = document.getElementById('target_id');
    data.value = p_id;

    const frm = document.getElementById('patientDtlsFrm');
    frm.submit();
}

async function admit_patient(){
	console.log('admit_patient');

    const frm = document.getElementById('admitFrm');
    frm.submit();
}

/*async function redirect_to_enc(enc_id){
    const data = document.getElementById('target_id');
    data.value = enc_id;

    const frm = document.getElementById('encHistoryFrm');
    frm.submit();
}*/

