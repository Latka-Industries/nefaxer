//! Criterion benchmarks: hashing, SQLite batch inserts, walkdir vs jwalk.

mod common;

use common::{
    flush_entry_batch, pipeline_context, populate_flat_tree, sample_entries, temp_dir, write_file,
};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use nefaxer::engine::{hash_file, open_db_in_memory};
use nefaxer::pipeline::spawn_walk_thread;
use nefaxer::utils::config::DB_INSERT_BATCH_SIZE;
use std::fs;
use std::hint::black_box;

fn bench_hashing(c: &mut Criterion) {
    let root = temp_dir("hash");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    let small_path = root.join("small.bin");
    let large_path = root.join("large.bin");
    write_file(&small_path, 64 * 1024);
    write_file(&large_path, 128 * 1024 * 1024);

    let mut group = c.benchmark_group("hash_file");
    group.throughput(Throughput::Bytes(64 * 1024));
    group.bench_function("small_64KiB_chunked", |b| {
        b.iter(|| black_box(hash_file(&small_path, 64 * 1024).unwrap()));
    });

    group.throughput(Throughput::Bytes(128 * 1024 * 1024));
    group.bench_function("large_128MiB_mmap", |b| {
        b.iter(|| black_box(hash_file(&large_path, 128 * 1024 * 1024).unwrap()));
    });
    group.finish();

    let _ = fs::remove_dir_all(&root);
}

fn bench_sqlite_inserts(c: &mut Criterion) {
    const TOTAL_ENTRIES: usize = 10_000;
    let entries = sample_entries(TOTAL_ENTRIES, 0);
    let batch_sizes = [DB_INSERT_BATCH_SIZE, 2_000, 5_000];

    let mut group = c.benchmark_group("sqlite_batch_insert");
    group.throughput(Throughput::Elements(TOTAL_ENTRIES as u64));

    for batch_size in batch_sizes {
        group.bench_with_input(
            BenchmarkId::new("entries", batch_size),
            &batch_size,
            |b, &batch_size| {
                let mut conn = open_db_in_memory().unwrap();
                b.iter(|| {
                    conn.execute("DELETE FROM paths", []).unwrap();
                    for chunk in entries.chunks(batch_size) {
                        flush_entry_batch(&mut conn, chunk);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_walk(c: &mut Criterion) {
    const FILE_COUNT: usize = 3_000;
    let root = temp_dir("walk");
    let _ = fs::remove_dir_all(&root);
    populate_flat_tree(&root, FILE_COUNT);

    let mut group = c.benchmark_group("directory_walk");
    group.sample_size(15);
    group.throughput(Throughput::Elements(FILE_COUNT as u64));

    for (name, parallel) in [("walkdir", false), ("jwalk", true)] {
        group.bench_function(name, |b| {
            b.iter(|| {
                let ctx = pipeline_context(&root);
                let (path_tx, path_rx) = crossbeam_channel::bounded(50_000);
                let (path_count_tx, path_count_rx) = crossbeam_channel::bounded(1);
                let handle = spawn_walk_thread(path_tx, path_count_tx, ctx, parallel);
                while path_rx.recv().is_ok() {}
                let count = handle.join().unwrap();
                let _ = path_count_rx.recv();
                black_box(count)
            });
        });
    }
    group.finish();

    let _ = fs::remove_dir_all(&root);
}

criterion_group!(benches, bench_hashing, bench_sqlite_inserts, bench_walk);
criterion_main!(benches);
