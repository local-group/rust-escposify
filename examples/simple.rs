extern crate escposify;
extern crate tempfile;

use escposify::device::File;
use escposify::printer::Printer;

use tempfile::NamedTempFileOptions;

fn main() {
    let tempf = NamedTempFileOptions::new().create().unwrap();

    let file = File::from(tempf);
    let mut printer = Printer::new(file, None, None);

    let _ = printer
        .font("C")
        .align("lt")
        .style("bu")
        .size(0, 0)
        .text("The quick brown fox jumps over the lazy dog")
        .text("敏捷的棕色狐狸跳过懒狗")
        .barcode("12345678", "EAN8", "", "", 0, 0)
        .feed(1)
        .cut(false)
        .flush();
}
