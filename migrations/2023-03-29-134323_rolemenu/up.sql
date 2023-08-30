create table rolemenu (
    id serial primary key,
    guild_id int8 not null,
    name text not null,
    max_selectable int4,
    roles int8[] not null,
    unique(guild_id, name)
);