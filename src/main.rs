///
/// 批量删除目标文件夹中的target目录
/// 为避免误删除，只有同时存在src和target目录时才会删除target目录。
///
use std::env::current_dir;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;

use anyhow::anyhow;
use ignore::{DirEntry, WalkBuilder};

///
/// target目录
///
const TARGET_DIR: &str = "target";

///
/// src目录
///
const SRC_DIR: &str = "src";

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let current = current_dir().unwrap();
    let root = args.get(1).map(|x| PathBuf::from(x)).unwrap_or(current);
    let walk = WalkBuilder::new(root)
        .hidden(true)
        .ignore(false)
        .git_ignore(false)
        .filter_entry(|entry| {
            entry
                .file_type()
                .map_or(false, |file_type| file_type.is_dir())
        })
        .threads(num_cpus::get())
        .build_parallel();
    walk.run(|| {
        Box::new(move |result| match result {
            Ok(entry) => {
                if entry.file_name() == TARGET_DIR {
                    if exists_src_dir(&entry) {
                        remove_target_dir(&entry);
                        ignore::WalkState::Skip
                    } else {
                        ignore::WalkState::Continue
                    }
                } else {
                    ignore::WalkState::Continue
                }
            }
            Err(e) => {
                eprintln!("walk error: {}", e);
                ignore::WalkState::Continue
            }
        })
    });
}

fn exists_src_dir(entry: &DirEntry) -> bool {
    let path = entry.path();
    let parent = path.parent().expect(&format!("get path:[{}] parent directory error!", path.display()));
    let src_dir = parent.join(SRC_DIR);
    src_dir.exists()
}

fn remove_target_dir(entry: &DirEntry) {
    let path = entry.path();
    match std::fs::remove_dir_all(path) {
        Ok(_) => println!("✅  remove target directory: {}", path.display()),
        Err(e) => eprintln!("❌  remove target directory '{}' on error: {}", path.display(), e),
    }
}
