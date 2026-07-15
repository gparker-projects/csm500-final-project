use derive_more::Display;

/// Ref: Practical Rust Projects, pg 181
/// 
/*
#[derive(Display, Debug)]
pub enum StatusCode {
    #[display("Bad Request")]
    BadRequest,
    #[display("Internal server error")]
    InternalServerError,
    #[display("Not found")]
    NotFoundError,
}*/

#[derive(Display, Debug)]
pub enum DatabaseError {
    //#[display("Invalid input parameter")]
   // ValidationError,
   // #[display("Datbase pool error")]
   // DBPoolGetError,
    #[display("Not found")]
    NotFoundError,
   // #[display("Internal server error")]
   // UnexpectedError,
   // #[display("Couldn't establish DB connection")]
   // ConnectionError,
}