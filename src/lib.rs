//! escposify - A ESC/POS driver for Rust
//!
//! ## Examples
//!
//! ### sample.rs
//!
//! Write to a temporary file using [device::File].
//! ```rust
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
//! ```no_run
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
//!
//! ### Writing to the stdout
//!
//! Understandably not all options work here (alignment, fonts, chain_cut etc.)
//! but for quick debugging and prototyping this is a good option
//! as it saves tons of time when working on the logic of your implementation.
//!
//! ```rust
//! use std::io::{self, stdout};
//! use escposify::printer::Printer;
//!
//! fn main() -> io::Result<()> {
//!
//!     let mut printer = Printer::new(stdout(), None, None);
//!
//!     printer
//!         .chain_feed(2)?
//!         .chain_text("The quick brown fox jumps over the lazy dog")?
//!         .chain_text("敏捷的棕色狐狸跳过懒狗")?
//!         .chain_feed(1)?
//!         .flush()
//! }
//! ```
//!
//! ### Printing to a printer via USB
//!
//! ```no_run
//! use std::io;
//! use escposify::printer::Printer;
//! use escposify::device::Usb;
//!
//! fn main() -> io::Result<()> {
//!     let product_id = 0xa700;
//!     let vendor_id = 0x0525;
//!     let usb = Usb::new(vendor_id, product_id)?;
//!
//!     let mut printer = Printer::new(usb, None, None);
//!
//!     printer
//!         .chain_feed(5)?
//!         .chain_font("C")?
//!         .chain_align("lt")?
//!         .chain_style("bu")?
//!         .chain_size(0, 0)?
//!         .chain_text("The quick brown fox jumps over the lazy dog")?
//!         .chain_barcode("12345678", "EAN8", "", "", 0, 0)?
//!         .chain_feed(5)?
//!         .chain_cut(false)?
//!         .flush()
//! }
//! ```

pub mod consts;
pub mod device;
pub mod img;
pub mod printer;
