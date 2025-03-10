-- Add up migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    
);

CREATE TABLE tags (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE records (
    id SERIAL PRIMARY KEY,

    title VARCHAR NOT NULL,
    artist VARCHAR NOT NULL,
    release_date DATE NOT NULL,
    cover_url VARCHAR NOT NULL,

    discogs_url VARCHAR NULL,
    spotify_url VARCHAR NULL,

    user_id SERIAL NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);



CREATE TABLE records_tags (
    record_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,

    FOREIGN KEY (record_id) REFERENCES records (id),
    FOREIGN KEY (tag_id) REFERENCES tags (id)
);