extern crate rocket;
extern crate rocket_db_pools;

pub mod models;
pub mod schema;
pub mod services;

use dotenvy::dotenv;
use std::env;

#[rocket::get("/")]
async fn index() -> &'static str {
    "hello world"
}

#[rocket::launch]
async fn rocket() -> _ {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set!");
    let figment = rocket::Config::figment().merge((
        "databases.dicetower.url",
        db_url,
    ));
    rocket::custom(figment)
        .attach(services::stage())
        .attach(services::tabletop::stage())
        .attach(services::decks::stage())
        .mount("/", rocket::routes![index])
}
