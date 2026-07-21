SELECT *
FROM PATIENT p
join ENCOUNTER e ON e._PATIENTID = p._id
WHERE SIN = 

SELECT *
FROM USER_PERMISSION
WHERE _USERID = 1;

All admitted patients, discharged (at a site; include location details) -> everyone
All assessments, orders, prescriptions, treatments for a patient -> care assignees
All orders, prescriptions, treatments -> relevant department
All assigned patients -> care assignees

Patient detail summary

Supporting queries
--------
- All permissions for a user
- A user's current department and site
- All feature preferences for a user