-- Procedure: Bandage

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100001, 'Wound size (length)', 'Wound size (length)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100001, 'Wound size (width)', 'Wound size (width)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100001, 'Wound depth', 'Wound depth', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100001, 'Wound location', 'Wound location', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100001, 'Skin temp at site (C)', 'Skin temperature at site (C)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100001, 'Capillary refill time (s)', 'Capillary refill time (s)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100001, 'Distal pulse', 'Distal pulse', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100001, 'Swelling/edema', 'Swelling/edema', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100001, 'Skin colour at site', 'Skin colour at site', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100001, 'Pain level (1-10)', 'Pain level (1-10)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100001, 'Drainage amount', 'Drainage amount', 'Y');
COMMIT;

-- Collect Specimen: Bloodwork

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100002, 'Hemoglobin', 'Hemoglobin', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100002, 'White blood cell count', 'White blood cell count', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100002, 'Platelet count', 'Platelet count', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100002, 'Glucose level', 'Glucose level', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100002, 'Creatinine', 'Creatinine', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100002, 'Sodium', 'Sodium', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100002, 'Potassium', 'Potassium', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100002, 'Hematocrit', 'Hematocrit', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100002, 'Blood pH', 'Blood pH', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100002, 'Cholesterol level', 'Cholesterol level', 'Y');
COMMIT;

-- Procedure: CT Scan

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Weight (kg)', 'Weight (kg)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Height (cm)', 'Height (cm)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Heart rate', 'Heart rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Blood pressure', 'Blood pressure', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Respiratory rate', 'Respiratory rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Oxygen saturation', 'Oxygen saturation', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Creatinine', 'Creatinine (renal function)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'eGFR', 'eGFR', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Contrast volume', 'Contrast volume', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Radiation dose', 'Radiation dose', 'Y');
COMMIT;

-- Procedure: Magnetic Resonance Imaging (MRI)

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100004, 'Weight (kg)', 'Weight (kg)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100004, 'Height (cm)', 'Height (cm)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100004, 'Heart rate', 'Heart rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100004, 'Blood pressure', 'Blood pressure', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100004, 'Respiratory rate', 'Respiratory rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100004, 'Oxygen saturation', 'Oxygen saturation', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100004, 'Creatinine', 'Creatinine (renal function)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100004, 'eGFR', 'eGFR', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100004, 'Contrast volume', 'Contrast volume', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100004, 'Body Temperature (C)', 'Body Temperature (C)', 'Y');
COMMIT;

-- Procedure: Administer Medication

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100005, 'Weight (kg)', 'Weight (kg)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100005, 'Height (cm)', 'Height (cm)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100005, 'Heart rate', 'Heart rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100005, 'Blood pressure', 'Blood pressure', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100005, 'Respiratory rate', 'Respiratory rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100005, 'Body Temperature (C)', 'Body Temperature (C)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100005, 'Oxygen saturation', 'Oxygen saturation', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100005, 'Pain level (1-10)', 'Pain level (1-10)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100005, 'Blood Glucose', 'Blood Glucose', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100005, 'Body surface area (cm^2)', 'Body surface area (cm^2)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100005, 'Medication Name/Manufacturer', 'Vaccine Name/Manufacturer', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100005, 'Prior Administration Date/Time', 'Prior Administration Date/Time', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100005, 'Dosage (ml/gm)', 'Dosage (ml/gm)', 'Y');

COMMIT;

-- Support Request: Patient Transfer
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100006, 'Heart rate', 'Heart rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100006, 'Blood pressure', 'Blood pressure', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100006, 'Consciousness level', 'Consciousness level', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100006, 'GCS: [E]ye Opening (1-4)', 'Glasgow Coma Score (GCS): [E]ye Opening (1-4)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100006, 'GCS: [V]erbal Response (1-5)', 'Glasgow Coma Score (GCS): [V]erbal Response (1-5)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100006, 'GCS: [M]otor Response (1-6)', 'Glasgow Coma Score (GCS): [M]otor Response (1-6)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100006, 'Respiratory rate', 'Respiratory rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100006, 'Body Temperature (C)', 'Body Temperature (C)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100006, 'Oxygen saturation', 'Oxygen saturation', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100006, 'Weight (kg)', 'Weight (kg)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100006, 'Mobility level', 'Mobility level', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100006, 'Pain level (1-10)', 'Pain level (1-10)', 'Y');

COMMIT;

-- Support Request: Physician Referral

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100007, 'Primary symptom severity (1-10)', 'Primary symptom severity (1-10)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100007, 'Weight (kg)', 'Weight (kg)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100007, 'Height (cm)', 'Height (cm)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100007, 'BMI', 'Body Mass Index', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100007, 'Blood pressure', 'Blood pressure', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100007, 'Heart rate', 'Heart rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100007, 'Body Temperature (C)', 'Body Temperature (C)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100007, 'Respiratory rate', 'Respiratory rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100007, 'Oxygen saturation', 'Oxygen saturation', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100007, 'Pain level (1-10)', 'Pain level (1-10)', 'Y');

