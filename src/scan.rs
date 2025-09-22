use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

fn is_epub(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .map(|e| e.eq_ignore_ascii_case("epub"))
        .unwrap_or(false)
}

pub fn gather_epubs(root: &Path, follow_symlinks: bool) -> Result<Vec<PathBuf>> {
    let mut v = Vec::new();
    let mut wd = WalkDir::new(root).into_iter();
    while let Some(Ok(entry)) = wd.next() {
        if entry.file_type().is_dir() {
            continue;
        }
        if is_epub(&entry) {
            v.push(entry.into_path());
        }
        if !follow_symlinks && entry.path_is_symlink() {
            // Skip
        }
    }
    Ok(v)
}
