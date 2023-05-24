use std::io::{self, Write};

use byteorder::{LittleEndian, WriteBytesExt};
use encoding::all::UTF_8;
use encoding::types::{EncoderTrap, EncodingRef};

use crate::consts;
use crate::img::Image;

pub enum TextPosition {
    Off = 0x00,
    Above = 0x01,
    Below = 0x02,
    Both = 0x03,
}

pub enum BarcodeType {
    UPCA = 0, // or 65?
    UPCE = 1, // or 66?
    EAN13 = 2, // or 67?
    EAN8 = 3, // or 68?
    CODE39 = 4, // or 69?
    ITF = 5, // or 70?
    Code93 = 72,
    Codabar = 6, // or 71?
    Code128 = 73,
    PDF417 = 10, // or 75?
    QRCode = 11, // or 76?
    Maxicode = 12, // or 77?
    GS1 = 13, // or 78?
}

pub enum Font {
    Standard,
    Compressed,
}

pub struct Barcode {
    pub width: u8, // 2 <= n <= 6
    pub height: u8, // 1 <= n <= 255
    pub font: Font,
    // pub code: &str,
    pub kind: BarcodeType,
    pub position: TextPosition,
}

impl Barcode {
    pub fn set_width(&mut self) -> [u8; 3] {
        if self.width >= 2 && self.width <= 6 {
            return [0x1d, 0x77, self.width];
        }
        [0x1d, 0x77, 0x02] // 2 is the default according to docs
    }

    pub fn set_height(&mut self) -> [u8; 3] {
        [0x1d, 0x68, self.height as u8]
    }

    pub fn set_text_position(&mut self) -> [u8; 3] {
        match self.position {
            TextPosition::Off => [0x1d, 0x48, 0x00],
            TextPosition::Above => [0x1d, 0x48, 0x01],
            TextPosition::Below => [0x1d, 0x48, 0x02],
            TextPosition::Both => [0x1d, 0x48, 0x03],
        }
    }

    pub fn set_font(&mut self) -> [u8; 3] {
        match self.font {
            Font::Compressed => [0x1d, 0x66, 0x01],
            _ => [0x1d, 0x66, 0x00], // Default to standard font
        }
    }

    pub fn set_barcode_type(&mut self) -> [u8; 3] {
        match self.kind {
            BarcodeType::EAN13 => [0x1d, 0x6b, 0x02],
            BarcodeType::Code128 => [0x1d, 0x6b, 0x49],
            _ => [0x1d, 0x6b, 0x02],
        }
    }

}

/// Allows for printing to a [::device]
///
/// # Example
/// ```rust
/// use std::fs::File;
/// use snbc::printer::Printer;
/// use tempfile::NamedTempFileOptions;
///
/// fn main() -> std::io::Result<()> {
///     // TODO: Fix this example as NamedTempFileOptions is out of date
///     let tempf = tempfile::NamedTempFileOptions::new().create().unwrap();
///     let file = File::from(tempf);
///     let mut printer = Printer::new(file, None, None);
///
///     printer
///       .chain_size(0,0)?
///       .chain_text("The quick brown fox jumped over the lazy dog")?
///       .chain_feed(1)?
///       .flush()
/// }
/// ```
pub struct Printer<W: io::Write> {
    writer: io::BufWriter<W>,
    codec: EncodingRef,
    trap: EncoderTrap,
}

impl<W: io::Write> Printer<W> {
    pub fn new(writer: W, codec: Option<EncodingRef>, trap: Option<EncoderTrap>) -> Printer<W> {
        Printer {
            writer: io::BufWriter::new(writer),
            codec: codec.unwrap_or(UTF_8 as EncodingRef),
            trap: trap.unwrap_or(EncoderTrap::Replace),
        }
    }

