use rand::Rng;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous},
    SqlitePool,
};
use std::{str::FromStr, time::SystemTime}; // Add this line

#[derive(Debug, sqlx::FromRow)]
struct Datum {
    id: i32,
    value: i32,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let options = SqliteConnectOptions::from_str(
        "sqlite:/Users/florian/Projects/sqlx-rusqlite/sqlx/db.sqlite",
    )?
    .create_if_missing(true)
    .journal_mode(SqliteJournalMode::Wal)
    .synchronous(SqliteSynchronous::Normal);

    let pool = SqlitePool::connect_with(options).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS datum (
            id    INTEGER PRIMARY KEY AUTOINCREMENT,
            value INTEGER NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    let num = 100000;
    let rand = rand::thread_rng().gen_range(1..=num);
    let now = SystemTime::now();
    for i in 0..num {
        sqlx::query("INSERT INTO datum (value) VALUES (?)")
            .bind(rand + i)
            .execute(&pool)
            .await?;
    }
    let elapsed = now.elapsed().unwrap().as_millis();
    println!("Insert: {:?} milis", elapsed);

    let mut acc = 0;
    let now = SystemTime::now();
    for _ in 0..100000 {
        let rand = rand::thread_rng().gen_range(1..=num);
        let datum = sqlx::query_as::<_, Datum>("SELECT id, value FROM datum WHERE id = ?")
            .bind(rand)
            .fetch_one(&pool)
            .await?;
        acc += datum.value % 3;
    }
    let elapsed = now.elapsed().unwrap().as_millis();
    println!("Select: {:?} milis", elapsed);
    println!("acc: {:?}", acc);

    pool.close().await;

    Ok(())
}
