
Version 0.3.0
=============

API change
----------
* Printer methods return `io::Result<usize>`
  ``` rust
  // Old version
  pub fn raster(&mut self, image: &Image, mode: Option<&str>) -> &mut Self;

  // New version
  pub fn raster(&mut self, image: &Image, mode: Option<&str>) -> io::Result<usize>;
  ```
* Add related chain API for printer (see: `examples/simple.rs`)
  ``` rust
  // New API
  pub fn chain_raster(&mut self, image: &Image, mode: Option<&str>) -> io::Result<&mut Self>;
  
  // Example
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
  ```

Bug fix
-------
* PR[#6](https://github.com/local-group/rust-escposify/pull/6) : Some bug fixes for image raster
