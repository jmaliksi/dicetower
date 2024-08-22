CREATE TABLE players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(40) NOT NULL,
    game UUID NOT NULL,
    CONSTRAINT fk_tabletop
        FOREIGN KEY(game)
        REFERENCES tabletops(id)
)
