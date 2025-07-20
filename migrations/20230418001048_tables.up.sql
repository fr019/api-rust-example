create table if not exists articles (
    id serial primary key,
    content varchar(255) not null,
    title varchar(255) not null,
    short_description varchar(255) not null,
    is_deleted bool default false
);

create table if not exists users (
    id serial primary key,
    name varchar(255) not null,
    email varchar(255) not null unique,
    password varchar(255) not null,
    is_deleted bool default false,
    created_at timestamptz default now() not null,
    updated_at timestamptz default now() not null
);
