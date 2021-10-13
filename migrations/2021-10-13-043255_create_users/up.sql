CREATE TABLE user{
    id SERIAL,
    username VARCHAR,
    email VARCHAR UNIQUE NOT NULL,
    password VARCHAR NOT NULL,
}