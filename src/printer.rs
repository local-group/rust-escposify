use std::io;
use std::io::Write;
use std::iter;

use byteorder::{LittleEndian, WriteBytesExt};
use encoding::all::UTF_8;
use encoding::types::{EncoderTrap, EncodingRef};

use consts;
use img::Image;

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

    fn encode(&mut self, content: &str) -> Vec<u8> {
        self.codec.encode(content, self.trap).unwrap()
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    pub fn write_u8(&mut self, n: u8) -> io::Result<usize> {
        self.write(vec![n].as_slice())
    }

    fn write_u16le(&mut self, n: u16) -> io::Result<usize> {
        let mut wtr = vec![];
        wtr.write_u16::<LittleEndian>(n).unwrap();
        self.write(wtr.as_slice())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }

    pub fn hwinit(&mut self) -> io::Result<usize> {
        self.write(consts::HW_INIT)
    }

    pub fn hwselect(&mut self) -> io::Result<usize> {
        self.write(consts::HW_SELECT)
    }

    pub fn hwreset(&mut self) -> io::Result<usize> {
        self.write(consts::HW_RESET)
    }

    pub fn print(&mut self, content: &str) -> io::Result<usize> {
        // let rv = self.encode(content);
        let rv = self.encode(content);
        self.write(rv.as_slice())
    }

    pub fn println(&mut self, content: &str) -> io::Result<usize> {
        self.print(format!("{}{}", content, consts::EOL).as_ref())
    }

    pub fn text(&mut self, content: &str) -> io::Result<usize> {
        self.println(content)
    }

    pub fn line_space(&mut self, n: i32) -> io::Result<usize> {
        if n >= 0 {
            Ok(self.write(consts::LS_SET)? + self.write_u8(n as u8)?)
        } else {
            self.write(consts::LS_DEFAULT)
        }
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

    pub fn control(&mut self, ctrl: &str) -> io::Result<usize> {
        let ctrl_upper = ctrl.to_uppercase();
        let ctrl_value = match ctrl_upper.as_ref() {
            "LF" => consts::CTL_LF,
            "FF" => consts::CTL_FF,
            "CR" => consts::CTL_CR,
            "HT" => consts::CTL_HT,
            "VT" => consts::CTL_VT,
            _ => panic!("Invalid control action: {}", ctrl),
        };
        self.write(ctrl_value)
    }

    pub fn align(&mut self, alignment: &str) -> io::Result<usize> {
        let align_upper = alignment.to_uppercase();
        let align_value = match align_upper.as_ref() {
            "LT" => consts::TXT_ALIGN_LT,
            "CT" => consts::TXT_ALIGN_CT,
            "RT" => consts::TXT_ALIGN_RT,
            _ => panic!("Invalid alignment: {}", alignment),
        };
        self.write(align_value)
    }

    pub fn font(&mut self, family: &str) -> io::Result<usize> {
        let family_upper = family.to_uppercase();
        let family_value = match family_upper.as_ref() {
            "A" => consts::TXT_FONT_A,
            "B" => consts::TXT_FONT_B,
            "C" => consts::TXT_FONT_C,
            _ => panic!("Invalid font family: {}", family),
        };
        self.write(family_value)
    }

    pub fn style(&mut self, kind: &str) -> io::Result<usize> {
        let kind_upper = kind.to_uppercase();
        match kind_upper.as_ref() {
            "B" => Ok(self.write(consts::TXT_UNDERL_OFF)? + self.write(consts::TXT_BOLD_ON)?),
            "U" => Ok(self.write(consts::TXT_BOLD_OFF)? + self.write(consts::TXT_UNDERL_ON)?),
            "U2" => Ok(self.write(consts::TXT_BOLD_OFF)? + self.write(consts::TXT_UNDERL2_ON)?),
            "BU" => Ok(self.write(consts::TXT_BOLD_ON)? + self.write(consts::TXT_UNDERL_ON)?),
            "BU2" => Ok(self.write(consts::TXT_BOLD_ON)? + self.write(consts::TXT_UNDERL2_ON)?),
            "NORMAL" | _ => {
                Ok(self.write(consts::TXT_BOLD_OFF)? + self.write(consts::TXT_UNDERL_OFF)?)
            }
        }
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

    pub fn hardware(&mut self, hw: &str) -> io::Result<usize> {
        let value = match hw {
            "INIT" => consts::HW_INIT,
            "SELECT" => consts::HW_SELECT,
            "RESET" => consts::HW_RESET,
            _ => panic!("Invalid hardware command: {}", hw),
        };
        self.write(value)
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
            "A" | _ => consts::BARCODE_FONT_A,
        };
        let txt_value = match position.as_ref() {
            "OFF" => consts::BARCODE_TXT_OFF,
            "ABV" => consts::BARCODE_TXT_ABV,
            "BTH" => consts::BARCODE_TXT_BTH,
            "BLW" | _ => consts::BARCODE_TXT_BLW,
        };
        let kind_value = match kind.as_ref() {
            "UPC_A" => consts::BARCODE_UPC_A,
            "UPC_E" => consts::BARCODE_UPC_E,
            "EAN8" => consts::BARCODE_EAN8,
            "CODE39" => consts::BARCODE_CODE39,
            "ITF" => consts::BARCODE_ITF,
            "NW7" => consts::BARCODE_NW7,
            "EAN13" | _ => consts::BARCODE_EAN13,
        };
        n += self.write(font_value)?;
        self.write(txt_value)?;
        self.write(kind_value)?;
        self.write(code.as_bytes())?;
        Ok(n)
    }

    #[cfg(feature = "qrcode")]
    pub fn qrimage(&mut self) -> io::Result<usize> {}

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
            "L" | _ => consts::QR_LEVEL_L,
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

    pub fn cashdraw(&mut self, pin: i32) -> io::Result<usize> {
        let pin_value = if pin == 5 {
            consts::CD_KICK_5
        } else {
            consts::CD_KICK_2
        };
        self.write(pin_value)
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

    pub fn bit_image(&mut self, image: &Image, density: Option<&str>) -> io::Result<usize> {
        let density = density.unwrap_or("d24");
        let density_upper = density.to_uppercase();
        let header = match density_upper.as_ref() {
            "S8" => consts::BITMAP_S8,
            "D8" => consts::BITMAP_D8,
            "S24" => consts::BITMAP_S24,
            "D24" | _ => consts::BITMAP_D24,
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

    pub fn raster(&mut self, image: &Image, mode: Option<&str>) -> io::Result<usize> {
        let mode_upper = mode.unwrap_or("NORMAL").to_uppercase();
        let header = match mode_upper.as_ref() {
            "DH" => consts::GSV0_DH,
            "DWDH" => consts::GSV0_DWDH,
            "DW" => consts::GSV0_DW,
            "NORMAL" | _ => consts::GSV0_NORMAL,
        };
        let mut n_bytes = 0;
        n_bytes += self.write(header)?;
        n_bytes += self.write_u16le(((image.width+7)/8) as u16)?;
        n_bytes += self.write_u16le(image.height as u16)?;
        n_bytes += self.write(image.get_raster().as_ref())?;
        Ok(n_bytes)
    }
}
