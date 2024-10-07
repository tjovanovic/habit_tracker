create table users (
    id serial primary key,
    name varchar,
    password varchar,
    created_at timestamptz default now()
);

create table habits (
    id serial primary key,
    name varchar,
    desired_week_days varchar[],
    completed_week_days varchar[],
    habit_type json,
    category varchar,
    priority varchar,
    user_id integer references users(id)
);
