//! escposify - A ESC/POS driver for Rust
//!
//! ## Examples
//!
//! ### sample.rs
//!
//! Write to a temporary file using [device::File].
//! ```rust
//! extern crate escposify;
//! extern crate tempfile;
//!
//! use std::io;
//!
//! use escposify::device::File;
//! use escposify::printer::Printer;
//!
//! use tempfile::NamedTempFileOptions;
//!
//! fn main() -> io::Result<()> {
//!     let tempf = NamedTempFileOptions::new().create().unwrap();
//!
//!     let file = File::from(tempf);
//!     let mut printer = Printer::new(file, None, None);
//!
//!     printer
//!         .chain_font("C")?
//!         .chain_align("lt")?
//!         .chain_style("bu")?
//!         .chain_size(0, 0)?
//!         .chain_text("The quick brown fox jumps over the lazy dog")?
//!         .chain_text("敏捷的棕色狐狸跳过懒狗")?
//!         .chain_barcode("12345678", "EAN8", "", "", 0, 0)?
//!         .chain_feed(1)?
//!         .chain_cut(false)?
//!         .flush()
//! }
//! ```
//!
//! ### Printing to /dev/usb/lp0
//!
//! When writing to a file ensure that `File::options().append(true)` is set otherwise writing is not possible.
//! ```rust
//! use std::fs::File;
//! use std::io;
//! 
//! use escposify::printer::Printer;
//! 
//! fn main() -> io::Result<()> {
//!     let device_file = File::options().append(true).open("/dev/usb/lp0").unwrap();
//! 
//!     let file = escposify::device::File::from(device_file);
//!     let mut printer = Printer::new(file, None, None);
//! 
//!     printer
//!         .chain_size(0,0)?
//!         .chain_text("The quick brown fox jumps over the lazy dog")?
//!         .chain_feed(1)?
//!         .chain_cut(false)?
//!         .flush()
//! }
//! ```
extern crate byteorder;
extern crate encoding;
extern crate image;

#[cfg(feature = "qrcode_builder")]
extern crate qrcode;

pub mod consts;
pub mod device;
pub mod img;
pub mod printer;
