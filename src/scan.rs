use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

fn is_epub(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .map(|e| e.eq_ignore_ascii_case("epub") || e.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false)
}

pub fn gather_epubs(root: &Path, follow_symlinks: bool) -> Result<Vec<PathBuf>> {
    let mut v = Vec::new();
    let mut wd = WalkDir::new(root).follow_links(follow_symlinks).into_iter();

    while let Some(Ok(entry)) = wd.next() {
        if entry.file_type().is_dir() {
            continue;
        }
        if is_epub(&entry) {
            v.push(entry.into_path());
        }
    }
    Ok(v)
}
