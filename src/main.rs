mod environment;
mod events;
mod fonts;
mod markdown;
mod mobs;
mod out;
mod page;
mod sections;
use crate::{out::File, sections::sections};
use mobs::mobs;
use readext::ReadExt;
use sha2::Digest;
use std::{fs, path::PathBuf};

fn main() {
    let output_dir: PathBuf = PathBuf::from(env!("OUTPUT_DIR"));
    let fonts = fonts::ALL.map(|font| File {
        source: out::Source::Font(font),
        target_path: font.output_filename().into(),
    });
    std::fs::create_dir_all(&output_dir).unwrap();
    let index_page = page::index();
    let mob_pages = page::mob_pages();
    let favicon = File {
        target_path: PathBuf::from("favicon.ico"),
        source: out::Source::Bytes(vec![]),
    };
    [
        fonts.as_slice(),
        [index_page].as_slice(),
        mob_pages.as_slice(),
        [favicon].as_slice(),
    ]
    .concat()
    .into_iter()
    .for_each(
        |out::File {
             target_path,
             source,
         }| {
            let output_file_path: PathBuf = output_dir.join(target_path);
            let contents = match source {
                out::Source::Markup(markup) => markup.0.into_bytes(),
                out::Source::Font(font) => {
                    if output_file_path.try_exists().unwrap() {
                        let existing_contents = fs::File::open(&output_file_path)
                            .unwrap()
                            .read_into_vec()
                            .unwrap();
                        let mut hasher = sha2::Sha256::new();
                        hasher.update(&existing_contents);
                        let existing_contents_hash = hasher.finalize();
                        if existing_contents_hash[..] == font.hash[..] {
                            return;
                        }
                    }

                    font.download_and_extract()
                }
                out::Source::Bytes(bytes) => bytes,
            };
            fs::write(output_file_path, contents).unwrap();
        },
    )
}
