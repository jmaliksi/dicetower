CREATE TABLE users (
    id SERIAL PRIMARY KEY
);

CREATE TABLE tabletops (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id),
    name VARCHAR(80) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE players (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id),
    name VARCHAR(40) NOT NULL,
    tabletop_id INT NOT NULL REFERENCES tabletops(id)
);

CREATE TABLE deck_archetypes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(80) NOT NULL
);

CREATE TABLE cards (
    id SERIAL PRIMARY KEY,
    archetype_id INT NOT NULL REFERENCES deck_archetypes(id),
    name VARCHAR(25) NOT NULL,
    body TEXT,
    image TEXT
);

CREATE TABLE decks (
    id SERIAL PRIMARY KEY,
    name VARCHAR(40) NOT NULL,
    tabletop_id INT REFERENCES tabletops(id),
    archetype_id INT NOT NULL REFERENCES deck_archetypes(id),
    draw_pile INT[],
    discard_pile INT[]
);
