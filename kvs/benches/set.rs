use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kvs::KvStore;
use tempfile::TempDir;

fn criterion_benchmark(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let mut store = KvStore::open(temp_dir.path().to_path_buf()).unwrap();

    c.bench_function("store set", |b| {
        b.iter(|| {
            store
                .set(black_box("key1".to_owned()), black_box("value1".to_owned()))
                .unwrap()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
