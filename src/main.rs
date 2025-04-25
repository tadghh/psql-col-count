use sqlx::{Executor, Pool, Postgres, postgres::PgPoolOptions};
use std::time::Instant;
const RUN_COUNT: i32 = 1601;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres@localhost/col_test")
        .await?;

    println!("Starting {} description column migrations...", RUN_COUNT);
    let start = Instant::now();

    for i in 1..=RUN_COUNT {
        add_description_column(&pool).await?;
        drop_description_column(&pool).await?;

        if i % 100 == 0 {
            println!("Completed {} migrations", i);
        }
    }

    let duration = start.elapsed();
    println!("Completed {} migrations in {:?}", RUN_COUNT, duration);

    Ok(())
}

async fn add_description_column(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    tx.execute("ALTER TABLE users ADD COLUMN description TEXT")
        .await?;

    tx.commit().await?;
    Ok(())
}

async fn drop_description_column(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    tx.execute("ALTER TABLE users DROP COLUMN description")
        .await?;

    tx.commit().await?;
    Ok(())
}
