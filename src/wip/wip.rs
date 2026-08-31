use crate::dao::feature_preference_dao::FeaturePreferenceDAO;

let fpdao = FeaturePreferenceDAO::new( app_session.get_db_connection() ).await;
let _ignore =  fpdao.upsert_feature_preference(user_id: i64, feature_id: i64).await.unwrap();


                // if save successful, record a feature preference as well
                let fpdao = FeaturePreferenceDAO::new( app_session.get_db_connection() ).await;
                let _ignore = fpdao.upsert_feature_preference( user_session_details.clone().get_userid_as_i64() , intv_id.clone()).await.unwrap();


get_all_active_feature_preferences_for_user(&self, user_id: i64)-> Result< Option< Vec<FeaturePreference> >

 ->  Result< Option< i64 >, std::io::Error>
