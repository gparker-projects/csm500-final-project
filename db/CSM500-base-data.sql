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
('Emergency Room A1', 'Main', '','1', 'Area 1', '', 1),
('Emergency Room A2', 'Main', '','1', 'Area 2', '', 1),
('Emergency Room A3', 'Main', '','1', 'Area 3', '', 1),
('Foundation', 'Main', '','1', '125', '', 1),
('Gift Shop', 'Main', '','1', '130', '', 1),
('Home Health Office', 'Main', '','1', '135', '', 1),
('Occupational Therapy', 'Main', '','1', '140', '', 1),
('Operating Room Suite and PACU', 'Main', '','1', '145', '', 1),
('Physiotherapy', 'Main', '','1', '150', '', 1),
('Pre-Surgery Clinic', 'Main', '','1', '155', '', 1),
('Registration', 'Main', '','1', '160', '', 1),
('Rehabilitation/pt gYM', 'Main', '','1', '165', '', 1),
('Surgical Daycare', 'Main', '','1', '170', '', 1),
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