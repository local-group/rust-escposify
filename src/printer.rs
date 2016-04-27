
use consts;

trait Device {
    fn write();
}

struct Printer<D> {
    device: D
}

impl<D: Device> Printer<D> {
    pub fn flush() {
        println!("{}", consts::EOL);
    }
    pub fn hwinit() {}
    pub fn hwselect() {}
    pub fn hwreset() {}
    pub fn print() {}
    pub fn println() {}
    pub fn text() {}
    pub fn line_space() {}
    pub fn feed() {}
    pub fn control() {}
    pub fn align() {}
    pub fn font() {}
    pub fn style() {}
    pub fn size() {}
    pub fn hardware() {}
    pub fn barcode() {}
    pub fn qrcode() {}
    pub fn cashdraw() {}
    pub fn cut() {}
    pub fn bitimage() {}
    pub fn raster() {}
}
