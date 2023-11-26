use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};


pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv::dotenv().ok();
    // Read the database URL from the environment variable
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Create a database connection manager for Diesel
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}