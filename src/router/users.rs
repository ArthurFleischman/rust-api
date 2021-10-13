use crate::database;
use crate::diesel::prelude::*;
use crate::models::user::User;
use rocket::serde::json::Json;

#[get("/users")]
pub fn get_users() -> Json<Vec<User>> {
    use crate::schema::users::dsl::*;
    let conn: PgConnection = database::config::get_pool();
    let user_list: Vec<User> = users
        .filter(email.eq("cabralfle@gmail.com"))
        .load::<User>(&conn)
        .expect("error loading code");
    Json(user_list)
}
