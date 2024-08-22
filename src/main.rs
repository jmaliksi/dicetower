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
        .mount("/", rocket::routes![index])
        .mount("/test", rocket::routes![services::test_endpoint])
}
