//! Defines a Data Transfer Object that contains permissions and departments
//!  associated with a user.
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 

use serde::{Deserialize, Serialize};

/// -------------------------------------------------------------------
/// Defines a Data Transfer Object that contains a single Permission 
/// associated with a Department, to be used with a user. The object 
/// is effectively read-only as it is loaded from the database once 
/// and then referenced (only) for the life of the user session.
/// -------------------------------------------------------------------
/// 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Permission {
    #[serde(rename = "department_id")]
    pub department_id: i64, 
    #[serde(rename = "permission_id")]
    pub permission_id: i64
}

impl Permission {

    /// ### Permission::New()
    ///    Basic constructor for a new Permission. Note: used by test cases only
    /// 
    /// #### Parameters:
    /// * permission_id (i64): the id of the Permission for a user
    /// * department_id (i64): the id of the Department associated with the permission
    /// 
    /// #### Returns:
    /// * a new Permission struct
    /// 
    #[allow(dead_code)]
    pub fn new(department_id: i64,
               permission_id: i64,
            ) -> Self {
        Self { 
            department_id,
            permission_id
        }
    }

    ///
    /// The constants are intimately tied to data in the PERMISSION table, which is used in conjunction with the
    ///  USER_PERMISSION table data to control User access. At some point the code "needs to know" what a specific
    ///  ID is equivalent to in terms of usability/access control. This set of constants provides that lowest level
    ///  mapping between a database entry and the application code.
    /// 
    /// Note: These values MUST match database PERMISSION table entries, in order for CRUD to function with the application.
    /// 
    pub const ALLOW_LOGIN: i64 = 1;
    pub const ALLOW_CREATE_CLINICAL_INTERVENTION : i64 = 2;
    pub const ALLOW_CREATE_NON_CLINICAL_INTERVENTION : i64 = 3;
    pub const ALLOW_CREATE_UPDATE_ADMIT : i64 = 4;
    pub const ALLOW_CREATE_UPDATE_DISCHARGE : i64 = 5;
    pub const ALLOW_UPDATE_CLINICAL_INTERVENTION : i64 = 6;
    pub const ALLOW_UPDATE_NON_CLINICAL_INTERVENTION : i64 = 7;
    pub const ALLOW_VIEW_ADMIT : i64 = 8;
    pub const ALLOW_VIEW_ANY_CLINICAL_DATA : i64 = 9;
    pub const ALLOW_VIEW_CLINICAL_INTERVENTION : i64 = 10;
    pub const ALLOW_VIEW_DISCHARGE : i64 = 11;
    pub const ALLOW_VIEW_NON_CLINICAL_INTERVENTION : i64 = 12;

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

    /// ### UserAuthorization::New()
    ///    Basic constructor for a new UserAuthorization. Note: used by test cases only
    /// 
    /// #### Parameters:
    /// * pv ( Vec<Permission> ): a vector of Permissions for the user to have
    /// 
    /// #### Returns:
    /// * a new UserAuthorization
    /// 
    #[allow(dead_code)]
    pub fn new(pv: Vec<Permission>) -> Self {
        Self { 
            granted_permissions: pv
        }
    }

    /// ### has_permission()
    ///    Confirms the permission set has a specific permission; ignores department
    /// 
    /// #### Parameters:
    /// * permission_id (i64): the id of the Permission which were are checking for the user to have
    /// 
    /// #### Returns:
    /// * bool: true/false for if the user does/does not have the permission
    /// 
    pub fn has_permission(&self, permission_id: i64) -> bool{
        for p in self.granted_permissions.iter() {
            if p.permission_id == permission_id {
                return true;
            } 
        }
        return false;
    }
    
    // Note: used by test cases only
    /// ### has_permission_for_dept()
    ///    Confirms the permission set has a specific permission, for a department
    /// 
    /// #### Parameters:
    /// * permission_id (i64): the id of the Permission which were are checking for the user to have
    /// * department_id (i64): the id of the Department associated with the permission
    /// 
    /// #### Returns:
    /// * bool: true/false for if the user does/does not have the permission
    /// 
    #[allow(dead_code)] 
    pub fn has_permission_for_dept(&self, permission_id: i64, department_id: i64) -> bool{
        for p in self.granted_permissions.iter() {
            if p.permission_id == permission_id && p.department_id == department_id {
                return true;
            } 
        }
        return false;
    } 
}