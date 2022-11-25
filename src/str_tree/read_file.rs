use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

pub fn cnt_lines<P>(filename: P) -> io::Result<u32>
where P: AsRef<Path>, {
    let file = io::BufReader::new(File::open(filename).expect("Unable to open file"));
    let mut cnt:u32 = 0;
    
    for _ in file.lines() {
        cnt = cnt + 1;
    }
    
    return Ok(cnt);
}