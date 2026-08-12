-- -------------------------------------------------------------------------------------
-- Loads seed patient data for the system.
-- -------------------------------------------------------------------------------------
INSERT INTO patient(
	id, legal_first_name, legal_last_name, legal_middle_names, sin, birthdate, phn)
	OVERRIDING SYSTEM VALUE
	VALUES (1, 'Sara', 'Newman', null, 123456789, '1980-08-04',9123456789);
	
INSERT INTO patient(
	id, legal_first_name, legal_last_name, legal_middle_names, sin, birthdate, phn)
	OVERRIDING SYSTEM VALUE
	VALUES (2, 'Sun', 'Tzu', null, 888444888, '2001-01-01',9098765123);
	
INSERT INTO patient(
	id, legal_first_name, legal_last_name, legal_middle_names, sin, birthdate, phn)
	OVERRIDING SYSTEM VALUE
	VALUES (3, 'Mary', 'Mallon', null, 186909230, '1969-09-23',9001100110);
	
COMMIT;

-- -------------------------------------------------------------------------------------
-- Loads seed ENCOUNTER data for the system.
-- -------------------------------------------------------------------------------------
INSERT INTO encounter(
	id, admit_timestamp, admit_notes, discharge_timestamp, discharge_notes, patient_id, location_id)
	OVERRIDING SYSTEM VALUE
	VALUES (1, '2026-08-04 14:30:00', 'Admitted', null, null, 1, 9);

INSERT INTO encounter(
	id, admit_timestamp, admit_notes, discharge_timestamp, discharge_notes, patient_id, location_id)
	OVERRIDING SYSTEM VALUE
	VALUES (2, '2026-08-04 14:45:00', 'Admitted', null, null, 2, 9);

INSERT INTO encounter(
	id, admit_timestamp, admit_notes, discharge_timestamp, discharge_notes, patient_id, location_id)
	OVERRIDING SYSTEM VALUE
	VALUES (3, '2026-08-04 15:00:00', 'Admitted', null, null, 3, 9);

INSERT INTO encounter(
	id, admit_timestamp, admit_notes, discharge_timestamp, discharge_notes, patient_id, location_id)
	OVERRIDING SYSTEM VALUE
	VALUES (4, '2026-08-02 15:00:00', 'Admitted', null, null, 3, 9);

COMMIT;

-- -------------------------------------------------------------------------------------
-- Loads seed intervention data for the system.
-- -------------------------------------------------------------------------------------
INSERT INTO intervention(
	id, description, notes, location_id, users_id, encounter_id, intervention_type_id, status_id)
	OVERRIDING SYSTEM VALUE
	VALUES (1, 'broken foot from tree climbing', '', 9, 2, 1, 38, 19);
	
INSERT INTO intervention(
	id, description, notes, location_id, users_id, encounter_id, intervention_type_id, status_id)
	OVERRIDING SYSTEM VALUE
	VALUES (2, 'Feeling exhausted', '',  11, 2, 2,  38, 19);
	
INSERT INTO intervention(
	id, description, notes, location_id, users_id, encounter_id, intervention_type_id, status_id)
	OVERRIDING SYSTEM VALUE
	VALUES (3, 'persistent cough', '',  9, 2, 3,  38, 19);
	
INSERT INTO intervention(
	id, description, notes, location_id, users_id, encounter_id, intervention_type_id, status_id)
	OVERRIDING SYSTEM VALUE
	VALUES (4, 'persistent cough', '',  10, 2, 4,  38, 19);

COMMIT;