    fn encode(&mut self, content: &str) -> io::Result<Vec<u8>> {
        self.codec
            .encode(content, self.trap)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    pub fn chain_write_u8(&mut self, n: u8) -> io::Result<&mut Self> {
        self.write_u8(n).map(|_| self)
    }
    pub fn write_u8(&mut self, n: u8) -> io::Result<usize> {
        self.write(vec![n].as_slice())
    }

    fn write_u16le(&mut self, n: u16) -> io::Result<usize> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(n)?;
        self.write(wtr.as_slice())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }

    /// ESC @ - Initialize printer, clear data in print buffer and set print mode
    /// to the default mode when powered on.
    ///
    /// ASCII    ESC   @
    /// Hex      1b   40
    /// Decimal  27   64
    /// Notes:
    ///   - The data in the receive buffer is not cleared
    ///   - The macro definition is not cleared
    ///   - The NV bitmap data is not cleared
    pub fn hwinit(&mut self) -> io::Result<usize> {
        self.write(&[0x1b, 0x40])
    }
    pub fn chain_hwinit(&mut self) -> io::Result<&mut Self> {
        self.hwinit().map(|_| self)
    }

    /// ESC = n - Enable/Disable Printer
    /// Docs describe this as "Select printer to which host computer sends data"
    ///
    /// ASCII    ESC   =  n
    /// Hex      1b   3d  n
    /// Decimal  27   61  n
    /// Range: 0 <= n <= 1
    ///
    /// Meaning of n is as follows:
    ///
    /// | Bit | 1/0 | Hex | Decimal | Function         |
    /// |-----|-----|-----|---------|------------------|
    /// |  0  |  0  |  00 |    0    | Printer disabled |
    /// |  0  |  1  |  01 |    1    | Printer enabled  |
    /// | 1-7 |     |     |         | Undefined        |
    ///
    /// Notes:
    /// When the printer is disabled, it ignores all commands except for
    /// real-time commands (DLE EOT, DLE ENQ, DLE DC4) until it is enabled by
    /// this command.
    ///
    /// Default: n = 1
    pub fn enable(&mut self) -> io::Result<usize> {
        self.write(&[0x1b, 0x3d, 0x01])
    }
    pub fn chain_enable(&mut self) -> io::Result<&mut Self> {
        self.enable().map(|_| self)
    }

    pub fn disable(&mut self) -> io::Result<usize> {
        self.write(&[0x1b, 0x3d, 0x00])
    }
    pub fn chain_disable(&mut self) -> io::Result<&mut Self> {
        self.disable().map(|_| self)
    }

    // TODO: There doesn't seem to be a hwreset command for snbc
    // pub fn hwreset(&mut self) -> io::Result<usize> {
    //     self.write(consts::HW_RESET)
    // }
    // pub fn chain_hwreset(&mut self) -> io::Result<&mut Self> {
    //     self.hwreset().map(|_| self)
    // }

    pub fn print(&mut self, content: &str) -> io::Result<usize> {
        // let rv = self.encode(content);
        let rv = self.encode(content)?;
        self.write(rv.as_slice())
    }
    pub fn chain_print(&mut self, content: &str) -> io::Result<&mut Self> {
        self.print(content).map(|_| self)
    }

    pub fn println(&mut self, content: &str) -> io::Result<usize> {
        self.print(format!("{}{}", content, "\n").as_ref())
    }
    pub fn chain_println(&mut self, content: &str) -> io::Result<&mut Self> {
        self.println(content).map(|_| self)
    }

    // TODO: This seems useless? just use print/println?
    pub fn text(&mut self, content: &str) -> io::Result<usize> {
        self.println(content)
    }
    pub fn chain_text(&mut self, content: &str) -> io::Result<&mut Self> {
        self.text(content).map(|_| self)
    }

