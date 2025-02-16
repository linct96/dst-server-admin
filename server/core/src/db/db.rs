// src/db.rs
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

pub type SqlitePool = Pool<SqliteConnectionManager>;

// const DB_PATH: &str = "cats.db";

pub struct DB {
    pub pool: SqlitePool,
    pub conn: PooledConnection<SqliteConnectionManager>,
}

impl DB {
    pub fn new() -> Result<Self, r2d2::Error> {
        let current_dir = std::env::current_dir().unwrap();
        let db_path = current_dir.join("cats.db");
        let pool =
            Pool::new(SqliteConnectionManager::file(db_path)).expect("Failed to create pool.");
        let conn = pool.get()?;
        Ok(Self { pool, conn })
    }
}
