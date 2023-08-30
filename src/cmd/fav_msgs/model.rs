use crate::db::schema::fav_msgs;
use crate::db::schema::fav_msgs::dsl::*;
use crate::Conn;
use diesel::prelude::*;
use diesel::result::Error;

sql_function!(fn random() -> Integer);

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

impl FavoritedMessage {
    pub fn delete_id(conn: &mut Conn, del_id: i32) -> Result<usize, Error> {
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
}

impl NewFavorite {
    pub fn find(&self, conn: &mut Conn) -> Result<Option<FavoritedMessage>, Error> {
        fav_msgs
            .filter(user_id.eq(self.user_id))
            .filter(guild_id.eq(self.guild_id))
            .filter(channel_id.eq(self.channel_id))
            .filter(message_id.eq(self.message_id))
            .first(conn)
            .optional()
    }

    pub fn delete(&self, conn: &mut Conn) -> Result<usize, Error> {
        diesel::delete(
            fav_msgs
                .filter(user_id.eq(self.user_id))
                .filter(guild_id.eq(self.guild_id))
                .filter(channel_id.eq(self.channel_id))
                .filter(message_id.eq(self.message_id)),
        )
        .execute(conn)
    }

    pub fn insert(&self, conn: &mut Conn) -> Result<usize, Error> {
        self.insert_into(fav_msgs).execute(conn)
    }
}
