CREATE TABLE cards (
    id SERIAL PRIMARY KEY,
    archetype INT NOT NULL,
    name VARCHAR(25) NOT NULL,
    body TEXT,
    image TEXT,
    CONSTRAINT fk_archetype
        FOREIGN KEY(archetype)
        REFERENCES deck_archetypes(id)
)
