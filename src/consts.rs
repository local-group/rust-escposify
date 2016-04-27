/**
 * ESC/POS Commands (Constants)
 */

pub const EOL: &'static str = "\n";

// const NUL: &'static str = "\x00";
// const LF: &'static str = "\x0a";
// const ESC: &'static str = "\x1b";
// const FS: &'static str = "\x1c";
// const FF: &'static str = "\x0c";
// const GS: &'static str = "\x1d";
// const DLE: &'static str = "\x10";
// const EOT: &'static str = "\x04";

/**
 * [FEED_CONTROL_SEQUENCES Feed control sequences]
 * @type {Object}
 */
  // .FEED_CONTROL_SEQUENCES
pub const CTL_LF: &'static str = "\x0a";   // Print and line feed
pub const CTL_FF: &'static str = "\x0c";   // Form feed
pub const CTL_CR: &'static str = "\x0d";   // Carriage return
pub const CTL_HT: &'static str = "\x09";   // Horizontal tab
pub const CTL_VT: &'static str = "\x0b";   // Vertical tab

  // .LINE_SPACING
pub const LS_DEFAULT: &'static str = "\x1b\x32";
pub const LS_SET: &'static str = "\x1b\x33";

/**
 * [HARDWARE Printer hardware]
 * @type {Object}
 */
  // .HARDWARE
pub const HW_INIT: &'static str = "\x1b\x40"; // Clear data in buffer and reset modes
pub const HW_SELECT: &'static str = "\x1b\x3d\x01";     // Printer select
pub const HW_RESET: &'static str = "\x1b\x3f\x0a\x00"; // Reset printer hardware

/**
 * [CASH_DRAWER Cash Drawer]
 * @type {Object}
 */
  // .CASH_DRAWER
pub const CD_KICK_2: &'static str = "\x1b\x70\x00"; // Sends a pulse to pin 2 []
pub const CD_KICK_5: &'static str = "\x1b\x70\x01"; // Sends a pulse to pin 5 []

/**
 * [PAPER Paper]
 * @type {Object}
 */
  // .PAPER
pub const PAPER_FULL_CUT: &'static str = "\x1d\x56\x00"; // Full cut paper
pub const PAPER_PART_CUT: &'static str = "\x1d\x56\x01"; // Partial cut paper
pub const PAPER_CUT_A: &'static str = "\x1d\x56\x41"; // Partial cut paper
pub const PAPER_CUT_B: &'static str = "\x1d\x56\x42"; // Partial cut paper

/**
 * [TEXT_FORMAT Text format]
 * @type {Object}
 */
  // .TEXT_FORMAT
pub const TXT_NORMAL: &'static str = "\x1b\x21\x00"; // Normal text
pub const TXT_2HEIGHT: &'static str = "\x1b\x21\x10"; // Double height text
pub const TXT_2WIDTH: &'static str = "\x1b\x21\x20"; // Double width text

pub const TXT_UNDERL_OFF: &'static str = "\x1b\x2d\x00"; // Underline font OFF
pub const TXT_UNDERL_ON: &'static str = "\x1b\x2d\x01"; // Underline font 1-dot ON
pub const TXT_UNDERL2_ON: &'static str = "\x1b\x2d\x02"; // Underline font 2-dot ON
pub const TXT_BOLD_OFF: &'static str = "\x1b\x45\x00"; // Bold font OFF
pub const TXT_BOLD_ON: &'static str = "\x1b\x45\x01"; // Bold font ON

pub const TXT_FONT_A: &'static str = "\x1b\x4d\x00"; // Font type A
pub const TXT_FONT_B: &'static str = "\x1b\x4d\x01"; // Font type B
pub const TXT_FONT_C: &'static str = "\x1b\x4d\x02"; // Font type C

pub const TXT_ALIGN_LT: &'static str = "\x1b\x61\x00"; // Left justification
pub const TXT_ALIGN_CT: &'static str = "\x1b\x61\x01"; // Centering
pub const TXT_ALIGN_RT: &'static str = "\x1b\x61\x02"; // Right justification

