/// Top-level declaration of modules, see:
/// https://stackoverflow.com/questions/46829539
/// https://stackoverflow.com/questions/58935890

mod constants {
    pub mod itunesdb_constants;
    pub mod itunesprefs_constants;
    pub mod photo_database_constants;
    pub mod photofolderalbums_constants;
    pub mod playcounts_constants;
    pub mod preferences_constants;
}

mod helpers {
    pub mod helpers;
    pub mod itunesdb_helpers;
}

mod parsers {
    pub mod itunesdb_parser;
    pub mod photo_type_parser;
    pub mod playcounts_parser;
    pub mod preferences_parser;
}

mod itunesdb;
mod itunesprefs;
mod photo_database;
mod preferences;

use std::io::Read;

fn main() {
    let itunesdb_filename: String = std::env::args()
        .nth(1)
        .expect("Missing first parameter: iTunes DB filename");


    let verbose_logging = std::env::args()
        .nth(3);
    let verbose_logging = match verbose_logging {
        Some(str) => str == "-v",
        None => false
    };

    let itunesdb_file_path = std::path::Path::new(&itunesdb_filename);

    if !itunesdb_file_path.exists() {
        panic!(
            "No itunesDB file with that name '{}' exists",
            itunesdb_filename
        );
    }

    let itunesdb_file_length = itunesdb_file_path.metadata().unwrap().len();

    if itunesdb_file_length < 3 {
        panic!(
            "iTunesDB file '{}' has insufficient length ({})",
            itunesdb_filename, itunesdb_file_length
        );
    }

    let mut itunesdb_file_as_bytes = Vec::new();

    let mut itunesdb_file = std::fs::File::open(itunesdb_file_path).unwrap();

    itunesdb_file.read_to_end(&mut itunesdb_file_as_bytes).unwrap();

    let itunesdb_file_type: String = std::env::args()
        .nth(2)
        .expect("Missing second parameter: iTunes DB file type. Supported types are 'photo', 'itunes', 'itprefs', 'playcounts', 'pfalbumbs', and 'preferences'");

    let desired_report_csv_filename = itunesdb_filename.to_string();

    if itunesdb_file_type == "photo" {
        let photos_csv_writer = helpers::helpers::init_csv_writer(&desired_report_csv_filename);
        parsers::photo_type_parser::parse_photo_type_file(
            itunesdb_file_as_bytes,
            photos_csv_writer,
        );
    } else if itunesdb_file_type == "itunes" {
        parsers::itunesdb_parser::parse_itunesdb_file(itunesdb_file_as_bytes, verbose_logging);
    } else if itunesdb_file_type == "itprefs" {
        parsers::preferences_parser::parse_itunes_prefs_file(itunesdb_file_as_bytes);
    } else if itunesdb_file_type == "playcounts" {
        let playcounts_csv_writer = helpers::helpers::init_csv_writer(&desired_report_csv_filename);
        parsers::playcounts_parser::parse_playcounts(itunesdb_file_as_bytes, playcounts_csv_writer);
    } else if itunesdb_file_type == "pfalbums" {
        parsers::photo_type_parser::parse_photofolder_albums_file(itunesdb_file_as_bytes);
    } else if itunesdb_file_type == "preferences" {
        parsers::preferences_parser::parse_preferences_file(itunesdb_file_as_bytes);
    } else {
        println!(
            "'{}' is not a supported iTunesDB file type!",
            itunesdb_file_type
        );
    }
}