    /// ESC 2/ESC 3 n - Set line spacing
    ///
    /// ESC 2 (0x1b, 0x32) Sets line spacing to default
    /// ESC 3 (0x1b, 0x33, n) Specifies a specific line spacing
    ///
    /// ASCII    ESC   2
    /// Hex      1b   32
    /// Decimal  27   50
    ///
    /// ASCII    ESC   3  n
    /// Hex      1b   33  n
    /// Decimal  27   51  n
    /// Range: 0 <= n <= 255
    ///
    /// Notes:
    ///   - The line spacing can be set independently in standard mode and in
    ///     page mode.
    ///   - The horizontal and vertical motion units are specified by GS P.
    ///     Changing the horizontal or vertical motion unit does not affect the
    ///     current line spacing.
    ///   - In standard mode, the vertical motion unit (y) is used.
    ///   - In page mode, this command functions as follows, depending on the
    ///     direction and starting position of the printable area:
    ///     1) When the starting position is set to the upper left or lower right
    ///        of the printable area by ESC T, the vertical motion unit (y) is
    ///        used.
    ///     2) When the starting position is set to the upper right or lower left
    ///        of the printable area by ESC T, the horizontal motion unit (x) is
    ///        used.
    ///   - The maximum paper feed amount is 1016 mm (40 inches). Even if a paper
    ///     feed amount of more than 1016 mm (40 inches) is set, the printer
    ///     feeds the paper only 1016 mm (40 inches).
    ///
    /// Default: The default line spacing is approximately 4.23mm (1/6 inches).
    pub fn line_space(&mut self, n: i32) -> io::Result<usize> {
        if n >= 0 && n <= 255 {
            Ok(self.write(&[0x1b, 0x33, n as u8])?)
        } else {
            self.write(&[0x1b, 0x32])
        }
    }
    pub fn chain_line_space(&mut self, n: i32) -> io::Result<&mut Self> {
        self.line_space(n).map(|_| self)
    }

    pub fn feed(&mut self, n: usize) -> io::Result<usize> {
        let n = if n < 1 { 1 } else { n };
        self.write("\n".repeat(n).as_ref())
    }
    pub fn chain_feed(&mut self, n: usize) -> io::Result<&mut Self> {
        self.feed(n).map(|_| self)
    }