/**
 * [BARCODE_FORMAT Barcode format]
 * @type {Object}
 */
  // .BARCODE_FORMAT
pub const BARCODE_TXT_OFF: &'static str = "\x1d\x48\x00"; // HRI barcode chars OFF
pub const BARCODE_TXT_ABV: &'static str = "\x1d\x48\x01"; // HRI barcode chars above
pub const BARCODE_TXT_BLW: &'static str = "\x1d\x48\x02"; // HRI barcode chars below
pub const BARCODE_TXT_BTH: &'static str = "\x1d\x48\x03"; // HRI barcode chars both above and below

pub const BARCODE_FONT_A: &'static str = "\x1d\x66\x00"; // Font type A for HRI barcode chars
pub const BARCODE_FONT_B: &'static str = "\x1d\x66\x01"; // Font type B for HRI barcode chars

pub const BARCODE_HEIGHT: &'static str = "\x1d\x68\x64"; // Barcode Height [1-255]
pub const BARCODE_WIDTH: &'static str = "\x1d\x77\x03"; // Barcode Width  [2-6]

pub const BARCODE_UPC_A: &'static str = "\x1d\x6b\x00"; // Barcode type UPC-A
pub const BARCODE_UPC_E: &'static str = "\x1d\x6b\x01"; // Barcode type UPC-E
pub const BARCODE_EAN13: &'static str = "\x1d\x6b\x02"; // Barcode type EAN13
pub const BARCODE_EAN8: &'static str = "\x1d\x6b\x03"; // Barcode type EAN8
pub const BARCODE_CODE39: &'static str = "\x1d\x6b\x04"; // Barcode type CODE39
pub const BARCODE_ITF: &'static str = "\x1d\x6b\x05"; // Barcode type ITF
pub const BARCODE_NW7: &'static str = "\x1d\x6b\x06"; // Barcode type NW7

  // .CODE2D_FORMAT
pub const TYPE_PDF417: &'static str = "\x1dZ\x00"; // = GS + 'Z' + '\x00'
pub const TYPE_DATAMATRIX: &'static str = "\x1dZ\x01"; // = GS + 'Z' + '\x01'
pub const TYPE_QR: &'static str = "\x1dZ\x02";         // = GS + 'Z' + '\x02'

pub const CODE2D: &'static str = "\x1bZ"; // = ESC + 'Z'

pub const QR_LEVEL_L: &'static str = "L"; // correct level 7%
pub const QR_LEVEL_M: &'static str = "M"; // correct level 15%
pub const QR_LEVEL_Q: &'static str = "Q"; // correct level 25%
pub const QR_LEVEL_H: &'static str = "H"; // correct level 30%

/**
 * [IMAGE_FORMAT Image format]
 * @type {Object}
 */
  // .IMAGE_FORMAT
pub const S_RASTER_N: &'static str = "\x1d\x76\x30\x00"; // Set raster image normal size
pub const S_RASTER_2W: &'static str = "\x1d\x76\x30\x01"; // Set raster image double width
pub const S_RASTER_2H: &'static str = "\x1d\x76\x30\x02"; // Set raster image double height
pub const S_RASTER_Q: &'static str = "\x1d\x76\x30\x03"; // Set raster image quadruple

  // .BITMAP_FORMAT
pub const BITMAP_S8: &'static str = "\x1b\x2a\x00";
pub const BITMAP_D8: &'static str = "\x1b\x2a\x01";
pub const BITMAP_S24: &'static str = "\x1b\x2a\x20";
pub const BITMAP_D24: &'static str = "\x1b\x2a\x21";

  // .GSV0_FORMAT
pub const GSV0_NORMAL: &'static str = "\x1d\x76\x30\x00";
pub const GSV0_DW: &'static str = "\x1d\x76\x30\x01";
pub const GSV0_DH: &'static str = "\x1d\x76\x30\x02";
pub const GSV0_DWDH: &'static str = "\x1d\x76\x30\x03";
