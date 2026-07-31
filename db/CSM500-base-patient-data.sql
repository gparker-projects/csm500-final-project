-- -------------------------------------------------------------------------------------
-- Loads seed patient data for the system.
-- -------------------------------------------------------------------------------------

INSERT INTO PATIENT (
  LEGAL_FIRST_NAME, 
  LEGAL_LAST_NAME, 
  LEGAL_MIDDLE_NAMES, 
  SIN, 
  BIRTHDATE, 
  LOCATION_ID)
VALUES (
'',
'',
'',
'',
'',
1
);


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

