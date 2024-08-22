extern crate rocket;
extern crate rocket_db_pools;

pub mod models;
pub mod schema;
pub mod services;

#[rocket::get("/")]
async fn index() -> &'static str {
    "hello world"
}

#[rocket::launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(services::stage())
        .mount("/", rocket::routes![index])
}
