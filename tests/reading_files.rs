extern crate rustybitmap;

use rustybitmap::bitmap::image::BitMap;

mod common;

#[test]
fn fails_when_reading_text_file() {
    common::setup();

    let bitmap = BitMap::read(common::TEXT_BITMAP_FILE);
    assert!(bitmap.is_err());

    common::teardown();
}

#[test]
fn fails_when_reading_big_text_file() {
    common::setup();

    let bitmap = BitMap::read(common::BIG_TEXT_BITMAP_FILE);
    assert!(bitmap.is_err());

    common::teardown();
}

#[test]
fn try_to_create_and_save_file_with_no_pixels() {
    let bitmap = BitMap::new(0, 0);
    assert_eq!(bitmap.save_as("temp.bmp").is_ok(), true);
    std::fs::remove_file("temp.bmp").unwrap();
}
