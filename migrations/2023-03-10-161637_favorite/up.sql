create table fav_msgs (
    id integer primary key not null,
    user_id bigint unsigned not null,
    guild_id bigint unsigned not null,
    channel_id bigint unsigned not null,
    message_id bigint unsigned not null,
    unique(user_id, guild_id, channel_id, message_id)
);

