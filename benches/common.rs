//! Fixtures for end-to-end nefaxer benchmarks.

use nefaxer::{NefaxOpts, tuning_for_path};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Nested tree with varied file sizes so hashing paths are exercised when enabled.
pub fn populate_index_tree(root: &Path, file_count: usize) {
    fs::create_dir_all(root).unwrap();
    let sizes = [512_u64, 8 * 1024, 64 * 1024];
    for i in 0..file_count {
        let dir = root
            .join(format!("branch{}", i % 48))
            .join(format!("leaf{}", i % 12));
        fs::create_dir_all(&dir).unwrap();
        let size = sizes[i % sizes.len()];
        write_sized_file(&dir.join(format!("file{i}.bin")), size);
    }
}

fn write_sized_file(path: &Path, size: u64) {
    let mut file = fs::File::create(path).unwrap();
    let chunk = vec![0x5A_u8; 8192];
    let mut remaining = size;
    while remaining > 0 {
        let n = remaining.min(chunk.len() as u64) as usize;
        file.write_all(&chunk[..n]).unwrap();
        remaining -= n as u64;
    }
}

pub fn temp_fixture_root(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "nefaxer-e2e-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos())
    ))
}

pub fn opts_for_root(root: &Path, with_hash: bool) -> NefaxOpts {
    let (n, drive_type, use_parallel_walk) = tuning_for_path(root, None);
    NefaxOpts {
        num_threads: Some(n),
        drive_type: Some(drive_type),
        use_parallel_walk: Some(use_parallel_walk),
        with_hash,
        ..Default::default()
    }
}

pub fn remove_index_file(root: &Path) {
    let _ = fs::remove_file(root.join(".nefaxer"));
}
