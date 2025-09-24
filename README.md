# epubr

A fast(ish) command-line indexer for large EPUB libraries.
It scans a directory tree, extracts basic EPUB metadata, computes content hashes (optional), and stores everything in a single JSON database (`books.json`). It also ships with maintenance commands (check/prune/merge/rehash) and clean, color-coded logs.

---

## Why?

If you keep thousands (or millions) of ebooks across disks and machines, you need a durable, mergeable catalog that’s decoupled from any GUI or proprietary tool. `epubr` aims to be:

* **Simple**: one JSON file, readable and versionable.
* **Fast to iterate**: incremental checks, optional hashing, parallel scanning.
* **Composable**: future adapters for search engines/DBs and a tiny web UI.

---

## Status at a glance

### ✅ Implemented

* **Database format**: `books.json` with these fields per book:

  * `full_path` (string)
  * `uri_path` (string; `file://…`)
  * `protocol` (string; currently `"file"`)
  * `filename` (string)
  * `size_bytes` (u64)
  * `xxhash` (u128 or `null`)
  * `date_found` (ISO-8601 string)
  * `missing` (bool)
  * `stale` (bool)
  * **Metadata** (best-effort from OPF):

    * `title` (string or `null`)
    * `author` (string or `null`)
    * `description` (string or `null`)
    * `chapters` (array of strings; currently empty placeholder)
    * `publish_date` (string or `null`)
    * `publisher` (string or `null`)
    * `other_metadata` (object; e.g., language/identifier)
* **Subcommands**

  * `load <DIR>`: scan and add/update entries
    Options: `--follow-symlinks`, `--no-hash`
  * `check`: verify `books.json` against the filesystem (mark `missing`, detect changed content and produce a fresh entry while marking the old one `stale`)
  * `prune`: remove `stale` entries
  * `merge <OTHER_DB>`: merge another `books.json` into the current DB
  * `rehash [--force]`: fill missing hashes (or recompute all with `--force`)
  * `count`: print number of entries
* **Parallelism**: configurable with `-t/--threads`
* **Logging**: centralized, color-coded, timestamped (`log.rs`)

  * `-v/--verbose 0|1|2` (quiet/info/debug)
  * “Inserted …” messages are **debug**; “Updated (stale→new) …” are **info**

### 🚧 In progress / planned

* **Metadata depth**:

  * EPUB2 NCX / EPUB3 `nav.xhtml` chapter extraction (populate `chapters`)
* **Incremental strategy**:

  * Track and leverage file `(mtime, size)` to avoid re-hashing unchanged files
* **Storage backends** (adapters + feature flags):

  * Meilisearch, SurrealDB, PostgreSQL, SQLite, Redis
* **Web UI**:

  * Minimal local dashboard (search/browse)
* **Stow**:

  * `stow`: ZPAQ (or alternative) ultra-compressed archive of selected epubs
* **Query**:

  * Terminal REPL for local querying
* **Serve**:

  * Launch index + adapters + web UI from one command

---

## Install / Build

### Prerequisites

* Rust (stable) and Cargo
* (Optional) Nix + naersk/flake if you’re doing Nix builds

### Cargo

```bash
# in the repository root:
cargo build --release
./target/release/epubr --help
```

### Nix (example)

```bash
# if you have a flake/naersk setup:
nix build .#epubr
./result/bin/epubr --help
```

---

## Usage

Global options:

* `--db <FILE>` (default `books.json`)
* `-t, --threads <N>` (0 = Rayon default)
* `-v, --verbose <0|1|2>` (quiet/info/debug)

### Load (scan a directory)

```bash
# Hash + metadata (default)
epubr -v 1 -t 8 --db books.json load ./library

# On HDDs try fewer threads
epubr -v 1 -t 1 load ./external-hdd/books

# Fast skeleton pass (no hashing)
epubr load --no-hash ./library

# Follow symlinks
epubr load --follow-symlinks ./library
```

### Check & Prune

```bash
# Mark missing files; detect changed content (old → stale+missing, add fresh)
epubr -v 1 check

# Remove stale entries
epubr prune
```

### Merge two DBs

```bash
# Merge OTHER_DB into --db (default: books.json)
epubr merge /path/to/other_books.json
```

Merge policy (current):

* Match on `full_path`:

  * If both have hashes and they’re equal → no-op
  * If hash differs or missing→present → mark existing as `stale+missing`, insert fresh
* If path not present → insert as new

*(Future enhancement: allow strategies like match by hash when paths differ across machines.)*

