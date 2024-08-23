use super::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
}

#[derive(Queryable, Selectable, Identifiable, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = tabletops)]
pub struct Tabletop {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub created_at: std::time::SystemTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = tabletops)]
pub struct NewTabletop {
    pub name: String,
    pub user_id: i32,
}

#[derive(Queryable, Identifiable, Associations, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(Tabletop, foreign_key = tabletop_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = players)]
pub struct Player {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub tabletop_id: i32,
}

#[derive(Queryable, Selectable, Identifiable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = deck_archetypes)]
pub struct DeckArchetype {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(DeckArchetype, foreign_key = archetype_id))]
#[diesel(table_name = cards)]
pub struct Card {
    pub id: i32,
    pub archetype_id: i32,
    pub name: String,
    pub body: Option<String>,
    pub image: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = cards)]
pub struct NewCard {
    pub archetype_id: i32,
    pub name: String,
    pub body: Option<String>,
    pub image: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = cards)]
pub struct CardUpdate {
    pub name: Option<String>,
    pub body: Option<String>,
    pub image: Option<String>,
}

#[derive(Queryable, Identifiable, Associations, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(DeckArchetype, foreign_key = archetype_id))]
#[diesel(belongs_to(Tabletop, foreign_key = tabletop_id))]
#[diesel(table_name = decks)]
pub struct Deck {
    pub id: i32,
    pub name: String,
    pub tabletop_id: i32,
    pub archetype_id: i32,
    pub draw_pile: Option<Vec<i32>>,
    pub discard_pile: Option<Vec<i32>>,
}