COMMIT;

-- Procedure: (Cardiovascular) Open Heart Surgery

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100008, 'Weight (kg)', 'Weight (kg)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100008, 'Height (cm)', 'Height (cm)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100008, 'Blood pressure', 'Blood pressure', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100008, 'Heart rate', 'Heart rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100008, 'Ejection fraction', 'Ejection fraction', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100008, 'Oxygen saturation', 'Oxygen saturation', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100008, 'Body Temperature (C)', 'Body Temperature (C)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100008, 'Central venous pressure', 'Central venous pressure', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100008, 'Blood type', 'Blood type', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100008, 'Hemoglobin', 'Hemoglobin', 'Y');

COMMIT;

-- Procedure: General Suture (100009)

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100009, 'Wound size (length)', 'Wound size (length)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100009, 'Wound size (width)', 'Wound size (width)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100009, 'Wound depth', 'Wound depth', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100009, 'Wound location', 'Wound location', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100009, 'Pain level (1-10)', 'Pain level (1-10)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100009, 'Distal pulse', 'Distal pulse', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100009, 'Bleeding amount', 'Bleeding amount', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100009, 'Skin colour at site', 'Skin colour at site', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100009, 'Capillary refill time (min)', 'Capillary refill time (min)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100009, 'Sensation at site (1-10)', 'Sensation at site (1-10)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100009, 'Range of motion', 'Range of motion', 'Y');

COMMIT;

-- Procedure: Blood Transfusion (100010)

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100010, 'Rh factor', 'Rh factor', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100010, 'Hemoglobin', 'Hemoglobin', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100010, 'Hematocrit', 'Hematocrit', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100010, 'Heart rate', 'Heart rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100010, 'Blood pressure', 'Blood pressure', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100010, 'Body temperature (C)', 'Body temperature (C)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100010, 'Respiratory rate', 'Respiratory rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100010, 'Oxygen saturation', 'Oxygen saturation', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100010, 'Platelet count', 'Platelet count', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100010, 'Blood type', 'Blood type', 'Y');

COMMIT;

-- Procedure: X-Ray (100011)

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100011, 'Body region measured', 'Body region measured', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100011, 'Body thickness at site', 'Body thickness at site', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100011, 'Pregnancy status', 'Pregnancy status', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100011, 'Weight (kg)', 'Weight (kg)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100011, 'Height (cm)', 'Height (cm)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100011, 'Heart rate', 'Heart rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100011, 'Blood pressure', 'Blood pressure', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100011, 'Range of motion', 'Range of motion', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100011, 'Pain level (1-10)', 'Pain level (1-10)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100011, 'Radiation dose', 'Radiation dose', 'Y');

COMMIT;

-- Other (100012)

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100012, 'Weight (kg)', 'Weight (kg)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100012, 'Height (cm)', 'Height (cm)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100012, 'Blood pressure', 'Blood pressure', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100012, 'Heart rate', 'Heart rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100012, 'Respiratory rate', 'Respiratory rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100012, 'Body Temperature (C)', 'Body Temperature (C)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100012, 'Oxygen saturation', 'Oxygen saturation', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100012, 'Pain level (1-10)', 'Pain level (1-10)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100012, 'Blood Glucose', 'Blood Glucose', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100012, 'BMI', 'Body Mass Index', 'Y');

COMMIT;

-- Procedure: Collect Vitals (100038)

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100038, 'Body Temperature (C)', 'Body Temperature (C)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100038, 'Heart rate', 'Heart rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100038, 'Respiratory rate', 'Respiratory rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100038, 'Blood pressure (systolic/diastolic)', 'Blood pressure (systolic/diastolic)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100038, 'Blood pressure (systolic)', 'Blood pressure (systolic)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100038, 'Oxygen saturation', 'Oxygen saturation', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100038, 'Pain level (1-10)', 'Pain level (1-10)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100038, 'Height (cm)', 'Height (cm)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100038, 'Weight (kg)', 'Weight (kg)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100038, 'Blood pressure', 'Blood pressure', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100038, 'Blood Glucose', 'Blood Glucose', 'Y');

COMMIT;

