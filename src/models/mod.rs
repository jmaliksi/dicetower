use super::schema::*;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Identifiable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = tabletops)]
pub struct Tabletop {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(Tabletop, foreign_key = game))]
#[diesel(table_name = players)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub game: Uuid,
}

#[derive(Queryable, Selectable, Identifiable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = deck_archetypes)]
pub struct DeckArchetype {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, Associations, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(DeckArchetype, foreign_key = archetype))]
#[diesel(table_name = cards)]
pub struct Card {
    pub id: i32,
    pub archetype: i32,
    pub name: String,
    pub body: Option<String>,
    pub image: Option<String>,
}

#[derive(Queryable, Identifiable, Associations, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(DeckArchetype, foreign_key = archetype))]
#[diesel(belongs_to(Tabletop, foreign_key = game))]
#[diesel(table_name = decks)]
pub struct Deck {
    pub id: Uuid,
    pub name: String,
    pub game: Uuid,
    pub archetype: i32,
    pub draw_pile: Option<Vec<i32>>,
    pub discard_pile: Option<Vec<i32>>,
}
