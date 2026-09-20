/// #Unit & Integration tests for the Convert Utils module
/// 
/// ###Includes:
/// * to_i64()
/// * is_aged()

mod common;

#[cfg(test)]
mod convert_utils_tests {
    use maple_emr::constants;

    use maple_emr::{dto::user_auth::*};
    use maple_emr::session::*;

    #[test]
    fn test_convert_utils() {
        assert!(maple_emr::dto::convert_utils::ConvertUtils::to_i64("not a number".to_string()) == constants::INVALID_OTHER_ID, "Not a number did not convert to -1");
        
        let earliest_birth_date = chrono::NaiveDate::from_ymd_opt(1880, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(); // very much aged
        let future = chrono::NaiveDate::from_ymd_opt(2030, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(); // very much aged

        assert!(maple_emr::dto::convert_utils::ConvertUtils::is_aged(chrono::Utc::now().naive_utc(), 0), "Timestamp was not aged, asserted as aged");
        assert!(maple_emr::dto::convert_utils::ConvertUtils::is_aged(earliest_birth_date, 4), "Timestamp was very aged, asserted as not aged");
        assert!(!maple_emr::dto::convert_utils::ConvertUtils::is_aged(future, 4), "Timestamp is in future, asserted as aged (past)");
    }

    #[test]
    fn test_sessions() {
        // create a couple of permission and put them in a Vector, then add to the UserAuthorization
        // the UserAuthorization gets put into the Session
        let perm: Permission = Permission::new(1, 1);
        let perm2: Permission = Permission::new(1, 2);

        let perms: Vec<Permission> = vec![perm, perm2];

        let ua = UserAuthorization {
            granted_permissions: perms
        };

        let sess: UserSession = UserSession {
            user_id: "1".to_string(), // 
            user_display_name: "TEST, UNIT".to_string(), //
            email: "test@gmail.com".to_string(), //
            user_authorizations: ua //
        };

        assert_eq!(sess.get_userid_as_i64(), 1);
        assert_eq!(sess.get_user_display_name(), "TEST, UNIT".to_string());
        assert_eq!(sess.has_permission(1), true);
        assert_eq!(sess.has_permission(9999), false);

        assert!(maple_emr::dto::convert_utils::ConvertUtils::to_i64("not a number".to_string()) == constants::INVALID_OTHER_ID, "Not a number did not convert to -1");
        
    }
}