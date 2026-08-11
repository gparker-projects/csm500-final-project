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

COMMIT;