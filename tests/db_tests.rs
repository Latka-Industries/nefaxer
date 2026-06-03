//! DB tests: in-memory path_count/load_index, and file-DB complex fixture.

use nefaxer::engine::{load_index, open_db, open_db_in_memory, path_count_from_db};
use std::path::PathBuf;

const INSERT_PATH_SQL: &str =
    "INSERT OR REPLACE INTO paths (path, mtime_ns, size, hash) VALUES (?1, ?2, ?3, ?4)";

#[test]
fn test_in_memory_path_count_and_load_index() {
    let conn = open_db_in_memory().unwrap();
    assert_eq!(path_count_from_db(&conn), Some(0));

    conn.execute(
        INSERT_PATH_SQL,
        rusqlite::params!["a/b", 100_i64, 10_i64, None::<Vec<u8>>],
    )
    .unwrap();
    conn.execute(
        INSERT_PATH_SQL,
        rusqlite::params!["c/d", 200_i64, 20_i64, None::<Vec<u8>>],
    )
    .unwrap();
    assert_eq!(path_count_from_db(&conn), Some(2));

    conn.execute(
        INSERT_PATH_SQL,
        rusqlite::params!["e/f", 300_i64, 30_i64, Some(vec![0u8; 32])],
    )
    .unwrap();
    assert_eq!(path_count_from_db(&conn), Some(3));

    let map = load_index(&conn).unwrap();
    assert_eq!(map.len(), 3);
    assert_eq!(
        map.get(&PathBuf::from("a/b")),
        Some(&(100, 10_u64, None))
    );
    assert_eq!(
        map.get(&PathBuf::from("c/d")),
        Some(&(200, 20_u64, None))
    );
    assert_eq!(
        map.get(&PathBuf::from("e/f")),
        Some(&(300, 30_u64, Some(vec![0u8; 32])))
    );
}

/// Uses tests/fixtures/.nefaxer_complex: real index of this repo (diskinfo wiped).
/// Guards path_count_from_db and load_index on a real-sized fixture.
#[test]
fn test_path_count_and_load_index_complex_fixture() {
    let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(".nefaxer_complex");
    if !db_path.exists() {
        eprintln!(
            "skip: {} not found (copy a .nefaxer here and rename)",
            db_path.display()
        );
        return;
    }
    let conn = open_db(&db_path, None).unwrap();
    let count = path_count_from_db(&conn).expect("COUNT(*) should succeed");
    assert!(count > 0, "complex fixture should have at least one path");

    let map = load_index(&conn).unwrap();
    assert_eq!(map.len(), count, "load_index len should match path count");

    // Sanity: repo fixture should contain these paths
    let expected = ["Cargo.toml", "src/main.rs", "src/lib.rs"];
    for p in &expected {
        assert!(
            map.contains_key(&PathBuf::from(p)),
            "complex fixture should contain {}",
            p
        );
    }
}
