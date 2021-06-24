CREATE TABLE users (
    id serial not null primary key,
    name varchar not null,
    email varchar not null
);

CREATE TABLE product (
    id serial not null primary key,
    user_id integer not null references users(id),
    name varchar not null,
    price integer not null
);