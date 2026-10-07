//! ---------------------------------------------------------------------------------
//! Core methods called by the main program executable for the project
//!
//! CSM500 Project (April - October 2026)
//! Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 
//! ---------------------------------------------------------------------------------


use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tracing;
use tracing_subscriber::{ Layer, filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Default, Clone)]
pub struct MapleHMSKernel;

impl MapleHMSKernel{

    /// ### fn get_static_base_path()
    ///  Obtains the static base path for files from the current system environment variables. This is used for 
    ///  locating files for load.
    /// 
    /// #### Parameters: None
    /// 
    /// #### Returns:
    /// * String - the Base path to the application executable, with "\\webc\\static\\" tacked on the end
    /// 
    pub fn get_static_base_path() -> String {
        std::env::current_dir().expect("Base path to executable could not be found").display().to_string() + "\\webc\\static\\"
    }

    /// ### fn init_database_pool()
    ///   Initializes a Pool<Postgres> database connection pool from Postgres, for system use:
    /// 
    /// #### Parameters:
    /// * db_conn_str: String - string for the connection to connect to the database with
    /// 
    /// #### Returns:
    /// * Pool<Postgres> - the Postgres database pool connection
    /// 
    pub async fn init_database_pool( db_url: String ) -> Option< Pool<Postgres> > {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
        {
            Ok(pool) => {
                tracing::info!("Database connection established to: http://{}", db_url);
                Some(pool)
            },
            Err(e) => {
                tracing::error!("{}", e);
                None
            },
        };
        return db_pool;
    }

    /// ### fn init_logging()
    /// Initializes standard Rust logging for the application, creating a file with name format: maple_hms-%Y-%b-%d_%H%M%S.log
    ///  added per recommendation from 0-to-Prod
    ///  https://rust.code-maven.com/logging/tracing-to-a-file.html
    ///
    /// #### Parameters: None
    /// 
    /// #### Returns:
    /// * String - the file name of the logging/trace file that was created
    /// 
    pub fn init_logging() -> String{
        let log_filename = "maple_hms-".to_owned() + &chrono::Local::now().format("%Y-%b-%d_%H%M%S").to_string() +".log";
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_ansi(false)
                    .with_writer(
                        std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(log_filename.clone() )
                            .unwrap(),
                    )
                    .with_filter( LevelFilter::DEBUG ),
            )
            .init();
        tracing::info!("MapleHMS is running!");
        log_filename 
    }
}