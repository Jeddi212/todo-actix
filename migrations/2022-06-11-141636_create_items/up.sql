CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    checked BOOLEAN DEFAULT FALSE,
    list_id int4
)