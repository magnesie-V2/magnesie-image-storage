#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn home() -> &'static str {
    "This is home!"
}
fn main() { 
    rocket::ignite()
        .mount("/", routes![home])
        .mount("/files", StaticFiles::from("/hostedFiles"))
        .launch();


}