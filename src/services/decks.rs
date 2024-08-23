use rocket::fairing::AdHoc;
use rocket::{get, post};
use rocket::{response::status::Created, serde::json::Json};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};

use crate::models::Card;

use super::{Db, Result};

#[derive(Serialize, Deserialize)]
pub struct NewCardRequest {
    archetype_id: i32,
    name: String,
    body: Option<String>,
    image: Option<String>,
}

#[post("/", format = "json", data = "<req>")]
async fn create_card(
    mut db: Connection<Db>,
    req: Json<NewCardRequest>,
) -> Result<Created<Json<Card>>> {
    use crate::schema::cards::dsl::*;
    let aid = req.archetype_id;
    let n = req.name.to_string();
    let b = req.body.clone();
    let img = req.image.clone();
    let result = diesel::insert_into(cards)
        .values((archetype_id.eq(aid), name.eq(n), body.eq(b), image.eq(img)))
        .returning(Card::as_returning())
        .get_result(&mut db)
        .await?;
    Ok(Created::new("/").body(Json(result)))
}

#[get("/?<aid>")]
async fn get_cards(mut db: Connection<Db>, aid: Option<i32>) -> Result<Json<Vec<Card>>> {
    use crate::schema::cards::dsl::{archetype_id, cards};
    let mut query = cards.select(Card::as_select()).into_boxed();
    if let Some(a) = aid {
        query = query.filter(archetype_id.eq(a));
    }
    let results = query.load(&mut db).await?;
    Ok(Json(results))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("card and deck routes", |rocket| async {
        rocket.mount("/cards", rocket::routes![create_card, get_cards])
    })
}
