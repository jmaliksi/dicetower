// @generated automatically by Diesel CLI.

diesel::table! {
    cards (id) {
        id -> Int4,
        archetype -> Nullable<Int4>,
        name -> Varchar,
        body -> Nullable<Text>,
        image -> Nullable<Text>,
    }
}

diesel::table! {
    deck_archetypes (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    decks (id) {
        id -> Uuid,
        #[max_length = 40]
        name -> Varchar,
        game -> Uuid,
        archetype -> Int4,
        draw_pile -> Nullable<Array<Nullable<Int4>>>,
        discard_pile -> Nullable<Array<Nullable<Int4>>>,
    }
}

diesel::table! {
    players (id) {
        id -> Uuid,
        name -> Varchar,
        game -> Nullable<Uuid>,
    }
}

diesel::table! {
    tabletops (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(cards -> deck_archetypes (archetype));
diesel::joinable!(decks -> deck_archetypes (archetype));
diesel::joinable!(decks -> tabletops (game));
diesel::joinable!(players -> tabletops (game));

diesel::allow_tables_to_appear_in_same_query!(
    cards,
    deck_archetypes,
    decks,
    players,
    tabletops,
);
