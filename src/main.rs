use std::env;

fn main() {
    if let Some(p) = env::args().nth(1) {
        // directly use current crate
        mkpir::mkdirp(p);
    }
}
