//! Defines a Data Transfer Object that contains permissions and departments
//!  associated with a user.
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Permission {
    #[serde(rename = "department_id")]
    pub department_id: i64, 
    #[serde(rename = "permission_id")]
    pub permission_id: i64
}

impl Permission {
    /// Basic constructor
    /// 
    pub fn new(department_id: i64,
               permission_id: i64,
            ) -> Self {
        Self { 
            department_id,
            permission_id
        }
    }
}

/// -------------------------------------------------------------------
/// Defines a Data Transfer Object that contains permissions and departments
/// associated with a user. The object is effectively read-only as it is
/// loaded from the database once and then referenced (only) for the life
/// of the user session.
/// -------------------------------------------------------------------
/// 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserAuthorization {
    #[serde(rename = "granted_permissions")]
    pub granted_permissions: Vec<Permission>,
} 

impl UserAuthorization {
    // Basic constructor
    // 
    pub fn new(p: Vec<Permission>) -> Self {
        Self { 
            granted_permissions: p
        }
    }

    // confirms the permission set has a specific permission; ignores department
    // TODO: if there is time, this could be done with a HashSet instead. Small dataset however will not benefit much.
    pub fn has_permission(&self, p_id: i64) -> bool{
        for p in self.granted_permissions.iter() {
            if p.permission_id == p_id {
                return true;
            } 
        }
        return false;
    }

    // confirms the permission set has a specific permission, for a department
    // TODO: if there is time, this could be done with a HashSet instead. Small dataset however will not benefit much.
    pub fn has_permission_for_dept(&self, p_id: i64, department_id: i64) -> bool{
        for p in self.granted_permissions.iter() {
            if p.permission_id == p_id && p.department_id == department_id {
                return true;
            } 
        }
        return false;
    }
}