use rusqlite::{Connection, Result};
use chrono::{offset::Local, DateTime};

#[derive(Debug)]
struct App {
    id: usize,
    name: String,
}

struct TimeEntry {
    id: usize,
    app_id: usize,
    duration: usize,
    date: DateTime<Local>,
}

pub fn get_database() -> Result<()>{
    let conn = Connection::open("time.db")?;

    Ok(())
}
