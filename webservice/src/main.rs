#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod beans;
pub mod cors;
pub mod models;
pub mod routes;
pub mod schema;

use std::env;

use rocket_contrib::serve::StaticFiles;

/// Database connection
#[database("rocket_app")]
pub struct DbConn(diesel::MysqlConnection);

fn main() {
    let _test_env_var = &env::var("HOSTED_FILES_FOLDER").unwrap();
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::home,
                routes::list_new_submissions,
                routes::update_submission,
                routes::submit,
                routes::list_users,
                routes::list_sites
            ],
        )
        .mount("/hostedFiles", StaticFiles::from("/hostedFiles"))
        .attach(DbConn::fairing())
        .attach(cors::CorsFairing)
        .launch();
}
