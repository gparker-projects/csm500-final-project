pub mod dao{
    use sqlx::postgres::{PgPoolOptions, PgPool}; 
    use sqlx::Row;
    use std::io::{Error, ErrorKind};

    #[derive(Debug, Clone)]
    pub struct CommonDAO {
        pub connection: PgPool,
    }

    impl CommonDAO {
        /// Creates a new AuthObjects object, with a database pool for use by other calls
        /// 
        pub async fn new(db_url: &str) -> Self {

            let db_pool = match PgPoolOptions::new()
                .max_connections(5)
                .connect(db_url)
                .await
            {
                Ok(pool) => pool,
                Err(e) => panic!("{}", e),
            };

            AuthObjects {
                connection: db_pool,
            }
        }

        pub fn get_departments(&self){
            // SELECT id, name FROM department where expiry_timestamp > now()
        }

        ///
        pub fn get_locations(&self){
            // SELECT id "location_id", name, building, wing, floor, room_identifier, notes FROM location where site_id = 9
        }

    }
}