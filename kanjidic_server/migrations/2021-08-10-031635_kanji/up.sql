CREATE TABLE kanji (
    id SERIAL PRIMARY KEY,
    literal TEXT NOT NULL,
    accepted_stroke_count INT NOT NULL,
    frequency INT,
    jlpt INT
);