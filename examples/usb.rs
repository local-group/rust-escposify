use std::io;

use escposify::device::Usb;
use escposify::printer::Printer;

fn main() -> io::Result<()> {
    let product_id = 0xa700; // find the correct product_id for your printer
    let vendor_id = 0x0525; // find the correct vendor_id for your printer
    let usb = Usb::new(vendor_id, product_id)?;

    let mut printer = Printer::new(usb, None, None);

    printer
        .chain_feed(5)?
        .chain_font("C")?
        .chain_align("lt")?
        .chain_style("bu")?
        .chain_size(0, 0)?
        .chain_text("The quick brown fox jumps over the lazy dog")?
        .chain_barcode("12345678", "EAN8", "", "", 0, 0)?
        .chain_feed(5)?
        .chain_cut(false)?
        .flush()
}
