create table role_menu (
    id serial primary key,
    guild_id int8 not null,
    name text not null,
    max_selectable int4,
    unique(guild_id, name)
);

create table role_option (
    id serial primary key,
    role_id int8 not null,
    description text,
    emoji text,
    role_menu_id serial references role_menu(id) on delete cascade
);