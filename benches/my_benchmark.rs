use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon::prelude::*;
use std::fs;
use std::io;
use std::path::Path;

fn list_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                list_dirs(&path)?;
            } else {
                // println!("{:?}", path);
            }
        }
    }
    Ok(())
}

fn list_dirs_parallel(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        let entries: Vec<_> = fs::read_dir(dir)?.collect::<Result<Vec<_>, io::Error>>()?;
        entries
            .into_par_iter()
            .try_for_each(|entry| -> io::Result<()> {
                let path = entry.path();
                if path.is_dir() {
                    list_dirs_parallel(&path)?;
                } else {
                    // println!("{:?}", path);
                }
                Ok(())
            })?;
    }
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Disk D", |b| {
        b.iter(|| list_dirs(black_box(Path::new("D:/"))))
    });
    c.bench_function("Disk D parallel", |b| {
        b.iter(|| list_dirs_parallel(black_box(Path::new("D:/"))))
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);
