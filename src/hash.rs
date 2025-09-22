use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use xxhash_rust::xxh3::Xxh3;

pub fn xxh3_file(path: &Path) -> Result<u128> {
    let f = File::open(path).with_context(|| format!("open for hash: {}", path.display()))?;
    let mut rdr = BufReader::new(f);
    let mut hasher = Xxh3::new();
    let mut buf = vec![0u8; 1 << 16];
    loop {
        let n = rdr.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.digest128())
}
