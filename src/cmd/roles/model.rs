use crate::db::schema;
use crate::Conn;

use diesel::result::Error;
use diesel::sql_types::Text;
use diesel::{insert_into, prelude::*};
use poise::serenity_prelude::RoleId;
use rm::role_menu;
use ro::role_option;
use schema::role_menu::dsl as rm;
use schema::role_option::dsl as ro;

sql_function!(fn lower(s: Text) -> Text);

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = schema::role_menu)]
pub struct RoleMenu {
    pub id: i32,
    pub guild_id: i64,
    pub name: String,
    pub max_selectable: Option<i32>,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(RoleMenu))]
#[diesel(table_name = schema::role_option)]
pub struct RoleOption {
    pub id: i32,
    pub role_id: i64,
    pub description: Option<String>,
    pub emoji: Option<String>,
    pub role_menu_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::role_menu)]
pub struct NewRoleMenu {
    pub guild_id: i64,
    pub name: String,
    pub max_selectable: Option<i32>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::role_option)]
pub struct NewRoleOption {
    pub role_id: i64,
    pub description: Option<String>,
    pub emoji: Option<String>,
    pub role_menu_id: Option<i32>,
}

impl RoleMenu {
    pub fn find(
        conn: &mut Conn,
        guild_id: i64,
        name: &str,
    ) -> Result<Option<(RoleMenu, Vec<RoleOption>)>, Error> {
        let menu: Option<RoleMenu> = role_menu
            .filter(rm::guild_id.eq(guild_id))
            .filter(lower(rm::name).eq(name.to_lowercase()))
            .first(conn)
            .optional()?;

        let res = if let Some(m) = menu {
            let roles = RoleOption::belonging_to(&m).load(conn)?;
            Some((m, roles))
        } else {
            None
        };

        Ok(res)
    }

    pub fn delete(conn: &mut Conn, guild_id: i64, name: &str) -> Result<usize, Error> {
        diesel::delete(
            role_menu
                .filter(rm::guild_id.eq(guild_id))
                .filter(lower(rm::name).eq(name.to_lowercase())),
        )
        .execute(conn)
    }

    pub fn comp_rolemenu(
        conn: &mut Conn,
        guild_id: i64,
        partial: &str,
    ) -> Result<Vec<String>, Error> {
        let pattern = format!("{}%", partial.to_lowercase());
        role_menu
            .select(rm::name)
            .filter(rm::guild_id.eq(guild_id))
            .filter(lower(rm::name).like(pattern))
            .get_results(conn)
    }

    pub fn rename(conn: &mut Conn, guild_id: i64, from: &str, to: &str) -> Result<usize, Error> {
        diesel::update(
            role_menu
                .filter(rm::guild_id.eq(guild_id))
                .filter(lower(rm::name).eq(from.to_lowercase())),
        )
        .set(rm::name.eq(to))
        .execute(conn)
    }
}

impl NewRoleMenu {
    pub fn insert(&self, conn: &mut Conn, roles: &mut [NewRoleOption]) -> Result<(), Error> {
        conn.transaction(|conn| {
            let menu: RoleMenu = self.insert_into(role_menu).get_result(conn)?;
            roles
                .iter_mut()
                .for_each(|u| u.role_menu_id = Some(menu.id));

            insert_into(role_option).values(&*roles).execute(conn)?;
            Ok(())
        })
    }
}

impl NewRoleOption {
    pub fn blank(role_id: &RoleId) -> Self {
        NewRoleOption {
            role_id: role_id.get() as i64,
            description: None,
            emoji: None,
            role_menu_id: None,
        }
    }
}
