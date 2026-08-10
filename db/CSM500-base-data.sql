-- -------------------------------------------------------------------------------------
-- Loads data required for the basic functioning of the application, using acute facilities
--  from the Fraser Health Authority (and Eagle Ridge Hospital specifically) as a model
--  for the data. All information is publically available (addresses, roomes),
--  fictitous/generalized (role names) or invented (e.g. PERMISSIONs)
--
-- REF: https://www.datacamp.com/doc/postgresql/insert
-- -------------------------------------------------------------------------------------

BEGIN;
INSERT INTO SITE (NAME, ADDRESS, MUNICIPAL_NAME, POSTAL_CODE)
VALUES
('Eagle Ridge Hospital', '475 Guildford Way', 'Port Moody', 'V3H3W9'),
('Surrey Memorial Hospital', '13750 96 Ave', 'Surrey', 'V3V1Z2'),
('Royal Columbian Hospital', '330 E Columbia St.', 'New Westminster', 'V3L3W7');
COMMIT;

BEGIN;
INSERT INTO LOCATION (NAME, BUILDING, WING, FLOOR, ROOM_IDENTIFIER, NOTES, SITE_ID)
VALUES
('Housekeeping', 'Main', '','0', '00-1A', '', 1),
('Health Records', 'Main', '','0', '00-1B', '', 1),
('Medical Device Reprocessing', 'Main', '','0', '00-10', '', 1),
('Facilities Maintenance Operations', 'Main', '','0', '00-20', '', 1),
('Volunteer Services', 'Main', '','0', '00-21', '', 1),
('Administration', 'Main', '','1', '110', '', 1),
('Ambulatory Daycare', 'Main', '','1', '120', '', 1),
('Cardiology', 'Main', '','1', '122', '', 1),
('Emergency Department Area 1', 'Main', '','1', 'Exam Room 1', '', 1),
('Emergency Department Area 1', 'Main', '','1', 'Exam Room 2', '', 1),
('Emergency Department Area 2', 'Main', '','1', 'Exam Room 1', '', 1),
('Emergency Department Area 3', 'Main', '','1', 'Exam Room 1', '', 1),
('Foundation', 'Main', '','1', '125', '', 1),
('Gift Shop', 'Main', '','1', '130', '', 1),
('Home Health Office', 'Main', '','1', '135', '', 1),
('Occupational Therapy', 'Main', '','1', '140', '', 1),
('Operating Room Suite and PACU', 'Main', '','1', '145', '', 1),
('Physiotherapy', 'Main', '','1', '150', '', 1),
('Pre-Surgery Clinic', 'Main', '','1', '155', '', 1),
('Registration', 'Main', '','1', '160', '', 1),
('Rehabilitation/pt gYM', 'Main', '','1', '165', '', 1),
('Surgical Daycare', 'Main', '','1', '', '', 1),
('Surgical Daycare', 'Main', '','1', '170A', '', 1),
('Surgical Daycare', 'Main', '','1', '170B', '', 1),
('Laboratory', 'Main', '','1', '175', '', 1),
('Medical Imaging', 'Main', '','1', '180', '', 1),
('Medicine E2A', 'Main', 'East', '2', 'E2A', '', 1),
('Medicine W2B', 'Main', 'West', '2', 'W2B', '', 1),
('Medicine C2B', 'Main', 'Central', '2', 'C2B', '', 1),
('Medicine E2B', 'Main', 'East', '2', 'E2B', '', 1),
('Surgery and Medicine', 'Main', 'West', '2', 'W2A', '', 1),
('Monitored Care', 'Main', 'Central','2', 'MCU', '', 1),
('Pharmacy', 'Main', 'Central','2', 'Pharmacy', '', 1),
('Path Unit', 'Main', 'Central','2', 'C2A', '', 1);
COMMIT;

BEGIN;
INSERT INTO DEPARTMENT (NAME, EXPIRY_DATETIME)
VALUES
('Admission',        '2050-01-01 23:59:59-00'),
('Ambulatory',       '2050-01-01 23:59:59-00'),
('Cardiology',       '2050-01-01 23:59:59-00'),
('Emergency',        '2050-01-01 23:59:59-00'),
('Housekeeping',     '2050-01-01 23:59:59-00'),
('Laboratory',       '2050-01-01 23:59:59-00'),
('Medical Imaging',  '2050-01-01 23:59:59-00'),
('Medicine',         '2050-01-01 23:59:59-00'),
('Monitored Care',   '2050-01-01 23:59:59-00'),
('Outpatient Rehab', '2050-01-01 23:59:59-00'),
('Pathology',        '2050-01-01 23:59:59-00'),
('Pharmacy',         '2050-01-01 23:59:59-00'),
('Surgical Daycare', '2050-01-01 23:59:59-00'),
('Surgical Unit',    '2050-01-01 23:59:59-00');
COMMIT;

