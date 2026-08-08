pub const DATABASE_ERROR_NOT_FOUND : &str = "Not Found";
pub const SESSION_ERROR_INVALID : &str = "User session invalid";

// todo: move this to a config file
// application-wide database string; should come from a configurable parameter file (TODO)
pub const DB_CONN_STR: &str = "postgres://postgres:csm500@localhost:5432/csm500";

pub const USER_SESSION : &str = r##"USER_SESSION"##;
pub const VALIDATION_ERRORS : &str = r##"VALIDATION_ERRORS"##;

pub const PATIENT_TILE_TAG : &str = r##"<div id="MapleEMR::PatientList"><div/>"##;
pub const LEGACY_MENU_TILE_TAG : &str = r##"<div id="MapleEMR::LegacyMenu"><div/>"##;
pub const _USER_COMMANDS_TILE_TAG : &str = r##"<div id="MapleEMR::UserCommands"><div/>"##;

pub const BODY_TILE_CONTENT: &str = r##"<div id="MapleEMR::BodyTile"><div/>"##;