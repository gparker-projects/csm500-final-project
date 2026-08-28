/// ---------------------------------------------------------------------------------
/// Defines standard database queries (as constants) used across the application. 
/// Centralized for reuse, and also for easier management.
/// ---------------------------------------------------------------------------------
/// 

// all patients at the current site of the user (per their permissions), where the patient has not been discharged
pub const QRY_ALL_PATIENTS_AT_USERS_SITE_NO_DISCHARGE: &str = r##"SELECT p.id, e.id, e.location_id, legal_first_name, legal_last_name, COALESCE(legal_middle_names, '') as "legal_middle_names",
                                        COALESCE(admit_notes, '') as "admit_notes", COALESCE(discharge_notes, '') as "discharge_notes",
                                        birthdate, admit_timestamp,
                                        discharge_timestamp as "discharge_timestamp?",
                                        phn, l2.short_name "location_short_name"
                                    FROM patient p
                                    join encounter e on p.id = e.patient_id
									join location l2 on l2.id = e.location_id
                                    where discharge_timestamp is null
                                      and location_id in (
                                        select l.id
                                        from location l
                                        where site_id in (
                                        select site_id
                                        from user_permission up
                                        where users_id = {}
                                            and up.site_id = l.site_id)  )"##;

// all encounters for a specific patient
pub const QRY_ALL_ENCOUNTERS: &str = r##"
                                    select e.id "encounter_id", admit_timestamp,
                                        discharge_timestamp as "discharge_timestamp?",
                                        COALESCE(admit_notes, '') as "admit_notes",
                                        COALESCE(discharge_notes, '') as "discharge_notes",
                                        s.name "encounter_site_name",
                                        l.short_name "room_identifier",
                                        CASE WHEN discharge_timestamp is null then 'Y' else 'N' end as "current_encounter"
                                    from encounter e
                                    join location l on e.location_id = l.id
                                    join site s on s.id = l.site_id
                                    where patient_id = {}
                                    order by admit_timestamp desc
                                    "##;

// only the current encounter of a specific patient
pub const QRY_CURRENT_ENCOUNTER: &str = r##"
                                    select e.id "encounter_id", admit_timestamp,
                                        discharge_timestamp as "discharge_timestamp?",
                                        COALESCE(admit_notes, '') as "admit_notes",
                                        COALESCE(discharge_notes, '') as "discharge_notes",
                                        s.name "encounter_site_name",
                                        l.short_name "room_identifier",
                                        'Y' as "current_encounter"
                                    from encounter e
                                    join location l on e.location_id = l.id
                                    join site s on s.id = l.site_id
                                    where patient_id = {}
                                      and discharge_timestamp is null
                                      limit 1
                                    "##;

pub const QRY_INTERVENTIONS_FOR_ENC_ID: &str = r##"
                                SELECT i.id intervention_id, encounter_id, i.description, i.notes, location_id, users_id,
                                     intervention_type_id, status_id,
                                    l.room_identifier,
                                    ref1.name "intervention_type", 
                                    ref2.name "status", i.scheduled_timestamp, i.performed_timestamp
                                FROM intervention i
                                join location l on  l.id = i.location_id
                                join common_reference_type ref1 on i.intervention_type_id = ref1.id
                                join common_reference_type ref2 on i.status_id = ref2.id 
                                where i.encounter_id = {}
                                ORDER BY i.id DESC
                                               "##;

// vitals are defined in the first intervention of an Encounter, with a type if of 38
pub const QRY_CURRENT_VITALS_FOR_ENC_ID: &str = r##"
                                SELECT i.id intervention_id, encounter_id, i.description, i.notes, location_id, users_id,
                                    intervention_type_id, status_id,
                                    l.room_identifier,
                                    ref1.name "intervention_type", 
                                    ref2.name "status", i.scheduled_timestamp, i.performed_timestamp
                                FROM intervention i
                                join location l on  l.id = i.location_id
                                join common_reference_type ref1 on i.intervention_type_id = ref1.id
                                join common_reference_type ref2 on i.status_id = ref2.id 
                                where i.encounter_id = {}
                                  AND i.intervention_type_id = 38
                                ORDER BY i.id DESC LIMIT 1
                                                "##; // VITALS ARE REF ID = 38

