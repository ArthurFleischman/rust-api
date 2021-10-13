#![feature(decl_macro)]
mod database;
mod models;
mod router;
mod schema;
#[macro_use]
extern crate rocket;

extern crate serde_json;
#[macro_use]
extern crate diesel;

extern crate serde_derive;

#[rocket::main]

async fn main() {
    if let Err(e) = rocket::build()
        .mount("/", router::get_routes())
        .launch()
        .await
    {
        eprint!("{}", e)
    }
}