BEGIN;
INSERT INTO ROLE (NAME, EXPIRY_DATETIME)
VALUES
('Physician', '2050-01-01 23:59:59-00'),
('Nurse', '2050-01-01 23:59:59-00'),
('Lab Technician', '2050-01-01 23:59:59-00'),
('Pharmacist', '2050-01-01 23:59:59-00'),
('Housekeeping', '2050-01-01 23:59:59-00'),
('Admin', '2050-01-01 23:59:59-00');
COMMIT;


BEGIN;
INSERT INTO PERMISSION  (NAME, EXPIRY_DATETIME)
VALUES
('perform-admit', '2050-01-01 23:59:59-00'),
('view-admit', '2050-01-01 23:59:59-00'),
('perform-discharge', '2050-01-01 23:59:59-00'),
('request-discharge', '2050-01-01 23:59:59-00'),
('view-discharge', '2050-01-01 23:59:59-00'),
('perform-chart', '2050-01-01 23:59:59-00'),
('view-chart', '2050-01-01 23:59:59-00'),
('perform-treatment', '2050-01-01 23:59:59-00'),
('request-treatment', '2050-01-01 23:59:59-00'),
('view-treatment', '2050-01-01 23:59:59-00'),
('perform-prescription', '2050-01-01 23:59:59-00'),
('request-prescription', '2050-01-01 23:59:59-00'),
('view-prescription', '2050-01-01 23:59:59-00'),
('perform-order', '2050-01-01 23:59:59-00'),
('request-order', '2050-01-01 23:59:59-00'),
('view-order', '2050-01-01 23:59:59-00'),
('perform-move', '2050-01-01 23:59:59-00'),
('request-move', '2050-01-01 23:59:59-00'),
('view-move', '2050-01-01 23:59:59-00'),
('perform-operation', '2050-01-01 23:59:59-00'),
('request-operation', '2050-01-01 23:59:59-00'),
('view-operation', '2050-01-01 23:59:59-00'),
('perform-clean', '2050-01-01 23:59:59-00'),
('request-clean', '2050-01-01 23:59:59-00'),
('view-clean', '2050-01-01 23:59:59-00'),
('perform-restock', '2050-01-01 23:59:59-00'),
('request-restock', '2050-01-01 23:59:59-00'),
('view-restock', '2050-01-01 23:59:59-00'),
('perform-carestart', '2050-01-01 23:59:59-00'),
('request-carestart', '2050-01-01 23:59:59-00'),
('view-carestart', '2050-01-01 23:59:59-00'),
('perform-careend', '2050-01-01 23:59:59-00'),
('request-careend', '2050-01-01 23:59:59-00'),
('view-careend', '2050-01-01 23:59:59-00'),
('find-patient', '2050-01-01 23:59:59-00'),
('find-allpatients', '2050-01-01 23:59:59-00'),
('find-all-dept-patients', '2050-01-01 23:59:59-00');
COMMIT;

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (1, 1, 'Y', 'Bandage', 'Bandage');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (2, 1, 'Y', 'Bloodwork', 'Bloodwork');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (3, 1, 'Y', 'CT Scan', 'CT Scan');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (4, 1, 'Y', 'MRI', 'MRI');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (5, 1, 'Y', 'Medication', 'Medication');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (6, 1, 'Y', 'Port', 'Patient Transfer');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (7, 1, 'Y', 'Referral', 'Referral');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (8, 1, 'Y', 'Surgery', 'Surgery');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (9, 1, 'Y', 'Suture', 'Suture');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (10, 1, 'Y', 'Transfusion', 'Transfusion');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (11, 1, 'Y', 'X-Ray', 'X-Ray');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (12, 1, 'Y', 'Other', 'Other');
COMMIT;


INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (13, 2, 'Y', 'New (Unassigned)', 'New (Unassigned)');
  
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (14, 2, 'Y', 'Pending (Assigned)', 'Pending (Assigned)');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (15, 2, 'Y', 'In Progress', 'In Progress');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (16, 2, 'Y', 'On Hold', 'On Hold');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (17, 2, 'Y', 'Complete', 'Complete');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (18, 2, 'Y', 'Archived', 'Archived');
  
COMMIT;


INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (19, 3, 'Y', 'Admit', 'Admit');
  
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (20, 3, 'Y', 'Triage', 'Triage');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (21, 3, 'Y', 'Discharge', 'Discharge');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (22, 3, 'Y', 'Consult', 'Consult');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (23, 3, 'Y', 'Direct Care', 'Direct Care');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (24, 3, 'Y', 'Examination', 'Examination');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (25, 3, 'Y', 'Surgery', 'Surgery');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (26, 3, 'Y', 'Treatment', 'Treatment');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (27, 3, 'Y', 'Other', 'Other');

COMMIT;

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (28, 4, 'Y', 'Scheduled Time', 'Scheduled Time');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (29, 4, 'Y', 'Performed Time', 'Performed Time');

COMMIT;

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (30, 5, 'Y', 'Height (cm)', 'Height (cm)');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (31, 5, 'Y', 'Weight (kg)', 'Weight (kg)');

COMMIT;