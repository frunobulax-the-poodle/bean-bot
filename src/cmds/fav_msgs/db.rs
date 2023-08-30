use super::model::*;

use crate::db::schema::fav_msgs::dsl::*;
use crate::Conn;
use diesel::prelude::*;
use diesel::result::Error;

sql_function!(fn random() -> Integer);

pub fn find(conn: &mut Conn, new: &NewFavorite) -> Result<Option<FavoritedMessage>, Error> {
    fav_msgs
        .filter(user_id.eq(new.user_id))
        .filter(guild_id.eq(new.guild_id))
        .filter(channel_id.eq(new.channel_id))
        .filter(message_id.eq(new.message_id))
        .first(conn)
        .optional()
}

pub fn delete_by(conn: &mut Conn, new: &NewFavorite) -> Result<usize, Error> {
    diesel::delete(
        fav_msgs
            .filter(user_id.eq(new.user_id))
            .filter(guild_id.eq(new.guild_id))
            .filter(channel_id.eq(new.channel_id))
            .filter(message_id.eq(new.message_id)),
    )
    .execute(conn)
}

pub fn delete(conn: &mut Conn, del_id: i32) -> Result<usize, Error> {
    diesel::delete(fav_msgs.find(del_id)).execute(conn)
}

pub fn rand(
    conn: &mut Conn,
    user: Option<i64>,
    guild: i64,
) -> Result<Option<FavoritedMessage>, Error> {
    let mut query = fav_msgs.filter(guild_id.eq(guild)).into_boxed();
    if let Some(u) = user {
        query = query.filter(user_id.eq(u))
    }
    // Not the most efficient but will do for now
    query.order(random()).first(conn).optional()
}

pub fn add_fav(conn: &mut Conn, fav: &NewFavorite) -> Result<usize, Error> {
    fav.insert_into(fav_msgs).execute(conn)
}
