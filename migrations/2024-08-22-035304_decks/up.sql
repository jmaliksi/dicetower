CREATE TABLE decks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    game UUID REFERENCES tabletops(id),
    archetype INT REFERENCES deck_archetypes(id),
    draw_pile INT[],
    discard_pile INT[]
)
