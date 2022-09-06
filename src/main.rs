mod events;
mod markdown;
mod mobs;
mod out;
mod page;
mod sections;
use crate::sections::sections;
use mobs::mobs;
use std::{fs, path::PathBuf};

fn main() {
    let index_page = page::index();
    let mob_pages = page::mob_pages();
    [[index_page].as_slice(), mob_pages.as_slice()]
        .concat()
        .into_iter()
        .for_each(
            |out::File {
                 target_path,
                 source,
             }| {
                let output_dir_path: PathBuf =
                    std::env::var("OUTPUT_DIR").unwrap().parse().unwrap();
                let output_file_path: PathBuf =
                    [output_dir_path, target_path].into_iter().collect();
                let contents = source.into_string();
                fs::write(output_file_path, contents).unwrap();
            },
        )
}
