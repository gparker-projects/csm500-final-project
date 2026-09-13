//! # dto (Data Transfer Objects)
//! 
//! Structs within the dto module
//!
//! ## Overview
//! This module provides struts and classes that wrap basic data of the application as it is retrieved from or prepared for
//! system use. These allow consistency of use across the system and protect against errant use.
//! 
pub mod convert_utils;
pub mod encounter;
pub mod feature_preference;
pub mod intervention;
pub mod intervention_detail;
pub mod patient;
pub mod user;
pub mod user_auth;