use rocket::fairing::AdHoc;
use rocket::response::Debug;
use rocket_db_pools::diesel::PgPool;
use rocket_db_pools::Database;

pub mod tabletop;
pub mod decks;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Database)]
#[database("dicetower")]
struct Db(PgPool);

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("postgres stage", |rocket| async {
        rocket.attach(Db::init())
    })
}
