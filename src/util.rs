use chrono::Utc;
use std::path::Path;
use url::Url;

pub fn now_iso8601() -> String {
    Utc::now().to_rfc3339()
}

pub fn file_uri(p: &Path) -> String {
    Url::from_file_path(p)
        .ok()
        .map(|u| u.to_string())
        .unwrap_or_default()
}
