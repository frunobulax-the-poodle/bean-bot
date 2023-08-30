pub mod schema;

use diesel::{
    backend::Backend,
    deserialize,
    pg::Pg,
    r2d2::{ConnectionManager, Pool, R2D2Connection},
    sql_types::{is_nullable::NotNull, Array, Nullable, SqlType},
    Queryable,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn connect<C: R2D2Connection + 'static>() -> Pool<ConnectionManager<C>> {
    let url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    let manager = ConnectionManager::<C>::new(&url);

    // C::establish(&url).unwrap_or_else(|_| panic!("Error connecting to {}", url))
    Pool::builder()
        .build(manager)
        .unwrap_or_else(|_| panic!("Could not build connection pool to {}", url))
}

pub fn run_pending_migrations<DB: Backend, M: MigrationHarness<DB>>(conn: &mut M) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

#[derive(Debug)]
pub struct VecNoNulls<T>(Vec<T>);

impl<I, O> Queryable<Array<Nullable<I>>, Pg> for VecNoNulls<O>
where
    I: SqlType<IsNull = NotNull>,
    O: deserialize::FromSql<I, Pg>,
{
    type Row = Vec<Option<O>>;

    fn build(vec: Self::Row) -> deserialize::Result<Self> {
        let v = vec.into_iter().flatten().collect();
        Ok(VecNoNulls(v))
    }
}

impl<T> From<VecNoNulls<T>> for Vec<T> {
    fn from(vec: VecNoNulls<T>) -> Self {
        vec.0
    }
}
