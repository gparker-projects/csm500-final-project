pub const DATABASE_ERROR_NOT_FOUND : &str = "Not Found";
pub const SESSION_ERROR_INVALID : &str = "User session invalid";

// todo: move this to a config file
// application-wide database string; should come from a configurable parameter file (TODO)
pub const DB_CONN_STR: &str = "postgres://postgres:csm500@localhost:5432/csm500";

pub const USER_SESSION : &str = r##"USER_SESSION"##;
pub const VALIDATION_ERRORS : &str = r##"VALIDATION_ERRORS"##;

// externalized HTML tags that will be present in the static tile files (*.htl)
pub const LEGACY_MENU_TILE_TAG : &str = r##"<div id="MapleEMR::LegacyMenu"></div>"##;
pub const USER_IDENTITY_TILE_TAG : &str = r##"<div id="MapleEMR::UserIdentity"></div>"##;
pub const BODY_TILE_CONTENT_TAG: &str = r##"<div id="MapleEMR::BodyTile"></div>"##;

pub const USER_COMMANDS_TILE_TAG : &str = r##"<div id="MapleEMR::UserCommands"></div>"##;

pub const PATIENT_HEADER_TILE_TAG : &str = r##"<div id="MapleEMR::PatientHeader"></div>"##;
pub const CURRENT_ENCOUNTER_TILE_TAG : &str =r##"<div id="MapleEMR::CurrentEncounter"></div>"##;
pub const CURRENT_INTERVENTIONS_TILE_TAG : &str =r##"<div id="MapleEMR::CurrentInterventions"></div>"##;
pub const ENCOUNTER_HISTORY_TILE_TAG : &str =r##"<div id="MapleEMR::EncounterHistory"></div>"##;

pub const INVALID_PATIENT_ID: i64 = -1;

// all encounters for a specific patient
pub const QRY_ALL_ENCOUNTERS: &str = r##"
                                    select e.id "encounter_id", admit_timestamp,
                                        discharge_timestamp as "discharge_timestamp?",
                                        COALESCE(admit_notes, '') as "admit_notes",
                                        COALESCE(discharge_notes, '') as "discharge_notes",
                                        s.name "encounter_site_name",
                                        l.room_identifier,
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
                                        l.room_identifier,
                                        'Y' as "current_encounter"
                                    from encounter e
                                    join location l on e.location_id = l.id
                                    join site s on s.id = l.site_id
                                    where patient_id = {}
                                      and discharge_timestamp is null
                                    "##;

pub const QRY_ALL_INTERVENTIONS: &str = r##"
                                 SELECT i.id intv_id, i.description, i.notes, location_id, users_id,
                                    encounter_id, intervention_type_id, status_id,
                                    l.room_identifier,
                                    ref1.name "intervention_type", 
                                    ref2.name "status"
                                FROM intervention i
                                join location l on  l.id = i.location_id
                                join common_reference_type ref1 on i.intervention_type_id = ref1.id
                                join common_reference_type ref2 on i.status_id = ref2.id 
                                where i.id = {}
                                ORDER BY i.id DESC
                                    "##;

pub const QRY_MOST_RECENT_INTERVENTION: &str = r##"
                                 SELECT i.id intv_id, i.description, i.notes, location_id, users_id,
                                    encounter_id, intervention_type_id, status_id,
                                    l.room_identifier,
                                    ref1.name "intervention_type", 
                                    ref2.name "status"
                                FROM intervention i
                                join location l on  l.id = i.location_id
                                join common_reference_type ref1 on i.intervention_type_id = ref1.id
                                join common_reference_type ref2 on i.status_id = ref2.id 
                                where i.id = {}
                                ORDER BY i.id DESC
                                limit 1
                                    "##;

pub const QRY_SINGLE_PATIENT_DETAILS: &str =  r##"SELECT p.id "patient_id", e.id "encounter_id", e.location_id "location_id", legal_first_name, legal_last_name, COALESCE(legal_middle_names, '') as "legal_middle_names",
                                                COALESCE(admit_notes, '') as "admit_notes", COALESCE(discharge_notes, '') as "discharge_notes",
                                                birthdate, admit_timestamp,
                                                discharge_timestamp as "discharge_timestamp?",
                                                sin, phn
                                            FROM patient p
                                            join encounter e on p.id = e.patient_id
                                            where patient_id = {}"##;