use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::post;
use crate::models::{self, DeckArchetype};
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{PgPool, prelude::*};
use rocket::fairing::AdHoc;

pub mod tabletop;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Database)]
#[database("dicetower")]
struct Db(PgPool);

#[derive(Serialize, Deserialize)]
pub struct TestReq {
    name: String,
}

#[post("/", format = "json", data = "<req>")]
async fn test_endpoint(mut db: Connection<Db>, req: Json<TestReq>) -> Result<Created<Json<DeckArchetype>>> {
    use super::schema::deck_archetypes::dsl::{deck_archetypes, name};
    use models::DeckArchetype;
    let n = req.name.to_string();
    diesel::insert_into(deck_archetypes)
        .values(name.eq(&n))
        .execute(&mut db)
        .await
        .expect("oh no");
    
    let result = deck_archetypes
        .filter(name.eq(&n))
        .select(DeckArchetype::as_select())
        .first(&mut db)
        .await
        .optional();

    match result {
        Ok(Some(result)) => Ok(Created::new("/").body(Json(result))),
        Ok(None) => panic!("it none"),
        Err(_) => panic!("aa"),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("postgres stage", |rocket| async {
        rocket.attach(Db::init())
            .mount("/test", rocket::routes![test_endpoint])
    })
}
