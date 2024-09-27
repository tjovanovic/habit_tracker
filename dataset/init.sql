create table users (
    id integer primary key,
    name varchar,
    password varchar,
    created_at timestamptz default now()
);

create table habits (
    id integer primary key,
    name varchar,
    desired_week_days json,
    completed_week_days json default null,
    habit_type json,
    category varchar,
    priority smallint,
    user_id integer references users(id)
);
