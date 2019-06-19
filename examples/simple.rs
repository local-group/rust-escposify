extern crate escposify;
extern crate tempfile;

use std::io;

use escposify::device::File;
use escposify::printer::Printer;

use tempfile::NamedTempFileOptions;

fn main() -> io::Result<()> {
    let tempf = NamedTempFileOptions::new().create().unwrap();

    let file = File::from(tempf);
    let mut printer = Printer::new(file, None, None);

    printer
        .chain_font("C")?
        .chain_align("lt")?
        .chain_style("bu")?
        .chain_size(0, 0)?
        .chain_text("The quick brown fox jumps over the lazy dog")?
        .chain_text("敏捷的棕色狐狸跳过懒狗")?
        .chain_barcode("12345678", "EAN8", "", "", 0, 0)?
        .chain_feed(1)?
        .chain_cut(false)?
        .flush()
}
