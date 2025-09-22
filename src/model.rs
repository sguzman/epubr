use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize, Default)]
pub enum Verbosity {
    #[default]
    #[value(name = "1")]
    Info,
    #[value(name = "0")]
    Quiet,
    #[value(name = "2")]
    Debug,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BookEntry {
    pub full_path: String,
    pub uri_path: String,
    pub protocol: String, // "file" for now
    pub filename: String,
    pub xxhash: Option<u128>, // XXH3 128-bit
    pub date_found: String,   // ISO 8601
    pub missing: bool,
    pub stale: bool,

    // Metadata
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub chapters: Vec<String>,
    pub publish_date: Option<String>,
    pub publisher: Option<String>,
    pub other_metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpubMeta {
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub chapters: Vec<String>,
    pub publish_date: Option<String>,
    pub publisher: Option<String>,
    pub other_metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BooksDb {
    pub books: Vec<BookEntry>,
    pub last_updated: Option<DateTime<Utc>>,
}
