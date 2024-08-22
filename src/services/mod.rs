extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::post;
use crate::models::{self, DeckArchetype};
use std::env;

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    return PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}

#[derive(Serialize, Deserialize)]
pub struct TestReq {
    name: String,
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/", format = "json", data = "<req>")]
pub fn test_endpoint(req: Json<TestReq>) -> Result<Created<Json<DeckArchetype>>> {
    use super::schema::deck_archetypes::dsl::{deck_archetypes, name};
    use models::DeckArchetype;
    let conn = &mut establish_connection_pg();
    let n = req.name.to_string();
    diesel::insert_into(deck_archetypes).values(name.eq(&n)).execute(conn).expect("oh no");
    
    let result = deck_archetypes
        .filter(name.eq(&n))
        .select(DeckArchetype::as_select())
        .first(conn)
        .optional();

    match result {
        Ok(Some(result)) => Ok(Created::new("/").body(Json(result))),
        Ok(None) => panic!("it none"),
        Err(_) => panic!("aa"),
    }
}

