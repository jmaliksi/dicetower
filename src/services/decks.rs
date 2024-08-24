use rocket::fairing::AdHoc;
use rocket::{get, post, put};
use rocket::{
    response::status::{Accepted, Created},
    serde::json::Json,
};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};

use crate::models::{Card, CardUpdate, DeckArchetype, NewCard};

use super::{Db, Result};

use crate::schema::cards::dsl::cards;
use crate::schema::deck_archetypes::dsl::deck_archetypes;

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

#[derive(Serialize, Deserialize, Clone)]
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

#[post("/", format = "json", data = "<req>")]
async fn create_archetype(
    mut db: Connection<Db>,
    req: Json<NewArchetypeRequest>,
) -> Result<Created<Json<DeckArchetype>>> {
    use crate::schema::deck_archetypes::dsl::name;
    let daname = req.name.to_string();
    let inserted_arch = db
        .transaction::<DeckArchetype, diesel::result::Error, _>(|mut conn| {
            Box::pin(async move {
                let da = diesel::insert_into(deck_archetypes)
                    .values(name.eq(daname))
                    .returning(DeckArchetype::as_returning())
                    .get_result(&mut conn)
                    .await?;
                let daid = da.id;
                let newcards: Vec<NewCard> = req
                    .cards
                    .clone()
                    .into_iter()
                    .map(|c| NewCard {
                        archetype_id: daid,
                        name: c.name,
                        body: c.body,
                        image: c.image,
                    })
                    .collect();
                diesel::insert_into(cards)
                    .values(newcards)
                    .execute(&mut conn)
                    .await?;
                Ok(da)
            })
        })
        .await?;
    Ok(Created::new("/").body(Json(inserted_arch)))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("card and deck routes", |rocket| async {
        rocket
            .mount(
                "/cards",
                rocket::routes![create_card, get_cards, update_card],
            )
            .mount("/archetypes/decks", rocket::routes![create_archetype])
    })
}
