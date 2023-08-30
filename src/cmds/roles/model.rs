use crate::db::{schema::rolemenu, VecNoNulls};
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = rolemenu)]
pub struct RoleMenu {
    pub id: i32,
    pub guild_id: i64,
    pub name: String,
    pub max_selectable: Option<i32>,
    #[diesel(deserialize_as = VecNoNulls<i64>)]
    pub roles: Vec<i64>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = rolemenu)]
pub struct NewRoleMenu {
    pub guild_id: i64,
    pub name: String,
    pub max_selectable: Option<i32>,
    pub roles: Vec<i64>,
}