    pub fn chain_control(&mut self, ctrl: &str) -> io::Result<&mut Self> {
        self.control(ctrl).map(|_| self)
    }
    pub fn control(&mut self, ctrl: &str) -> io::Result<usize> {
        let ctrl_upper = ctrl.to_uppercase();
        let ctrl_value = match ctrl_upper.as_ref() {
            "LF" => consts::CTL_LF,
            "FF" => consts::CTL_FF,
            "CR" => consts::CTL_CR,
            "HT" => consts::CTL_HT,
            "VT" => consts::CTL_VT,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid control action: {}", ctrl),
                ))
            }
        };
        self.write(ctrl_value)
    }

    pub fn chain_align(&mut self, alignment: &str) -> io::Result<&mut Self> {
        self.align(alignment).map(|_| self)
    }
    pub fn align(&mut self, alignment: &str) -> io::Result<usize> {
        let align_upper = alignment.to_uppercase();
        let align_value = match align_upper.as_ref() {
            "LT" => consts::TXT_ALIGN_LT,
            "CT" => consts::TXT_ALIGN_CT,
            "RT" => consts::TXT_ALIGN_RT,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid alignment: {}", alignment),
                ))
            }
        };
        self.write(align_value)
    }

    pub fn chain_font(&mut self, family: &str) -> io::Result<&mut Self> {
        self.font(family).map(|_| self)
    }
    pub fn font(&mut self, family: &str) -> io::Result<usize> {
        let family_upper = family.to_uppercase();
        let family_value = match family_upper.as_ref() {
            "A" => consts::TXT_FONT_A,
            "B" => consts::TXT_FONT_B,
            "C" => consts::TXT_FONT_C,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid font family: {}", family),
                ))
            }
        };
        self.write(family_value)
    }

    pub fn chain_style(&mut self, kind: &str) -> io::Result<&mut Self> {
        self.style(kind).map(|_| self)
    }
    pub fn style(&mut self, kind: &str) -> io::Result<usize> {
        let kind_upper = kind.to_uppercase();
        match kind_upper.as_ref() {
            "B" => Ok(self.write(consts::TXT_UNDERL_OFF)? + self.write(consts::TXT_BOLD_ON)?),
            "U" => Ok(self.write(consts::TXT_BOLD_OFF)? + self.write(consts::TXT_UNDERL_ON)?),
            "U2" => Ok(self.write(consts::TXT_BOLD_OFF)? + self.write(consts::TXT_UNDERL2_ON)?),
            "BU" => Ok(self.write(consts::TXT_BOLD_ON)? + self.write(consts::TXT_UNDERL_ON)?),
            "BU2" => Ok(self.write(consts::TXT_BOLD_ON)? + self.write(consts::TXT_UNDERL2_ON)?),
            // "NORMAL" | _ =>
            _ => Ok(self.write(consts::TXT_BOLD_OFF)? + self.write(consts::TXT_UNDERL_OFF)?),
        }
    }

    pub fn chain_size(&mut self, width: usize, height: usize) -> io::Result<&mut Self> {
        self.size(width, height).map(|_| self)
    }
    pub fn size(&mut self, width: usize, height: usize) -> io::Result<usize> {
        let mut n = self.write(consts::TXT_NORMAL)?;
        if width == 2 {
            n += self.write(consts::TXT_2WIDTH)?;
        }
        if height == 2 {
            n += self.write(consts::TXT_2HEIGHT)?;
        }
        Ok(n)
    }

    // TODO: I don't think we need this, maybe just write a better function?
    // pub fn chain_hardware(&mut self, hw: &str) -> io::Result<&mut Self> {
    //     self.hardware(hw).map(|_| self)
    // }
    // pub fn hardware(&mut self, hw: &str) -> io::Result<usize> {
    //     let value = match hw {
    //         "INIT" => consts::HW_INIT,
    //         "SELECT" => consts::HW_SELECT,
    //         "RESET" => consts::HW_RESET,
    //         _ => {
    //             return Err(io::Error::new(
    //                 io::ErrorKind::InvalidData,
    //                 format!("Invalid hardware command: {}", hw),
    //             ))
    //         }
    //     };
    //     self.write(value)
    // }

    pub fn chain_barcode(
        &mut self,
        code: &str,
        kind: BarcodeType,
        position: TextPosition,
        font: Font,
        width: u8,
        height: u8,
    ) -> io::Result<&mut Self> {
        self.barcode(code, kind, position, font, width, height)
            .map(|_| self)
    }
    pub fn barcode(
        &mut self,
        code: &str,
        kind: BarcodeType,
        position: TextPosition,
        font: Font,
        width: u8,
        height: u8,
    ) -> io::Result<usize> {
        let mut n = 0;
        let mut bc = Barcode{
            width,
            height,
            position,
            font,
            kind,
        };
        n += self.write(&bc.set_width())?;
        n += self.write(&bc.set_height())?;
        n += self.write(&bc.set_text_position())?;
        n += self.write(&bc.set_font())?;
        n += self.write(&bc.set_barcode_type())?;
        // This was the example in the docs, doesn't seem to actually print a Code128 barcode
        // self.write(&[0x0a_u8, 0x7b_u8, 0x42_u8, 0x4e_u8, 0x6f_u8, 0x2e_u8, 0x7b_u8, 0x43_u8, 0x0c_u8, 0x22_u8, 0x38_u8])?;

        self.write(code.as_bytes())?;
        self.write(&[0x00_u8])?; // Need to send NULL to finish
        Ok(n)
    }

    #[cfg(feature = "qrcode")]
    pub fn chain_qrimage(&mut self) -> io::Result<&mut Self> {
        self.qrimage().map(|_| self)
    }
    #[cfg(feature = "qrcode")]
    pub fn qrimage(&mut self) -> io::Result<usize> {
        Ok(0)
    }

    #[cfg(feature = "qrcode")]
    pub fn chain_qrcode(
        &mut self,
        code: &str,
        version: Option<i32>,
        level: &str,
        size: Option<i32>,
    ) -> io::Result<&mut Self> {
        self.qrcode(code, version, level, size).map(|_| self)
    }
    #[cfg(feature = "qrcode")]
    pub fn qrcode(
        &mut self,
        code: &str,
        version: Option<i32>,
        level: &str,
        size: Option<i32>,
    ) -> io::Result<usize> {
        let level = level.to_uppercase();
        let level_value = match level.as_ref() {
            "M" => consts::QR_LEVEL_M,
            "Q" => consts::QR_LEVEL_Q,
            "H" => consts::QR_LEVEL_H,
            // "L" | _ =>
            _ => consts::QR_LEVEL_L,
        };
        let mut n = 0;
        n += self.write(consts::TYPE_QR)?;
        n += self.write(consts::CODE2D)?;
        n += self.write_u8(version.unwrap_or(3) as u8)?;
        n += self.write(level_value)?;
        n += self.write_u8(size.unwrap_or(3) as u8)?;
        n += self.write_u16le(code.len() as u16)?;
        n += self.write(code.as_bytes())?;
        Ok(n)
    }

    pub fn chain_cashdraw(&mut self, pin: i32) -> io::Result<&mut Self> {
        self.cashdraw(pin).map(|_| self)
    }
    pub fn cashdraw(&mut self, pin: i32) -> io::Result<usize> {
        let pin_value = if pin == 5 {
            consts::CD_KICK_5
        } else {
            consts::CD_KICK_2
        };
        self.write(pin_value)
    }

    pub fn chain_full_cut(&mut self) -> io::Result<&mut Self> {
        self.full_cut().map(|_| self)
    }

    pub fn full_cut(&mut self) -> io::Result<usize> {
        self.write(&[0x0a, 0x0a, 0x0a, 0x1d, 0x56, 0x00])
    }

    pub fn chain_partial_cut(&mut self) -> io::Result<&mut Self> {
        self.partial_cut().map(|_| self)
    }

    pub fn partial_cut(&mut self) -> io::Result<usize> {
        self.write(&[0x0a, 0x0a, 0x0a, 0x1d, 0x56, 0x01])
    }

    pub fn chain_bit_image(
        &mut self,
        image: &Image,
        density: Option<&str>,
    ) -> io::Result<&mut Self> {
        self.bit_image(image, density).map(|_| self)
    }
    pub fn bit_image(&mut self, image: &Image, density: Option<&str>) -> io::Result<usize> {
        let density = density.unwrap_or("d24");
        let density_upper = density.to_uppercase();
        let header = match density_upper.as_ref() {
            "S8" => consts::BITMAP_S8,
            "D8" => consts::BITMAP_D8,
            "S24" => consts::BITMAP_S24,
            // "D24" | _ =>
            _ => consts::BITMAP_D24,
        };
        let n = if density == "s8" || density == "d8" {
            1
        } else {
            3
        };
        let mut n_bytes = 0;
        n_bytes += self.line_space(0)?;
        for line in image.bitimage_lines(n * 8) {
            n_bytes += self.write(header)?;
            n_bytes += self.write_u16le((line.len() / n as usize) as u16)?;
            n_bytes += self.write(line.as_ref())?;
            n_bytes += self.feed(1)?;
        }
        Ok(n_bytes)
    }

    pub fn chain_raster(&mut self, image: &Image, mode: Option<&str>) -> io::Result<&mut Self> {
        self.raster(image, mode).map(|_| self)
    }
    pub fn raster(&mut self, image: &Image, mode: Option<&str>) -> io::Result<usize> {
        let mode_upper = mode.unwrap_or("NORMAL").to_uppercase();
        let header = match mode_upper.as_ref() {
            "DH" => consts::GSV0_DH,
            "DWDH" => consts::GSV0_DWDH,
            "DW" => consts::GSV0_DW,
            // "NORMAL" | _ =>
            _ => consts::GSV0_NORMAL,
        };
        let mut n_bytes = 0;
        n_bytes += self.write(header)?;
        n_bytes += self.write_u16le(((image.width + 7) / 8) as u16)?;
        n_bytes += self.write_u16le(image.height as u16)?;
        n_bytes += self.write(image.get_raster().as_ref())?;
        Ok(n_bytes)
    }
}
