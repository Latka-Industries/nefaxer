//! Shared fixtures for nefaxer benchmarks.

use nefaxer::Entry;
use nefaxer::pipeline::PipelineContext;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

const INSERT_PATH_SQL: &str =
    "INSERT OR REPLACE INTO paths (path, mtime_ns, size, hash) VALUES (?1, ?2, ?3, ?4)";

pub fn temp_dir(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "nefaxer-bench-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos())
    ))
}

pub fn write_file(path: &Path, size: u64) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = fs::File::create(path).unwrap();
    let chunk = vec![0xAB_u8; 64 * 1024];
    let mut remaining = size;
    while remaining > 0 {
        let n = remaining.min(chunk.len() as u64) as usize;
        file.write_all(&chunk[..n]).unwrap();
        remaining -= n as u64;
    }
}

pub fn populate_flat_tree(root: &Path, file_count: usize) {
    fs::create_dir_all(root).unwrap();
    for i in 0..file_count {
        let dir = root.join(format!("dir{}", i % 32));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(format!("file{i}.txt")), format!("payload{i}")).unwrap();
    }
}

pub fn pipeline_context(root: &Path) -> PipelineContext {
    PipelineContext {
        root: root.to_path_buf(),
        db_canonical: None,
        temp_canonical: None,
        exclude: Vec::new(),
        strict: false,
        follow_links: false,
        first_error: Arc::new(Mutex::new(None)),
        skipped_paths: Arc::new(Mutex::new(Vec::new())),
    }
}

pub fn sample_entries(count: usize, run_id: u64) -> Vec<Entry> {
    (0..count)
        .map(|i| Entry {
            path: PathBuf::from(format!("bench/{run_id}/{i}")),
            mtime_ns: 1_700_000_000_000_000_000 + i as i64,
            size: (i as u64 % 4096) + 1,
            hash: None,
        })
        .collect()
}

/// Mirror production batch inserts (`flush_batch` in `engine/db_ops/indexer.rs`).
pub fn flush_entry_batch(conn: &mut rusqlite::Connection, batch: &[Entry]) {
    let tx = conn.transaction().unwrap();
    let mut stmt = tx.prepare(INSERT_PATH_SQL).unwrap();
    for e in batch {
        stmt.execute((
            e.path.to_string_lossy().as_ref(),
            e.mtime_ns,
            e.size.cast_signed(),
            e.hash.as_ref().map(<[u8; 32]>::as_slice),
        ))
        .unwrap();
    }
    drop(stmt);
    tx.commit().unwrap();
}
