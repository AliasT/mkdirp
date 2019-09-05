use std::fs::DirBuilder;
use std::path::Path;

pub fn mkdirp<P: AsRef<Path>>(p: P) {
  DirBuilder::new().recursive(true).create(p).unwrap();
}
