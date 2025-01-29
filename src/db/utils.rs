use diesel::{r2d2, Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn get_pool(db_url: &str) -> DbPool {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(db_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Error building a connection pool");

    pool.get()
        .unwrap()
        .run_pending_migrations(MIGRATIONS)
        .unwrap_err();

    pool
}