### Rehash missing (or all)

```bash
# Fill only missing hashes where files still exist
epubr rehash

# Recompute hashes for all non-stale entries (use if you suspect corruption)
epubr rehash --force
```

### Count

```bash
epubr count
# prints "DB entries: N" (log) and "N" to stdout
```

---

## Data model

Minimal example (pretty-printed):

```json
{
  "books": [
    {
      "full_path": "/books/Fiction/Author/Title.epub",
      "uri_path": "file:///books/Fiction/Author/Title.epub",
      "protocol": "file",
      "filename": "Title.epub",
      "size_bytes": 524288,
      "xxhash": "152531415824071388527627642686790332337",
      "date_found": "2025-09-23T08:25:17Z",
      "missing": false,
      "stale": false,
      "title": "Title",
      "author": "Author",
      "description": null,
      "chapters": [],
      "publish_date": "2018-10-01",
      "publisher": "Publisher",
      "other_metadata": {
        "language": "en",
        "identifier": "urn:isbn:…"
      }
    }
  ],
  "last_updated": "2025-09-23T08:25:17Z"
}
```

**Notes**

* `xxhash` may be `null` if you used `--no-hash` or before `rehash`.
* `size_bytes` is always recorded.
* `stale`+`missing`: when content at a path changes, the old record is retained (history), and the new record is added fresh.

---

## Performance tips

* **HDDs**: use fewer threads (`-t 1` or `-t 2`). Seek storms kill throughput.
* **First pass**: `load --no-hash` to populate quickly; run `rehash` later.
* **Narrow scope**: scan smaller subtrees instead of entire disks.
* **Hash buffer**: we already use a large streaming buffer; hashing cost is dominated by file reads, not CPU.

**Planned**: Incremental checks using `(size, mtime)` to avoid re-hashing unchanged files on subsequent runs.

---

## Project layout

```
src/
  args.rs          # clap CLI & subcommands
  commands/
    mod.rs         # dispatcher + shared setup
    common.rs      # shared merge helper
    load.rs        # load <DIR>
    check.rs       # check
    prune.rs       # prune
    merge.rs       # merge <OTHER_DB>
    rehash.rs      # rehash [--force]
    count.rs       # count
  db.rs            # JSON load/save
  hash.rs          # XXH3 streaming
  log.rs           # logging init (colors, timestamps)
  metadata.rs      # EPUB metadata (container.xml -> OPF)
  model.rs         # BookEntry/EpubMeta/BooksDb
  scan.rs          # filesystem walk (WalkDir)
  util.rs          # time/URI helpers
```

---

## Development notes

* **Edition**: Rust 2024
* **Key crates**: `clap`, `rayon`, `walkdir`, `zip`, `roxmltree`, `serde`, `xxhash-rust`, `tracing`, `tracing-subscriber`, `chrono`, `url`
* **Logging**:

  * centralized in `log.rs`
  * uses RFC-3339 UTC timestamps and ANSI color
  * log levels controlled by `-v` (0=quiet, 1=info, 2=debug)
* **Testing strategy** (suggested):

  * Sample fixtures with tiny/medium/large epubs (and malformed ones)
  * Golden `books.json` snapshots
  * Property tests for merge behaviors

---

## Roadmap

* [ ] Chapter extraction (EPUB2 NCX / EPUB3 `nav.xhtml`)
* [ ] Incremental hashing via `(size, mtime)`
* [ ] `stow` archive implementation (ZPAQ or alternative)
* [ ] `serve` adapters (Meilisearch/Surreal/SQLite/Postgres)
* [ ] Minimal web UI + API
* [ ] Config file (toml) to set defaults per machine
* [ ] Strategy flags for merge: `--strategy path|hash|path-or-hash`

---

## Contributing

Issues and PRs welcome. Please:

* keep modules focused and small,
* prefer pure helpers + orchestration in commands,
* avoid breaking the JSON schema (add fields with `#[serde(default)]`),
* write a small note in the README if you tweak the DB semantics.

---

## License

MIT (or your preferred license—fill this section in your repo).

---

## FAQ

**Why JSON instead of a DB?**
Easy to inspect, diff, and merge. Backends are planned via adapters.

**Why XXH3?**
It’s fast and good for change detection while scanning huge trees. If you want cryptographic guarantees, we can add a feature-flagged SHA-256.

**Why is `load` slow on my external HDD?**
Disks are seek-bound; parallelism can make it worse. Use fewer threads, try `--no-hash` on first pass, and rely on `rehash` later.
