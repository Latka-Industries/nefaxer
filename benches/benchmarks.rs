//! End-to-end indexing benchmarks (full pipeline, not component microbenches).

mod common;

use common::{opts_for_root, populate_index_tree, remove_index_file, temp_fixture_root};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use nefaxer::Entry;
use nefaxer::nefax_dir;
use std::hint::black_box;
use std::path::Path;
use std::process::Command;

const FILE_COUNT: usize = 3_000;

fn setup_tree() -> std::path::PathBuf {
    let root = temp_fixture_root("tree");
    let _ = std::fs::remove_dir_all(&root);
    populate_index_tree(&root, FILE_COUNT);
    root
}

fn bench_lib_index(c: &mut Criterion) {
    let root = setup_tree();
    let opts = opts_for_root(&root, false);
    let opts_hash = opts_for_root(&root, true);
    let (existing, _) =
        nefax_dir(&root, &opts, None, None::<fn(&Entry)>).expect("seed index for reindex");

    let mut group = c.benchmark_group("nefax_dir");
    group.sample_size(10);
    group.throughput(Throughput::Elements(FILE_COUNT as u64));

    group.bench_function("fresh", |b| {
        b.iter(|| black_box(nefax_dir(&root, &opts, None, None::<fn(&Entry)>).unwrap()));
    });

    group.bench_function("fresh_with_hash", |b| {
        b.iter(|| black_box(nefax_dir(&root, &opts_hash, None, None::<fn(&Entry)>).unwrap()));
    });

    group.bench_function("reindex_unchanged", |b| {
        b.iter(|| black_box(nefax_dir(&root, &opts, Some(&existing), None::<fn(&Entry)>).unwrap()));
    });

    group.finish();
    let _ = std::fs::remove_dir_all(&root);
}

fn bench_cli_index(c: &mut Criterion) {
    let root = setup_tree();
    let nefaxer = env!("CARGO_BIN_EXE_nefaxer");

    let mut group = c.benchmark_group("cli_index");
    group.sample_size(10);
    group.throughput(Throughput::Elements(FILE_COUNT as u64));

    group.bench_function("fresh", |b| {
        b.iter(|| run_cli_index(nefaxer, &root, false));
    });

    group.bench_function("fresh_with_hash", |b| {
        b.iter(|| run_cli_index(nefaxer, &root, true));
    });

    group.finish();
    let _ = std::fs::remove_dir_all(&root);
}

fn run_cli_index(nefaxer: &str, root: &Path, with_hash: bool) {
    remove_index_file(root);
    let mut cmd = Command::new(nefaxer);
    cmd.arg(root);
    if with_hash {
        cmd.arg("-c");
    }
    let status = cmd.status().expect("run nefaxer CLI");
    assert!(status.success(), "nefaxer exited with {status}");
    black_box(root.join(".nefaxer").exists());
}

criterion_group!(benches, bench_lib_index, bench_cli_index);
criterion_main!(benches);
