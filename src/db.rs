use crate::model::BooksDb;
use anyhow::{Context, Result};
use chrono::Utc;
use serde_json::json;
use std::fs;
use std::path::Path;

pub fn load_db<P: AsRef<Path>>(p: P) -> Result<BooksDb> {
    let path = p.as_ref();
    if !path.exists() {
        return Ok(BooksDb::default());
    }
    let data = fs::read_to_string(path)
        .with_context(|| format!("reading DB from {}", path.to_string_lossy()))?;
    let mut db: BooksDb = serde_json::from_str(&data)
        .with_context(|| format!("parsing JSON DB {}", path.to_string_lossy()))?;
    db.last_updated = Some(Utc::now());
    Ok(db)
}

pub fn save_db<P: AsRef<Path>>(p: P, db: &BooksDb) -> Result<()> {
    let mut db = db.clone();
    db.last_updated = Some(Utc::now());
    let s = serde_json::to_string_pretty(&db)?;
    fs::write(p.as_ref(), s)?;
    Ok(())
}
