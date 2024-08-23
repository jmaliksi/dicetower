// @generated automatically by Diesel CLI.

diesel::table! {
    cards (id) {
        id -> Int4,
        archetype_id -> Int4,
        #[max_length = 25]
        name -> Varchar,
        body -> Nullable<Text>,
        image -> Nullable<Text>,
    }
}

diesel::table! {
    deck_archetypes (id) {
        id -> Int4,
        #[max_length = 80]
        name -> Varchar,
    }
}

diesel::table! {
    decks (id) {
        id -> Int4,
        #[max_length = 40]
        name -> Varchar,
        tabletop_id -> Nullable<Int4>,
        archetype_id -> Int4,
        draw_pile -> Nullable<Array<Nullable<Int4>>>,
        discard_pile -> Nullable<Array<Nullable<Int4>>>,
    }
}

diesel::table! {
    players (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 40]
        name -> Varchar,
        tabletop_id -> Int4,
    }
}

diesel::table! {
    spread (id) {
        id -> Int4,
        created_at -> Timestamp,
        tabletop_id -> Int4,
        player_id -> Nullable<Int4>,
        #[max_length = 40]
        name -> Nullable<Varchar>,
        state -> Jsonb,
        private -> Bool,
    }
}

diesel::table! {
    tabletops (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 80]
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
    }
}

diesel::joinable!(cards -> deck_archetypes (archetype_id));
diesel::joinable!(decks -> deck_archetypes (archetype_id));
diesel::joinable!(decks -> tabletops (tabletop_id));
diesel::joinable!(players -> tabletops (tabletop_id));
diesel::joinable!(players -> users (user_id));
diesel::joinable!(spread -> players (player_id));
diesel::joinable!(spread -> tabletops (tabletop_id));
diesel::joinable!(tabletops -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cards,
    deck_archetypes,
    decks,
    players,
    spread,
    tabletops,
    users,
);
