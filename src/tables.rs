use rusqlite::{Connection, Result};
use chrono::{offset::Local, DateTime};

#[derive(Debug)]
struct App {
    id: usize,
    name: String,
}

#[derive(Debug)]
struct TimeEntry {
    id: usize,
    app_id: usize,
    duration: usize,
    date: DateTime<Local>,
}

pub fn get_database() -> Result<Connection>{
    let conn = Connection::open("time.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS App (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS TimeEntry (
             id INTEGER PRIMARY KEY,
             app_id INTEGER NOT NULL,
             duration INTEGER NOT NULL,
             date TEXT NOT NULL,
             FOREIGN KEY (app_id) REFERENCES App(id)
         )",
        (),
    )?;

    Ok(conn)
}
