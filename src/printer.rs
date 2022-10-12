use std::io::{self, Write};
use std::iter;

use byteorder::{LittleEndian, WriteBytesExt};
use encoding::all::UTF_8;
use encoding::types::{EncoderTrap, EncodingRef};

use consts;
use img::Image;

///
/// Allows for printing to a [::device]
///
/// # Example
/// ```rust
///  use escposify::printer::Printer;
///
/// fn main() -> std::io::Result<()> {
///
///     let mut printer = Printer::new(file, None, None);
///
///     printer
///     .chain_size(0,0)?
///     .chain_text("The quick brown fox jumped over the lazy dog")?
///     .chain_feed(1)?
///     .flush()
/// }
/// ```
///
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

    pub fn chain_hwinit(&mut self) -> io::Result<&mut Self> {
        self.hwinit().map(|_| self)
    }
    pub fn hwinit(&mut self) -> io::Result<usize> {
        self.write(consts::HW_INIT)
    }

    pub fn chain_hwselect(&mut self) -> io::Result<&mut Self> {
        self.hwselect().map(|_| self)
    }
    pub fn hwselect(&mut self) -> io::Result<usize> {
        self.write(consts::HW_SELECT)
    }

    pub fn chain_hwreset(&mut self) -> io::Result<&mut Self> {
        self.hwreset().map(|_| self)
    }
    pub fn hwreset(&mut self) -> io::Result<usize> {
        self.write(consts::HW_RESET)
    }

    pub fn chain_print(&mut self, content: &str) -> io::Result<&mut Self> {
        self.print(content).map(|_| self)
    }
    pub fn print(&mut self, content: &str) -> io::Result<usize> {
        // let rv = self.encode(content);
        let rv = self.encode(content)?;
        self.write(rv.as_slice())
    }

    pub fn chain_println(&mut self, content: &str) -> io::Result<&mut Self> {
        self.println(content).map(|_| self)
    }
    pub fn println(&mut self, content: &str) -> io::Result<usize> {
        self.print(format!("{}{}", content, consts::EOL).as_ref())
    }

    pub fn chain_text(&mut self, content: &str) -> io::Result<&mut Self> {
        self.text(content).map(|_| self)
    }
    pub fn text(&mut self, content: &str) -> io::Result<usize> {
        self.println(content)
    }

    pub fn chain_line_space(&mut self, n: i32) -> io::Result<&mut Self> {
        self.line_space(n).map(|_| self)
    }
    pub fn line_space(&mut self, n: i32) -> io::Result<usize> {
        if n >= 0 {
            Ok(self.write(consts::LS_SET)? + self.write_u8(n as u8)?)
        } else {
            self.write(consts::LS_DEFAULT)
        }
    }

    pub fn chain_feed(&mut self, n: usize) -> io::Result<&mut Self> {
        self.feed(n).map(|_| self)
    }
    pub fn feed(&mut self, n: usize) -> io::Result<usize> {
        let n = if n < 1 { 1 } else { n };
        self.write(
            iter::repeat(consts::EOL)
                .take(n)
                .collect::<String>()
                .as_ref(),
        )
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

    pub fn chain_hardware(&mut self, hw: &str) -> io::Result<&mut Self> {
        self.hardware(hw).map(|_| self)
    }
    pub fn hardware(&mut self, hw: &str) -> io::Result<usize> {
        let value = match hw {
            "INIT" => consts::HW_INIT,
            "SELECT" => consts::HW_SELECT,
            "RESET" => consts::HW_RESET,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid hardware command: {}", hw),
                ))
            }
        };
        self.write(value)
    }

    pub fn chain_barcode(
        &mut self,
        code: &str,
        kind: &str,
        position: &str,
        font: &str,
        width: usize,
        height: usize,
    ) -> io::Result<&mut Self> {
        self.barcode(code, kind, position, font, width, height)
            .map(|_| self)
    }
    pub fn barcode(
        &mut self,
        code: &str,
        kind: &str,
        position: &str,
        font: &str,
        width: usize,
        height: usize,
    ) -> io::Result<usize> {
        let mut n = 0;
        if width >= 1 || width <= 255 {
            n += self.write(consts::BARCODE_WIDTH)?;
        }
        if height >= 2 || height <= 6 {
            n += self.write(consts::BARCODE_HEIGHT)?;
        }

        let font = font.to_uppercase();
        let position = position.to_uppercase();
        let kind = kind.to_uppercase().replace("-", "_");
        let font_value = match font.as_ref() {
            "B" => consts::BARCODE_FONT_B,
            // "A" | _ =>
            _ => consts::BARCODE_FONT_A,
        };
        let txt_value = match position.as_ref() {
            "OFF" => consts::BARCODE_TXT_OFF,
            "ABV" => consts::BARCODE_TXT_ABV,
            "BTH" => consts::BARCODE_TXT_BTH,
            // "BLW" | _ =>
            _ => consts::BARCODE_TXT_BLW,
        };
        let kind_value = match kind.as_ref() {
            "UPC_A" => consts::BARCODE_UPC_A,
            "UPC_E" => consts::BARCODE_UPC_E,
            "EAN8" => consts::BARCODE_EAN8,
            "CODE39" => consts::BARCODE_CODE39,
            "ITF" => consts::BARCODE_ITF,
            "NW7" => consts::BARCODE_NW7,
            // "EAN13" | _ =>
            _ => consts::BARCODE_EAN13,
        };
        n += self.write(font_value)?;
        self.write(txt_value)?;
        self.write(kind_value)?;
        self.write(code.as_bytes())?;
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

    pub fn chain_cut(&mut self, part: bool) -> io::Result<&mut Self> {
        self.cut(part).map(|_| self)
    }

    pub fn cut(&mut self, part: bool) -> io::Result<usize> {
        let mut n_bytes = 0;
        n_bytes += self.print(
            iter::repeat(consts::EOL)
                .take(3)
                .collect::<String>()
                .as_ref(),
        )?;
        let paper_cut_type = if part {
            consts::PAPER_PART_CUT
        } else {
            consts::PAPER_FULL_CUT
        };
        n_bytes += self.write(paper_cut_type)?;
        Ok(n_bytes)
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
