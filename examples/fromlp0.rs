use std::fs::File;
use std::io;

use posify::printer::Printer;

fn main() -> io::Result<()> {
    let device_file = File::options().append(true).open("/dev/usb/lp0").unwrap();

    let file = posify::device::File::from(device_file);
    let mut printer = Printer::new(file, None, None);

    printer
        .chain_size(0, 0)?
        .chain_text("The quick brown fox jumps over the lazy dog")?
        .chain_feed(1)?
        .chain_cut(false)?
        .flush()
}
