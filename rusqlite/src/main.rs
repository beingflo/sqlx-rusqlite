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

    let num = 100000;
    let rand = rand::thread_rng().gen_range(1..=num);
    let now = SystemTime::now();
    for i in 0..num {
        conn.execute("INSERT INTO datum (value) VALUES (?1)", &[&(rand + i)])?;
    }
    let elapsed = now.elapsed().unwrap().as_millis();
    println!("Insert: {:?} milis", elapsed);

    let mut acc = 0;
    let now = SystemTime::now();
    for _ in 0..100000 {
        let rand = rand::thread_rng().gen_range(1..=num);
        let datum = conn.query_row(
            "SELECT id, value FROM datum WHERE id = (?1)",
            [rand],
            |row| {
                Ok(Datum {
                    id: row.get(0)?,
                    value: row.get(1)?,
                })
            },
        )?;
        acc += datum.value % 3;
    }
    let elapsed = now.elapsed().unwrap().as_millis();
    println!("Select: {:?} milis", elapsed);
    println!("acc: {:?}", acc);

    Ok(())
}
