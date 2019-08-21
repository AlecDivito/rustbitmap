use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static TEST_TEXT: &str = "This is just some test text";
pub const TEXT_BITMAP_FILE: &str = "test.bmp";
pub const EMPTY_BITMAP_FILE: &str = "empty.bmp";

pub fn setup() {
    // create a path to the desired file
    let text_bitmap_test = Path::new(TEXT_BITMAP_FILE);
    let empty_file_bitmap = Path::new(EMPTY_BITMAP_FILE);
    let files = vec![text_bitmap_test, empty_file_bitmap];

    for filename in files {
        // open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&filename) {
            Err(why) => panic!(
                "couldn't create {}: {}",
                filename.display(),
                why.description()
            ),
            Ok(file) => file,
        };

        // Write the file
        match file.write_all(TEST_TEXT.as_bytes()) {
            Err(why) => panic!(
                "couldn't write to {}: {}",
                filename.display(),
                why.description()
            ),
            Ok(_) => (),
        };
    }
}

pub fn teardown() {
    // create a path to the desired file
    let text_bitmap_test = Path::new(TEXT_BITMAP_FILE);
    let empty_file_bitmap = Path::new(EMPTY_BITMAP_FILE);
    let files = vec![text_bitmap_test, empty_file_bitmap];

    for file in files {
        match std::fs::remove_file(file) {
            Err(why) => panic!(
                "couldn't delete file {}: {}",
                file.display(),
                why.description()
            ),
            Ok(_) => (),
        }
    }
}
