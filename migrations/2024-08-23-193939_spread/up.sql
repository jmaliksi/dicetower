CREATE TABLE spread (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tabletop_id INT NOT NULL REFERENCES tabletops(id),
    player_id INT REFERENCES players(id),
    name VARCHAR(40),
    state JSONB NOT NULL,
    private bool NOT NULL
);
