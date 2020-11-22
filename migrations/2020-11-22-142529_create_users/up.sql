create extension if not exists "uuid-ossp";

create table users(
       id uuid primary key not null default uuid_generate_v1(),
       username varchar(50) not null,
       password_encrypted text not null,
       created_at timestamp with time zone default current_timestamp not null
);

create unique index users_username on users(username);
