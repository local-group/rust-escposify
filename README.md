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

use escposify::printer::Printer;
use escposify::device::File;

use tempfile::{NamedTempFileOptions};

fn main() {
    let tempf = NamedTempFileOptions::new()
        .create()
        .unwrap();
    let temp_path = tempf.path().to_str().unwrap().to_owned();

    let file = File::from(temp_path.as_str(), tempf);
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
```
