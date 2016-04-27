
pub mod consts;

use consts::{EOL};

pub fn func() {}


#[cfg(test)]
mod tests {
    use super::func;

    #[test]
    fn it_works() {
        func();
    }
}
