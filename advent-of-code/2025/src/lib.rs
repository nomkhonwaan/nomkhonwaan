use std::{fs, io, io::BufRead, path};

pub fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}
