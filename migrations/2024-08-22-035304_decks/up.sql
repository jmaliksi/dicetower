CREATE TABLE decks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(40) NOT NULL,
    game UUID NOT NULL,
    archetype INT NOT NULL,
    draw_pile INT[],
    discard_pile INT[],
    CONSTRAINT fk_archetype
        FOREIGN KEY(archetype)
        REFERENCES deck_archetypes(id),
    CONSTRAINT fk_game
        FOREIGN KEY(game)
        REFERENCES tabletops(id)
)
