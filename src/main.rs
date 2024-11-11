use rayon::prelude::*;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn list_dirs_parallel(dir: &Path, file_count: &Arc<AtomicUsize>) -> io::Result<()> {
    if dir.is_dir() {
        let entries: Vec<_> = fs::read_dir(dir)?.collect::<Result<Vec<_>, io::Error>>()?;
        entries
            .into_par_iter()
            .try_for_each(|entry| -> io::Result<()> {
                let path = entry.path();
                if path.is_dir() {
                    list_dirs_parallel(&path, file_count)?;
                } else {
                    // println!("{:?}", path);
                    file_count.fetch_add(1, Ordering::SeqCst);
                }
                Ok(())
            })?;
    }
    Ok(())
}

fn main() {
    let file_count = Arc::new(AtomicUsize::new(0));
    let path = Path::new("D:/");
    let _ = list_dirs_parallel(path, &file_count);
    println!("Total files: {}", file_count.load(Ordering::SeqCst));
}
