
/// performs a connect to the database
/// 
/// check by going to: http://127.0.0.1:8000/db 
///                    http://localhost:8000/db
/// 
async fn db(_req: HttpRequest) -> impl Responder {
  println!("-> /db Requested");
  // TODO: use a pool instead, as this will block another query/result in multiple connections the DB may not be able to accomodate
  let cur_db_conn = auth_objects::AuthObjects::new(DB_CONN_STR).await;
  let users = cur_db_conn.get_users( Some(10), 0).await.expect( &errors::DatabaseError::NotFoundError.to_string() );

  let mut s = String::new();
  for row in users.iter() {                          // row: &User
      s = s + &row.id.to_string() + " " + &row.username + "; ";
  }
  HttpResponse::Ok().body(format!("Users: {}, #{}", s, users.len()))
}



/// Checks the user is in the database, and that the password matches (TODO)
    /// Returns a true/false value
    /// 
    pub async fn OLD_can_user_login(&self,
                            user_name: String, 
                            user_password: String,
                          ) -> Result< bool, std::io::Error> {

        // may not be needed, but this works: a single result/single variable to capture the count (true/false)
        // from the datbase query
        #[derive(sqlx::FromRow)]
        struct SingleResult{
            pub count: i64,
        }

        // query the database for a user that matches the username and password (TODO)
        let query = format!("SELECT COUNT(ID) FROM USERS WHERE USERNAME = '{}' AND PASSWORD = '{}'", user_name, user_password);

        println!("Query: {}", query);

        match sqlx::query(&query)
            .map(|row: PgRow| SingleResult {
                count: row.get("count"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(results) => {
                // if there are no results or an error, the query did not find a valid user for the username/pw combo
                // if there is an exact match only, then the procedure succeeds.
                if results[0].count == 0{
                    println!("No results for: {} ({})", user_name, results[0].count);
                    Ok(false)
                }
                else {
                    println!("Successful login (results found) for: {}", user_name);
                    Ok(true)
                }
            } 
            Err(_e) => {
                println!("Error on login for: {}", user_name);
                Ok(false)
            }
        }
    }