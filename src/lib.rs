

extern crate encoding;
extern crate byteorder;
extern crate image;

pub mod consts;
pub mod printer;
pub mod device;


#[cfg(test)]
mod tests {
    use printer::{Printer};
    use device::{File};

    #[test]
    fn simple() {
        let mut printer = Printer::new(File::new("/tmp/test.printer"), None, None);
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
}
