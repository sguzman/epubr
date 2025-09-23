use anyhow::Result;
use tracing::info;

use crate::model::BooksDb;

/// Print the number of entries in the current DB.
pub fn cmd_count(db: &BooksDb) -> Result<()> {
    let n = db.books.len();
    info!("DB entries: {}", n);
    // Also print to stdout in case user pipes/greps:
    println!("{}", n);
    Ok(())
}
