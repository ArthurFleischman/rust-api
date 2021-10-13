pub mod users;
use rocket::Route;
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn get_routes() -> Vec<Route> {
    rocket::routes![index, users::get_users]
}