pub const QRY_INTERVENTION_FOR_ID: &str = r##"SELECT i.id intervention_id,
                                                encounter_id,
                                                i.description,
                                                i.notes,
                                                i.location_id,
                                                users_id,
                                                intervention_type_id,
                                                status_id,
                                                l.room_identifier,
                                                ref1.name "intervention_type", 
                                                ref2.name "status",
                                                i.scheduled_timestamp,
                                                i.performed_timestamp,
                                                e.patient_id
                                            FROM intervention i
                                            join encounter e ON e.id = i.encounter_id
                                            join location l on  l.id = i.location_id
                                            join common_reference_type ref1 on i.intervention_type_id = ref1.id
                                            join common_reference_type ref2 on i.status_id = ref2.id 
                                            where i.id = {}
                                              "##;


pub const QRY_SINGLE_PATIENT_DETAILS: &str =  r##"SELECT p.id "patient_id", e.id "encounter_id", e.location_id "location_id", legal_first_name, legal_last_name, COALESCE(legal_middle_names, '') as "legal_middle_names",
                                                COALESCE(admit_notes, '') as "admit_notes", COALESCE(discharge_notes, '') as "discharge_notes",
                                                birthdate, admit_timestamp,
                                                discharge_timestamp as "discharge_timestamp?",
                                                sin, phn, l2.short_name "location_short_name"
                                            FROM patient p
                                            join encounter e on p.id = e.patient_id
											join location l2 on l2.id = e.location_id
                                            where patient_id = {}"##;



pub const QRY_ALL_INTERVENTION_DETAILS: &str =  r##"
                                                    SELECT i.id "intervention_details_id", value, notes, entry_timestamp, ref1.name "intervention_type", intervention_id, type_id
                                                        FROM intervention_details i
                                                        JOIN common_reference_type ref1 on i.type_id = ref1.id 
                                                        WHERE intervention_id = {}
                                                "##;

pub const QRY_ALL_INTERVENTION_DETAILS_FOR_TYPE: &str = r##"
                                                    SELECT i.id "intervention_details_id", value, notes,
                                                        entry_timestamp,
                                                        ref1.name "intervention_type", intervention_id, type_id
                                                        FROM intervention_details i
                                                        JOIN common_reference_type ref1 on i.type_id = ref1.id 
                                                        WHERE intervention_id = {1}
                                                            AND type_id = {2}
                                                "##;


pub const QRY_COMMON_REF_TYPES_FOR_GROUP: &str = r##"select id, name, description from common_reference_type where group_id = {group_id} ORDER BY NAME"##;

pub const QRY_COMMON_REF_TYPES_FOR_GROUP_ACTIVE_ONLY: &str = r##"select id, name, description from common_reference_type where group_id = {group_id} and active_flag = 'Y' ORDER BY NAME"##;


pub const QRY_ACTIVE_LOCATIONS: &str = r##"select id, name ||' (' || short_name|| ')' from location where active_flag = 'Y'"##; // used by CommonDAO... not sure why the IDE keeps flagging as unused

pub const QRY_ACTIVE_DEPARTMENTS: &str = r##"select id, name, name "description" from department where expiry_timestamp > now()"##;

pub const QRY_COMMON_REF_TYPES_SINGLE_FOR_A_GROUP_AND_TYPE: &str = r##"SELECT id, name, description FROM common_reference_type WHERE ID = {common_ref_id} LIMIT 1"##;


pub const QRY_CURRENT_USER_LOCATIONS: &str = r##"
                                        select id, name || ' (' || short_name|| ')' from location
                                        where active_flag = 'Y'
                                        and site_id in (
                                            select site_id
                                            from user_permission
                                            where users_id = {}
                                                and active_flag = 'Y' )"##;


pub const QRY_ALL_USERS_AND_DEPARTMENT_NAME: &str = r##"
                                                select distinct u.id user_id,
                                                    d.name || ' ('|| u.name||')' "name",
                                                    u.name "user_name"
                                                from user_permission up
                                                join users u on up.users_id = u.id
                                                join department d on up.department_id = d.id
                                                where up.active_flag = 'Y'
                                                and site_id is not null
												order by  d.name || ' ('|| u.name||')'
                                                "##;    // Removed clause --and users_id = {user_id}


pub const QRY_GET_ALL_ACTIVE_FEATURE_PREFERENCE_FOR_USER: &str = r##"SELECT id, display_order, weight, calculation_date, department_id, feature_id
                                                                     FROM feature_preference
                                                                     where active_flag = 'Y' and users_id={users_id}
                                                                     order by weight, calculation_date"##;

