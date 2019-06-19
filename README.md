# escposify-rs
A ESC/POS driver for Rust

[Documentation](https://docs.rs/escposify)

Most ESC/POS Printers will appear as a file. To print to the device, open a file to the location and pass this to the ```File::from``` function.

To enable this in Windows, install the printer and its driver. Share the printer and specifiy a name for it (Receipt Printer in this case). The printer will then be accessable via ```\\%COMPUTERNAME%\Receipt Printer```.
To test this in the command line:
```
echo "Hello World" > testfile
copy testfile "\\%COMPUTERNAME%\Receipt Printer"
del testfile
```


# Examples

## Rust
See: [simple.rs](examples/simple.rs)

``` rust
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
```
