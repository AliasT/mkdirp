use std::fs::DirBuilder;

pub fn mkdirp(p: String) {
  DirBuilder::new().recursive(true).create(p).unwrap();
}