-- Alerts/CCI/SPI (100040)
-- Ref: https://policy.nshealth.ca/site_published/nsha/document_render.aspx?documentRender.IdType=6&documentRender.GenericField=&documentRender.Id=108815

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Malignant Hyperthermia','Alert: Malignant Hyperthermia', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Pescription Alert', 'Alert: Pescription Alert', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Palliative Care', 'Alert: Palliative Care', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: MRSA Contact', 'Alert: MRSA Contact', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: MRSA Positive','Alert: MRSA Positive', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: VRE Contact','Alert: VRE Contact', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: VRE Positive','Alert: VRE Positive', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Difficult Intubation','Alert: Difficult Intubation', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Community Rx Order','Alert: Community Rx Order', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Certificate of Leave',  'Alert: Certificate of Leave', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Previous History of Banned Substance', 'Alert: Previous History of Banned Substance', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: IWK Emergency Care Plan', 'Alert: IWK Emergency Care Plan', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Required Irradiated Blood Product','Alert: Required Irradiated Blood Product', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Research', 'Research', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Advance Directives/Personal Directive', 'Alert: Advance Directives/Personal Directive', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Potential Aggressive Behavior - Family', 'Alert: Potential Aggressive Behavior - Family', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Bleeding Disorder', 'Alert: Bleeding Disorder', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Custody Order', 'Alert: Custody Order', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Active Chemotherapy', 'Alert: Active Chemotherapy', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Hearing Impaired', 'Alert: Hearing Impaired', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Immunosuppressed', 'Alert: Immunosuppressed', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Alert: Language Interpreter', 'Alert: Language Interpreter', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'CCI: Palliative Care', 'CCI: Palliative Care', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'CCI: Do Not Resuscitate (DNR)', 'CCI: Do Not Resuscitate (DNR', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100040, 'Drug Interactions', 'Drug Interactions', 'Y');

COMMIT;

-- Appointments (100041)

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100041, 'Clinic (Health Authority)', 'Clinic (Health Authority)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100041, 'Clinic (Private)', 'Clinic (Private)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100041, 'Acute Facility', 'Acute Facility', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100041, 'Community Facility', 'Community Facility', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100041, 'Specialist Referral', 'Specialist Referral', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100041, 'External Agency', 'External Agency', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100041, 'Virtual Exam', 'Virtual Exam', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100041, 'Phone Exam', 'Phone Exam', 'Y');

COMMIT;

-- Allergies (100043)

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100043, 'Drug Interactions', 'Drug Interactions', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100043, 'Heart rate', 'Heart rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100043, 'Blood pressure (systolic/diastolic)', 'Blood pressure (systolic/diastolic)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100043, 'Body Temperature (C)', 'Body Temperature (C)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100043, 'Respiratory rate', 'Respiratory rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100043, 'Oxygen saturation', 'Oxygen saturation', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100043, 'Swelling/edema', 'Swelling/edema', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100043, 'Eosinophil count', 'Eosinophil count', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100012, 'Pain level (1-10)', 'Pain level (1-10)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100043, 'IgE level', 'IgE level', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100043, 'Skin reaction (rash/hives)', 'Skin reaction (rash/hives)', 'Y');

COMMIT;

-- Immunizations (100045)

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Vaccine Name/Manufacturer', 'Vaccine Name/Manufacturer', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Prior Vaccination Date', 'Prior Vaccination Date', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Dosage (ml)', 'Dosage (ml)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Height (cm)', 'Height (cm)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Weight (kg)', 'Weight (kg)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Heart rate', 'Heart rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Blood pressure (systolic/diastolic)', 'Blood pressure (systolic/diastolic)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Body Temperature (C)', 'Body Temperature (C)', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Respiratory rate', 'Respiratory rate', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Oxygen saturation', 'Oxygen saturation', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Antibody titre', 'Antibody titre', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100045, 'Injection site reaction', 'Injection site reaction', 'Y');

COMMIT;

-- Diagnoses (100045)

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100042, 'Detectable Signs', 'Detectable Signs', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100042, 'Detectable Symptoms', 'Detectable Symptoms', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100042, 'Expected Undetectable Signs', 'Expected Undetectable Signs', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100042, 'Expected Undetectable Symptoms', 'Expected Undetectable Symptoms', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100042, 'Non-Falsified Signs', 'Non-Falsified Signs', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100042, 'Non-Falsified Symptoms', 'Non-Falsified Symptoms', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100042, 'Diagnostics Performed', 'Diagnostics Performed', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100042, 'Diagnostics Not Performed', 'Diagnostics Not Performed', 'Y');

COMMIT;

-- Additional Documentation

INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100046, 'Photocopy/Scan', 'Photocopy/Scan', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100046, 'Emailed File', 'Emailed File', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100046, 'Photograph/Digital Image', 'Photograph/Digital Image', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100046, 'Physical Paper Record', 'Physical Paper Record', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100046, 'Identification Card', 'Identification Card', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100046, 'Device Instructions', 'Device Instructions', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100046, 'Physician Instructions', 'Physician Instructions', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100046, 'Online Instructions', 'Online Instructions', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100046, 'Written Prescription', 'Written Prescription', 'Y');
INSERT INTO common_reference_type(group_id, name, description, active_flag) VALUES (100046, 'External Fax', 'External Fax', 'Y');

COMMIT;