use std::io::{self, stdout};

use escposify::printer::Printer;

fn main() -> io::Result<()> {
    let mut printer = Printer::new(stdout(), None, None);

    printer
        .chain_feed(2)?
        .chain_text("The quick brown fox jumps over the lazy dog")?
        .chain_text("敏捷的棕色狐狸跳过懒狗")?
        .chain_feed(1)?
        .flush()
}
