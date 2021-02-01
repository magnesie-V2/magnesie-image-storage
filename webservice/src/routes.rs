use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::models::*;
// use crate::schema;
use crate::DbConn;

#[get("/")]
pub fn home() -> &'static str {
    "This is home!"
}

#[get("/users")]
pub fn list_users(conn: DbConn) -> Result<Json<Vec<User>>, String> {
    use crate::schema::users::dsl::*;

    users.load(&conn.0).map_err(|err| -> String {
        println!("Error querying users: {:?}", err);
        "Error querying users from the database".into()
    }).map(Json)
}