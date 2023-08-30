use super::model::*;

use crate::db::schema::rolemenu::dsl as db;
use crate::Conn;
use db::rolemenu;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sql_types::Text;

sql_function!(fn lower(s: Text) -> Text);

pub fn find(conn: &mut Conn, guild_id: i64, name: &str) -> Result<Option<RoleMenu>, Error> {
    rolemenu
        .filter(db::guild_id.eq(guild_id))
        .filter(lower(db::name).eq(name.to_lowercase()))
        .first(conn)
        .optional()
}

pub fn _all_guild(conn: &mut Conn, guild_id: i64) -> Result<Vec<RoleMenu>, Error> {
    rolemenu.filter(db::guild_id.eq(guild_id)).get_results(conn)
}

pub fn delete(conn: &mut Conn, guild_id: i64, name: &str) -> Result<usize, Error> {
    diesel::delete(
        rolemenu
            .filter(db::guild_id.eq(guild_id))
            .filter(lower(db::name).eq(name.to_lowercase())),
    )
    .execute(conn)
}

pub fn new(conn: &mut Conn, new: &NewRoleMenu) -> Result<usize, Error> {
    new.insert_into(rolemenu).execute(conn)
}

pub fn comp_rolemenu(conn: &mut Conn, guild_id: i64, partial: &str) -> Result<Vec<String>, Error> {
    let pattern = format!("{}%", partial.to_lowercase());
    rolemenu
        .select(db::name)
        .filter(db::guild_id.eq(guild_id))
        .filter(lower(db::name).like(pattern))
        .get_results(conn)
}

pub fn rename(conn: &mut Conn, guild_id: i64, from: &str, to: &str) -> Result<usize, Error> {
    diesel::update(
        rolemenu
            .filter(db::guild_id.eq(guild_id))
            .filter(lower(db::name).eq(from.to_lowercase())),
    )
    .set(db::name.eq(to))
    .execute(conn)
}
