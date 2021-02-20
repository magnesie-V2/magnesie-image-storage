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

pub mod cors;
pub mod models;
pub mod routes;
pub mod schema;
pub mod beans;

use rocket_contrib::serve::StaticFiles;

#[database("rocket_app")]
pub struct DbConn(diesel::MysqlConnection);

fn main() { 
    rocket::ignite()
        .mount("/", routes![routes::home, routes::list_new_submissions, routes::update_submission, routes::create_user, routes::submit])
        .mount("/files", StaticFiles::from("/hostedFiles"))
        .attach(DbConn::fairing())
        .attach(cors::CorsFairing)
        .launch();
}