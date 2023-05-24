/**
 * ESC/POS Commands (Constants)
 */

/**
 * [`FEED_CONTROL_SEQUENCES` Feed control sequences]
 */
// .FEED_CONTROL_SEQUENCES
pub const CTL_LF: &[u8] = b"\x0a"; // Print and line feed
pub const CTL_FF: &[u8] = b"\x0c"; // Form feed
pub const CTL_CR: &[u8] = b"\x0d"; // Carriage return
pub const CTL_HT: &[u8] = b"\x09"; // Horizontal tab
pub const CTL_VT: &[u8] = b"\x0b"; // Vertical tab

// .LINE_SPACING
pub const LS_DEFAULT: &[u8] = b"\x1b\x32";
pub const LS_SET: &[u8] = b"\x1b\x33";

/**
 * [`CASH_DRAWER` Cash Drawer]
 */
// .CASH_DRAWER
pub const CD_KICK_2: &[u8] = b"\x1b\x70\x00"; // Sends a pulse to pin 2 []
pub const CD_KICK_5: &[u8] = b"\x1b\x70\x01"; // Sends a pulse to pin 5 []

/**
 * [`TEXT_FORMAT` Text format]
 */
// .TEXT_FORMAT
pub const TXT_NORMAL: &[u8] = b"\x1b\x21\x00"; // Normal text
pub const TXT_2HEIGHT: &[u8] = b"\x1b\x21\x10"; // Double height text
pub const TXT_2WIDTH: &[u8] = b"\x1b\x21\x20"; // Double width text

pub const TXT_UNDERL_OFF: &[u8] = b"\x1b\x2d\x00"; // Underline font OFF
pub const TXT_UNDERL_ON: &[u8] = b"\x1b\x2d\x01"; // Underline font 1-dot ON
pub const TXT_UNDERL2_ON: &[u8] = b"\x1b\x2d\x02"; // Underline font 2-dot ON
pub const TXT_BOLD_OFF: &[u8] = b"\x1b\x45\x00"; // Bold font OFF
pub const TXT_BOLD_ON: &[u8] = b"\x1b\x45\x01"; // Bold font ON

pub const TXT_FONT_A: &[u8] = b"\x1b\x4d\x00"; // Font type A
pub const TXT_FONT_B: &[u8] = b"\x1b\x4d\x01"; // Font type B
pub const TXT_FONT_C: &[u8] = b"\x1b\x4d\x02"; // Font type C

pub const TXT_ALIGN_LT: &[u8] = b"\x1b\x61\x00"; // Left justification
pub const TXT_ALIGN_CT: &[u8] = b"\x1b\x61\x01"; // Centering
pub const TXT_ALIGN_RT: &[u8] = b"\x1b\x61\x02"; // Right justification

/**
 * [`BARCODE_FORMAT` Barcode format]
 */
// .BARCODE_FORMAT

pub const BARCODE_FONT_A: &[u8] = b"\x1d\x66\x00"; // Font type A for HRI barcode chars
pub const BARCODE_FONT_B: &[u8] = b"\x1d\x66\x01"; // Font type B for HRI barcode chars


// .CODE2D_FORMAT
pub const TYPE_PDF417: &[u8] = b"\x1dZ\x00"; // = GS + 'Z' + '\x00'
pub const TYPE_DATAMATRIX: &[u8] = b"\x1dZ\x01"; // = GS + 'Z' + '\x01'
pub const TYPE_QR: &[u8] = b"\x1dZ\x02"; // = GS + 'Z' + '\x02'

pub const CODE2D: &[u8] = b"\x1bZ"; // = ESC + 'Z'

pub const QR_LEVEL_L: &[u8] = b"L"; // correct level 7%
pub const QR_LEVEL_M: &[u8] = b"M"; // correct level 15%
pub const QR_LEVEL_Q: &[u8] = b"Q"; // correct level 25%
pub const QR_LEVEL_H: &[u8] = b"H"; // correct level 30%

/**
 * [`IMAGE_FORMAT` Image format]
 */
// .IMAGE_FORMAT
pub const S_RASTER_N: &[u8] = b"\x1d\x76\x30\x00"; // Set raster image normal size
pub const S_RASTER_2W: &[u8] = b"\x1d\x76\x30\x01"; // Set raster image double width
pub const S_RASTER_2H: &[u8] = b"\x1d\x76\x30\x02"; // Set raster image double height
pub const S_RASTER_Q: &[u8] = b"\x1d\x76\x30\x03"; // Set raster image quadruple

// .BITMAP_FORMAT
pub const BITMAP_S8: &[u8] = b"\x1b\x2a\x00"; // 0 : 8 dots single density,102dpi
pub const BITMAP_D8: &[u8] = b"\x1b\x2a\x01"; // 1 : 8 dots double density,203dpi
pub const BITMAP_S24: &[u8] = b"\x1b\x2a\x20"; // 31: 24 dots single density,102dpi
pub const BITMAP_D24: &[u8] = b"\x1b\x2a\x21"; // 32: 24 dots double density,203dpi

// .GSV0_FORMAT
pub const GSV0_NORMAL: &[u8] = b"\x1d\x76\x30\x00";
pub const GSV0_DW: &[u8] = b"\x1d\x76\x30\x01";
pub const GSV0_DH: &[u8] = b"\x1d\x76\x30\x02";
pub const GSV0_DWDH: &[u8] = b"\x1d\x76\x30\x03";
