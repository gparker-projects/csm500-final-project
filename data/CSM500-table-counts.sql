select 'common_reference_type' as "table name", count(*) from common_reference_type
union select 'department' as "table name", count(*) from department
union select 'encounter' as "table name", count(*) from encounter
union select 'feature_preference' as "table name", count(*) from feature_preference
union select 'intervention' as "table name", count(*) from intervention
union select 'intervention_details' as "table name", count(*) from intervention_details
union select 'location' as "table name", count(*) from location
union select 'patient' as "table name", count(*) from patient
union select 'permission' as "table name", count(*) from permission
union select 'role' as "table name", count(*) from role
union select 'user_permission' as "table name", count(*) from user_permission
union select 'site' as "table name", count(*) from role
union select 'users' as "table name", count(*) from users