//!
//! A rust library that can read, write and edit bitmap files.
//!
#![deny(
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
///
/// Read, write and edit bitmaps
///
pub mod bitmap;

pub use bitmap::image::BitMap;
pub use bitmap::rgba::Rgba;
