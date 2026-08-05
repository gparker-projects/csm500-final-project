-- summary details of all site patients, regardless of current user
--
SELECT p.id "patient_id", e.id "encounter_id", e.location_id, legal_first_name, legal_last_name, legal_middle_names, sin, birthdate, admit_timestamp, admit_notes, discharge_notes, discharge_timestamp
FROM patient p
join encounter e on p.id = e.patient_id
where location_id in (
	select l.id
	from location l
	where site_id in (
	  select site_id
	  from user_permission up
	  where up.site_id = l.site_id)
)

-- summary details of all site patients admitted, at a site of the current user
--
SELECT p.id "patient_id", e.id "encounter_id", e.location_id, legal_first_name, legal_last_name, legal_middle_names, sin, birthdate, admit_timestamp, admit_notes, discharge_notes, discharge_timestamp
FROM patient p
join encounter e on p.id = e.patient_id
where location_id in (
	select l.id
	from location l
	where site_id in (
	  select site_id
	  from user_permission up
	  where users_id = 2
	    and up.site_id = l.site_id)
)
