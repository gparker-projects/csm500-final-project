-- This script should be run prior to a full-set of unit test scripts, as there may be data that will directly 
-- conflict with the tests being performed. Specifically, some tests require some data to not be present, but 
-- also require pre-existing data.

-- In a full production implementation, these deletes would be performed automatically, but we are short on time.

-- For test: test_get_preferences_for_new_never_existed_user()
-- 
delete from feature_preference where users_id = 7;

