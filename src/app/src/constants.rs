/// ---------------------------------------------------------------------------------
/// Defines standard constants used across the application. Centralized
/// for reuse, and also for easier management.
/// ---------------------------------------------------------------------------------
/// 
pub const DATABASE_ERROR_NOT_FOUND : &str = "Not Found";
pub const GENERAL_ERROR_NOT_FOUND : &str = "Not Found";
pub const SESSION_ERROR_INVALID : &str = "User session invalid";

pub const SYSTEM_DATETIME_FORMAT : &str= "%Y-%b-%d %H:%M:%S";

pub const CARGO_MANIFEST_DIR : &str= r##"CARGO_MANIFEST_DIR"##;

pub const DATA_SUB_DIRECTORY: &str = "\\data\\";

//pub const EMPTY_DATASET : &str = "No data was returned";

pub const ERROR_READING_TEMPLATE : &str = "Error reading tile template file";

// todo: move this to a config file
// application-wide database string; should come from a configurable parameter file (TODO)
pub const DB_CONN_STR: &str = "postgres://postgres:csm500@localhost:5432/csm500";

pub const USER_SESSION : &str = r##"USER_SESSION"##;
pub const VALIDATION_ERRORS : &str = r##"VALIDATION_ERRORS"##;

/// ------------------   ------------------   ------------------   ------------------
/// Record ID constants
/// ------------------   ------------------   ------------------   ------------------
pub const DEFAULT_LOCATION_REGISTRATION: i64 = 20;

pub const INVALID_PATIENT_ID: i64 = -1;

pub const INVALID_OTHER_ID: i64 = -1;

pub const NOT_SPECIFIED_ID: i64 = -1;


pub const DEFAULT_INTERVENTION_STATUS_NEW: i64 = 13;

/// ------------------   ------------------   ------------------   ------------------
//  Externalized HTML tags that will be present in the static tile files (*.htl)
/// ------------------   ------------------   ------------------   ------------------
pub const LEGACY_MENU_TILE_TAG : &str = r##"<div id="MapleEMR::LegacyMenu"></div>"##;
pub const USER_IDENTITY_TILE_TAG : &str = r##"<div id="MapleEMR::UserIdentity"></div>"##;
pub const BODY_TILE_CONTENT_TAG: &str = r##"<div id="MapleEMR::BodyTile"></div>"##;

pub const USER_COMMANDS_TILE_TAG : &str = r##"<div id="MapleEMR::UserCommands"></div>"##;

pub const PATIENT_HEADER_TILE_TAG : &str = r##"<div id="MapleEMR::PatientHeader"></div>"##;
pub const CURRENT_ENCOUNTER_TILE_TAG : &str =r##"<div id="MapleEMR::CurrentEncounter"></div>"##;
pub const CURRENT_INTERVENTIONS_TILE_TAG : &str =r##"<div id="MapleEMR::CurrentInterventions"></div>"##;
pub const ENCOUNTER_HISTORY_TILE_TAG : &str =r##"<div id="MapleEMR::EncounterHistory"></div>"##;

pub const INTERVENTION_TYPE_DROP_DOWN_CONTROL_TAG : &str = r##"<div id="MapleEMR::InterventionTypeDropDownControl"></div>"##;

pub const LEGACY_MENU_ON_ERROR : &str = r##"<div id="legacyMenu" align="left"><ul><li><a class="menuNotCurrent" href="\home">My Dashboard</li></ul></div>"##;

