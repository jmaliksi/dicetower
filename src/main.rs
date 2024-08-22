#[rocket::get("/")]
async fn index() -> &'static str {
    "hello world"
}

#[rocket::launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![index])
}