// ------------------------------------------------------------------------------------------
// 
// Insert/Update statements
// 
// ------------------------------------------------------------------------------------------
// 
pub const UPSERT_PATIENT: &str = r##"
    INSERT INTO patient (legal_last_name, legal_first_name, legal_middle_names, birthdate, phn)
    VALUES ('{legal_last_name}', '{legal_first_name}', '{legal_middle_names}', to_timestamp('{birthdate}', 'YYYY/MM/DD'), {phn})
    ON CONFLICT (phn)
    DO UPDATE SET
        legal_first_name   = EXCLUDED.legal_first_name,
        legal_last_name    = EXCLUDED.legal_last_name,
        legal_middle_names = EXCLUDED.legal_middle_names,
        birthdate          = EXCLUDED.birthdate
    RETURNING ID; "##;
                
// (to_timestamp('15-08-2026 14:30:00', 'DD-MM-YYYY HH24:MI:SS'));

pub const INSERT_ENCOUNTER: &str = r##"
    INSERT INTO encounter( admit_timestamp, admit_notes, discharge_timestamp, discharge_notes, patient_id, location_id)
        VALUES ( NOW(),
                '{admit_notes}',
                NULL,
                NULL,
                {patient_id},
                {location_id}) RETURNING ID;
"##;

pub const UPDATE_ENCOUNTER: &str = r##"
    UPDATE encounter
        SET admit_notes = '{admit_notes}',
            discharge_timestamp = to_timestamp('{discharge_timestamp}', 'YYYY/MM/DD HH24:MI:SS'),
            discharge_notes = '{discharge_notes}',
            patient_id = {patient_id},
            location_id = {location_id}
        WHERE id = {encounter_id} RETURNING ID;
"##;

pub const UPDATE_ENCOUNTER_FOR_DISCHARGE: &str = r##"
    UPDATE encounter
        SET discharge_timestamp = NOW(),
            discharge_notes = '{discharge_notes}'
        WHERE id = {encounter_id} RETURNING ID;
"##;

pub const INSERT_INTERVENTION: &str = r##"
INSERT INTO INTERVENTION(description, notes, location_id, users_id, encounter_id,
                         intervention_type_id, status_id, scheduled_timestamp, performed_timestamp)
        VALUES ('{description}',
        '{notes}',
        {location_id},
        {users_id},
        {encounter_id},
        {intervention_type_id},
        {status_id}, 
        to_timestamp('{scheduled_timestamp}', 'YYYY-Mon-DD HH24:MI:SS'),
        to_timestamp('{performed_timestamp}', 'YYYY-Mon-DD HH24:MI:SS')
        ) RETURNING ID;
"##;

pub const UPDATE_INTERVENTION: &str = r##"
    UPDATE INTERVENTION
    SET description='{description}',
        notes='{notes}',
        location_id={location_id},
        users_id={users_id},
        encounter_id={encounter_id},
        intervention_type_id={intervention_type_id},
        status_id={status_id}, 
        scheduled_timestamp=to_timestamp('{scheduled_timestamp}', 'YYYY-Mon-DD HH24:MI:SS'),
        performed_timestamp=to_timestamp('{performed_timestamp}', 'YYYY-Mon-DD HH24:MI:SS')
    WHERE id={intervention_id} RETURNING ID;
"##;

pub const INSERT_INTERVENTION_DETAILS: &str = r##"
    INSERT INTO INTERVENTION_DETAILS(intervention_id,
                                     type_id, value,
                                     notes, entry_timestamp)
        VALUES ({intervention_id},
                {type_id},
                '{value}',
                '{notes}',
                NOW()) RETURNING ID;
"##;

pub const UPDATE_INTERVENTION_DETAILS: &str = r##"
    UPDATE INTERVENTION_DETAILS
        SET intervention_id={intervention_id},
            type_id={type_id},
            value='{value}',
            notes='{notes}'
        WHERE id={intervention_id} RETURNING ID;
"##;

pub const INSERT_FEATURE_PREFERENCE: &str = r##"
    INSERT INTO feature_preference(
            display_order,
            weight,
            calculation_date,
            users_id,
            department_id,
            active_flag,
            feature_id)
    VALUES (0, 0, NOW(),
            {users_id},
            null, 'Y',
            {feature_id})
    RETURNING ID;
"##;

pub const UPDATE_FEATURE_PREFERENCE: &str = r##"
UPDATE feature_preference
	SET weight = weight + 1,
		calculation_date = NOW()
	WHERE users_id = {users_id}
	  AND feature_id = {feature_id}
	  AND active_flag='Y' RETURNING ID;
"##;