use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;
use rocket_db_pools::Database;
use sqlx::{pool::PoolConnection, Postgres};

// pub type DbCon= Connection<Db>;
pub type ConnectionDb = Connection<Db>;
pub type DbCon = PoolConnection<Postgres>;

#[derive(Database)]
#[database("records")]
pub struct Db(sqlx::PgPool);
