use std::fmt::Display;

use counted_array::counted_array;
use michie::memoized;
use readext::ReadExt;

macro_rules! define_fonts {
    (
        $(
            $name:ident: {
                filename_in_archive: $filename_in_archive:literal,
                hash: $hash:literal,
            }
        );*
    ) => {
        #[allow(non_upper_case_globals)]
        pub(crate) mod fonts {$(
            pub(crate) const $name: super::Font = super::Font {
                name: ::std::stringify!($name),
                filename_in_archive: $filename_in_archive,
                hash: hex_literal::hex!($hash),
            };
        )*}
        counted_array!(pub(crate) const ALL: [Font; _] = [$(fonts::$name,)*]);
    }
}

define_fonts! {
    Vollkorn: {
        filename_in_archive: "Vollkorn-VariableFont_wght.ttf",
        hash: "2a8cb6feea966f02d8eb14d2705c461a4ecf1ea6a6ced42f473c82b9b935a867",
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Font {
    pub(crate) name: &'static str,
    pub(crate) filename_in_archive: &'static str,
    pub(crate) hash: [u8; 32],
}

impl Font {
    pub(crate) fn output_filename(&self) -> String {
        format!("{}.ttf", self.name).to_lowercase()
    }
}

impl Font {
    #[memoized(key_expr=(self.name, self.filename_in_archive))]
    pub(crate) fn download_and_extract(&self) -> Vec<u8> {
        let font_url = format!("https://fonts.google.com/download?family={}", self.name);
        let archive = reqwest::blocking::get(font_url).unwrap().bytes().unwrap();
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(archive)).unwrap();
        let mut font_file = archive.by_name(self.filename_in_archive).unwrap();
        font_file.read_into_vec().unwrap()
    }
}

impl Display for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
