use rand::Rng;
use std::time::SystemTime; // Add this line

use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Datum {
    id: i32,
    value: i32,
}

fn main() -> Result<()> {
    let conn = Connection::open("./db.sqlite")?;

    conn.pragma_update_and_check(None, "journal_mode", &"WAL", |_| Ok(()))
        .unwrap();
    conn.pragma_update(None, "synchronous", &"NORMAL").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS datum (
            id    INTEGER PRIMARY KEY AUTOINCREMENT,
            value INTEGER NOT NULL
        )",
        (),
    )?;

    let mut stmt = conn.prepare("INSERT INTO datum (value) VALUES (?1)")?;
    let num = 100000;
    let rand = rand::thread_rng().gen_range(1..=num);
    let now = SystemTime::now();
    for i in 0..num {
        stmt.execute(&[&(rand + i)])?;
    }
    let elapsed = now.elapsed().unwrap().as_millis();
    println!("Insert: {:?} milis", elapsed);

    let mut stmt = conn.prepare("SELECT id, value FROM datum WHERE id = (?1)")?;

    let mut acc = 0;
    let now = SystemTime::now();
    for _ in 0..100000 {
        let rand = rand::thread_rng().gen_range(1..=num);
        let data = stmt.query_map([rand], |row| {
            Ok(Datum {
                id: row.get(0)?,
                value: row.get(1)?,
            })
        })?;
        for datum in data {
            let datum = datum.unwrap();
            acc += datum.value % 3;
        }
    }
    let elapsed = now.elapsed().unwrap().as_millis();
    println!("Select: {:?} milis", elapsed);
    println!("acc: {:?}", acc);

    Ok(())
}
