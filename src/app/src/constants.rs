//! ---------------------------------------------------------------------------------
//! Defines standard constants used across the application. Centralized
//!  for reuse, and also for easier management.
//! ---------------------------------------------------------------------------------
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! ---------------------------------------------------------------------------------

pub const SYSTEM_CONFIGURATION_FILE : &str = "\\config\\maple-config.toml";

pub const DATA_SUB_DIRECTORY: &str = "\\data\\"; // TODO

pub const DATABASE_ERROR_NOT_FOUND : &str = "Not Found";
pub const GENERAL_ERROR_NOT_FOUND : &str = "Not Found";
pub const SESSION_ERROR_INVALID : &str = "User session invalid";

pub const SYSTEM_DATETIME_FORMAT : &str= "%Y-%b-%d %H:%M:%S";

pub const CARGO_MANIFEST_DIR : &str= r##"CARGO_MANIFEST_DIR"##;

pub const ERROR_READING_TEMPLATE : &str = "Error reading tile template file";

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

pub const COMMON_REF_TYPE_INTERVENTION_GROUP_ID: i64 = 1;

/// ------------------   ------------------   ------------------   ------------------
//  Externalized HTML tags that will be present in the static tile files (*.htl)
/// ------------------   ------------------   ------------------   ------------------

pub const RELEASE_NUMBER: &str = r##"<div id="MapleEMR::ReleaseNumber"></div>"##;
pub const LEGACY_MENU_TILE_TAG : &str = r##"<div id="MapleEMR::LegacyMenu"></div>"##;
pub const USER_IDENTITY_TILE_TAG : &str = r##"<div id="MapleEMR::UserIdentity"></div>"##;
pub const BODY_TILE_CONTENT_TAG: &str = r##"<div id="MapleEMR::BodyTile"></div>"##;

pub const FEATURE_PREFERENCE_TILE_TAG : &str = r##"<div id="MapleEMR::FeaturePreference"></div>"##;

pub const PATIENT_HEADER_TILE_TAG : &str = r##"<div id="MapleEMR::PatientHeader"></div>"##;
pub const CURRENT_ENCOUNTER_TILE_TAG : &str =r##"<div id="MapleEMR::CurrentEncounter"></div>"##;
pub const CURRENT_INTERVENTIONS_TILE_TAG : &str =r##"<div id="MapleEMR::CurrentInterventions"></div>"##;
pub const ENCOUNTER_HISTORY_TILE_TAG : &str =r##"<div id="MapleEMR::EncounterHistory"></div>"##;

pub const SECTION_1_VISIBLE_TAG : &str = r##"<div id="MapleEMR::SectionVisible_1"></div>"##;
pub const SECTION_2_VISIBLE_TAG : &str = r##"<div id="MapleEMR::SectionVisible_2"></div>"##;
pub const SECTION_3_VISIBLE_TAG : &str = r##"<div id="MapleEMR::SectionVisible_3"></div>"##;
pub const SECTION_4_VISIBLE_TAG : &str = r##"<div id="MapleEMR::SectionVisible_4"></div>"##;

pub const INTERVENTION_TYPE_DROP_DOWN_CONTROL_TAG : &str = r##"<div id="MapleEMR::InterventionTypeDropDownControl"></div>"##;

pub const LEGACY_MENU_ON_ERROR : &str = r##"<div id="legacyMenu" align="left"><ul><li><a class="menuNotCurrent" href="\home">My Dashboard</li></ul></div>"##;

// The Intervention Details editing section has a lot of inline/control level replaces, so the regular tagging above becomes cumbersome to work with
pub const ITEM_ID_INLINE_TAG : &str = r##"{MapleEMR::itemId}"##; // This one is different from the rest because it will be used multiple times in a single tile
pub const INTERVENTION_DETAILS_TYPE_DROP_DOWN_CONTROL_TAG : &str = r##"<div id="MapleEMR::InterventionDetailsTypeDropDownControl"></div>"##;              