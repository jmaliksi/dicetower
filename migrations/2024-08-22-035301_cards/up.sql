CREATE TABLE cards (
    id SERIAL PRIMARY KEY,
    archetype INT REFERENCES deck_archetypes(id),
    name VARCHAR NOT NULL,
    body TEXT,
    image TEXT
)
