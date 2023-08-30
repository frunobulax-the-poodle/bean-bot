create table fav_msgs (
    id serial primary key,
    user_id int8 not null,
    guild_id int8 not null,
    channel_id int8 not null,
    message_id int8 not null,
    unique(user_id, guild_id, channel_id, message_id)
);

