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
INSERT INTO LOCATION (NAME, BUILDING, WING, FLOOR, ROOM_IDENTIFIER, SHORT_NAME, NOTES, SITE_ID)
VALUES
('Housekeeping', 'Main', '','0', '00-1A','SHORT_NAME', 'ERH MHS R00-1A', 1),
('Health Records', 'Main', '','0', '00-1B', 'SHORT_NAME', 'ERH HLTHR R00-1B', 1),
('Medical Device Reprocessing', 'Main', '','0', '00-10', 'ERH MDR R00-10', '', 1),
('Facilities Maintenance Operations', 'Main', '','0', '00-20', 'ERH FMO R00-20', '', 1),
('Volunteer Services', 'Main', '','0', '00-21', 'ERH VOLS R00-21', '', 1),
('Administration', 'Main', '','1', '110', 'ERH ADM R110', '', 1),
('Ambulatory Daycare', 'Main', '','1', '120', 'ERH AMBD R120', '', 1),
('Cardiology', 'Main', '','1', '122', 'SHORT_NAME', 'ERH CARD R122', 1),
('Emergency Department Area 1', 'Main', '','1', 'Exam Room 1', 'ERH ED-A1-RM1', '', 1),
('Emergency Department Area 1', 'Main', '','1', 'Exam Room 2', 'ERH ED-A1-RM2', '', 1),
('Emergency Department Area 2', 'Main', '','1', 'Exam Room 1', 'ERH ED-A2-RM1', '', 1),
('Emergency Department Area 3', 'Main', '','1', 'Exam Room 1', 'ERH ED-A3-RM1', '', 1),
('Foundation', 'Main', '','1', '125', 'ERH FND-01 R125', '', 1),
('Gift Shop', 'Main', '','1', '130', 'ERH GFT R130', '', 1),
('Home Health Office', 'Main', '','1', '135', 'ERH HHS R135', '', 1),
('Occupational Therapy', 'Main', '','1', '140', 'ERH OCCTH R140', '', 1),
('Operating Room Suite and PACU', 'Main', '','1', '145', 'ERH OR-01 R145', '', 1),
('Physiotherapy', 'Main', '','1', '150', 'ERH PHYS R150', '', 1),
('Pre-Surgery Clinic', 'Main', '','1', '155', 'ERH PSURG R155', '', 1),
('Registration', 'Main', '','1', '160', 'ERH REG R160', '', 1),
('Rehabilitation/pt gYM', 'Main', '','1', '165', 'ERH REHAB R165', '', 1),
('Surgical Daycare', 'Main', '','1', '', 'ERH SRGD-01', '', 1),
('Surgical Daycare', 'Main', '','1', '170A', 'ERH SGD R170A', '', 1),
('Surgical Daycare', 'Main', '','1', '170B', 'ERH SGD R170B', '', 1),
('Laboratory', 'Main', '','1', '175', 'ERH LAB-01 R175', '', 1),
('Medical Imaging', 'Main', '','1', '180', 'ERH MI-01 R180', '', 1),
('Medicine E2A', 'Main', 'East', '2', 'E2A', 'ERH MED E2A', '', 1),
('Medicine W2B', 'Main', 'West', '2', 'W2B', 'ERH MED W2B', '', 1),
('Medicine C2B', 'Main', 'Central', '2', 'C2B', 'ERH MED C2B', '', 1),
('Medicine E2B', 'Main', 'East', '2', 'E2B', 'ERH MED E2B', '', 1),
('Surgery and Medicine', 'Main', 'West', '2', 'W2A', 'ERH SURG W2A', '', 1),
('Monitored Care', 'Main', 'Central','2', 'MCU', 'ERH MCU C2', '', 1),
('Pharmacy', 'Main', 'Central','2', 'Pharmacy', 'ERH PHM C2', '', 1),
('Path Unit', 'Main', 'Central','2', 'C2A', 'ERH PATU C2A', '', 1);
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
('Medical Office Assistant (MOA)', '2050-01-01 23:59:59-00'),
('Nurse', '2050-01-01 23:59:59-00'),
('Physician', '2050-01-01 23:59:59-00'),
('Lab Technician', '2050-01-01 23:59:59-00'),
('Porter', '2050-01-01 23:59:59-00'),
('Admin', '2050-01-01 23:59:59-00');
COMMIT;

BEGIN;
INSERT INTO PERMISSION (NAME, EXPIRY_DATETIME)
VALUES
('login', '2050-01-01 23:59:59-00'),
('create-clinical-intervention', '2050-01-01 23:59:59-00'),
('create-non-clinical-intervention', '2050-01-01 23:59:59-00'),
('create-update-admit', '2050-01-01 23:59:59-00'),
('create-update-discharge', '2050-01-01 23:59:59-00'),
('update-clinical-intervention', '2050-01-01 23:59:59-00'),
('update-non-clinical-intervention', '2050-01-01 23:59:59-00'),
('view-admit', '2050-01-01 23:59:59-00'),
('view-any-clinical-data', '2050-01-01 23:59:59-00'),
('view-clinical-intervention', '2050-01-01 23:59:59-00'),
('view-discharge', '2050-01-01 23:59:59-00'),
('view-non-clinical-intervention', '2050-01-01 23:59:59-00');

