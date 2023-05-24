extern crate tempfile;

extern crate snbc;

use snbc::device::File;
use snbc::printer::Printer;
use tempfile::NamedTempFileOptions;

#[test]
fn simple() {
    let tempf = NamedTempFileOptions::new().create().unwrap();

    let file = File::from(tempf);
    let mut printer = Printer::new(file, None, None);

    let _ = printer
        .chain_font("C")
        .unwrap()
        .chain_align("lt")
        .unwrap()
        .chain_style("bu")
        .unwrap()
        .chain_size(0, 0)
        .unwrap()
        .chain_text("The quick brown fox jumps over the lazy dog")
        .unwrap()
        .chain_text("敏捷的棕色狐狸跳过懒狗")
        .unwrap()
        .chain_barcode("12345678", "EAN8", "", "", 0, 0)
        .unwrap()
        .chain_feed(1)
        .unwrap()
        .chain_cut(false)
        .unwrap()
        .flush();
}
