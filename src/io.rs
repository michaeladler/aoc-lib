use std::fs;
use std::io;
use std::io::prelude::*;

pub fn read_input(day: i32) -> io::Result<Vec<u8>> {
    let fname = format!("input/day{:02}.txt", day);
    let mut file = fs::File::open(fname)?;
    let mut buf = Vec::with_capacity(4096);
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
