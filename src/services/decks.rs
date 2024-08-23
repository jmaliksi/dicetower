use rocket::fairing::AdHoc;
use rocket::{get, post, put};
use rocket::{
    response::status::{Accepted, Created},
    serde::json::Json,
};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};

use crate::models::{Card, CardUpdate, NewCard};

use super::{Db, Result};

use crate::schema::cards::dsl::cards;

#[post("/", format = "json", data = "<req>")]
async fn create_card(mut db: Connection<Db>, req: Json<NewCard>) -> Result<Created<Json<Card>>> {
    let result = diesel::insert_into(cards)
        .values(req.into_inner())
        .returning(Card::as_returning())
        .get_result(&mut db)
        .await?;
    Ok(Created::new("/").body(Json(result)))
}

#[get("/?<archetype_id>")]
async fn get_cards(mut db: Connection<Db>, archetype_id: Option<i32>) -> Result<Json<Vec<Card>>> {
    use crate::schema::cards::dsl::archetype_id as aid;
    let mut query = cards.select(Card::as_select()).into_boxed();
    if let Some(a) = archetype_id {
        query = query.filter(aid.eq(a));
    }
    let results = query.load(&mut db).await?;
    Ok(Json(results))
}

#[put("/<cid>", format = "json", data = "<req>")]
async fn update_card(
    mut db: Connection<Db>,
    cid: i32,
    req: Json<CardUpdate>,
) -> Result<Accepted<Json<Card>>> {
    let result = diesel::update(cards.find(&cid))
        .set(req.into_inner())
        .returning(Card::as_returning())
        .get_result(&mut db)
        .await?;
    Ok(Accepted(Json(result)))
}

#[derive(Serialize, Deserialize)]
pub struct NewArchetypeCard {
    name: String,
    body: Option<String>,
    image: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewArchetypeRequest {
    name: String,
    cards: Vec<NewArchetypeCard>,
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("card and deck routes", |rocket| async {
        rocket.mount(
            "/cards",
            rocket::routes![create_card, get_cards, update_card],
        )
    })
}
