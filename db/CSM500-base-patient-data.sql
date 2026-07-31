-- -------------------------------------------------------------------------------------
-- Loads seed patient data for the system.
-- -------------------------------------------------------------------------------------

INSERT INTO patient(
       legal_first_name,
	   legal_last_name,
	   legal_middle_names,
	   sin,
	   birthdate,
	   location_id)
VALUES ('Sun',
	   '',
	   'Tzu',
	   888444888,
	   '2000-01-01 06:00:01-00',
	   9);


INSERT INTO INTERVENTION (
  INTERVENTION_CODE,
  DESCRIPTION,
  NOTES,  
  LOCATION_ID,
  USERS_ID 
VALUES (
  '',
  '',
  '',
  '',
  1
);

INSERT INTO ENCOUNTER (
  ADMIT_NOTES,
  DISCHARGE_TIMESTAMP, 
  DISCHARGE_NOTES,  
  PATIENT_ID,
  INTERVENTION_ID,
  1)
VALUES (
  '',
  '',
  '',
  '',
  '',
  1
);

