use crate::db::schema::fav_msgs;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = fav_msgs)]
pub struct FavoritedMessage {
    pub id: i32,
    pub user_id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub message_id: i64,
}

#[derive(Insertable)]
#[diesel(table_name = fav_msgs)]
pub struct NewFavorite {
    pub user_id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub message_id: i64,
}