COMMIT;

-- group 1: Intervention Types
--
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100001, 1, 'Y', 'Procedure: Bandage', 'Procedure: Bandage');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100002, 1, 'Y', 'Collect Specimen: Bloodwork', 'Collect Specimen: Bloodwork');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100003, 1, 'Y', 'Procedure: CT Scan', 'Procedure: CT Scan');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100004, 1, 'Y', 'Procedure: Magnetic Resonance Imaging (MRI)', 'Procedure: Magnetic Resonance Imaging (MRI)');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100005, 1, 'Y', 'Procedure: Administer Medication', 'Procedure: Administer Medication');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100006, 1, 'Y', 'Support Request: Patient Transfer', 'Support Request: Patient Transfer');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100007, 1, 'Y', 'Support Request: Physician Referral', 'Support Request: Physician Referral');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100008, 1, 'Y', 'Procedure: (Cardiovascular) Open Heart Surgery', 'Procedure: (Cardiovascular) Open Heart Surgery');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100009, 1, 'Y', 'Procedure: General Suture', 'Procedure: General Suture');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100010, 1, 'Y', 'Procedure: Blood Transfusion', 'Procedure: Blood Transfusion');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100011, 1, 'Y', 'Procedure: X-Ray', 'Procedure: X-Ray');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100012, 3, 'Y', 'Other', 'Other');

COMMIT;

-- group 2: Intervention Status
--
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

-- group 3: Care Types
--
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

-- group 4: Intervention metadata
--
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (28, 4, 'Y', 'Scheduled Time', 'Scheduled Time');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (29, 4, 'Y', 'Performed Time', 'Performed Time');

COMMIT;

-- group 5: standard measurements for data fields; https://canadiem.org/how-to-read-patient-monitors/
--
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (30, 5, 'Y', 'Height (cm)', 'Height (cm)');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (31, 5, 'Y', 'Weight (kg)', 'Weight (kg)');
  
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (32, 5, 'Y', 'HR', 'Heart Rate (HR)');
  
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (33, 5, 'Y', 'BP', 'Blood Pressure (BP)');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (34, 5, 'Y', 'RR', 'Respiratory Rate');
  
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (35, 5, 'Y', 'SpO2', 'Oxygen Saturation');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (36, 5, 'Y', 'Temp (C)', 'Body Temperature');
  
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (37, 5, 'Y', 'PL', 'Pain Level');

COMMIT;

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100038, 1, 'Y', 'Procedure: Collect Vitals', 'Procedure: Collect Vitals');
  
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
    OVERRIDING SYSTEM VALUE
 VALUES (100039, 5, 'Y', 'Eye', 'Eye Colour');
  
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
    OVERRIDING SYSTEM VALUE
 VALUES (100040, 1, 'Y', 'Alerts/CCI/SPI', 'Alerts/CCI/SPI');
 
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
    OVERRIDING SYSTEM VALUE
 VALUES (100041, 3, 'Y', 'Appointments', 'Appointments');
 
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
     OVERRIDING SYSTEM VALUE
  VALUES (100042, 1, 'Y', 'Diagnoses', 'Diagnoses');
  
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
     OVERRIDING SYSTEM VALUE
 VALUES (100043, 1, 'Y', 'Allergies', 'Allergies');
  
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
     OVERRIDING SYSTEM VALUE
 VALUES (100044, 1, 'Y', 'Immunizations', 'Immunizations');
 
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
     OVERRIDING SYSTEM VALUE
 VALUES (100045, 3, 'Y', 'Documents', 'Documents');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
      OVERRIDING SYSTEM VALUE
 VALUES (100047, 1, 'Y', 'Mental Health and Wellness', 'Mental Health and Wellness');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
      OVERRIDING SYSTEM VALUE
 VALUES (100048, 1, 'Y', 'Physiotherapy', 'Physiotherapy');
COMMIT;

-- quick addition of new permissions that directly align to interventions being requested for creation
-- via the NLE
--

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100049, 3, 'Y', 'Next of Kin/Contacts', 'Next of Kin/Contacts');

INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100050, 3, 'Y', 'Primary Address', 'Primary Address');
  
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100051, 1, 'Y', 'Clinical Discharge', 'Clinical Discharge');
  
INSERT INTO common_reference_type(id, group_id, active_flag, name, description)
  OVERRIDING SYSTEM VALUE
  VALUES (100052, 3, 'Y', 'Medical Insurance', 'Medical Insurance');

INSERT INTO PERMISSION (ID, NAME, EXPIRY_DATETIME)
OVERRIDING SYSTEM VALUE
SELECT ID, NAME, (TIMESTAMP '2100-12-31')
FROM COMMON_REFERENCE_TYPE
WHERE GROUP_ID in (1,3);

COMMIT;
