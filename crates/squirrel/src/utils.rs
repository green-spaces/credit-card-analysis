use std::{fs::File, io::Read, path::Path};

pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> String {
    let mut contents = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut contents).unwrap();
    contents
}
