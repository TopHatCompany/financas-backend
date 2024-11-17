use diesel::{r2d2, PgConnection};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn get_pool(db_url: &str) -> DbPool {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Error building a connection pool")
}
