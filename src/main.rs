pub mod frontend;
pub mod typing;
pub mod cipher;

use std::env;

fn main() {
    let args = env::args().collect();
    frontend::frontend(args);
}
