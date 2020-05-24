use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::TimeFreq;

#[derive(Debug)]
pub struct LoadErr;

pub fn load_file<P>(filename: P) -> Result<Vec<TimeFreq>, LoadErr>
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(filename) {
        let mut out = vec![];
        for line in lines {
            if let Ok(l) = line {
                if let Ok(tf) = parse(&format!("{}", l)) {
                    out.push(tf);
                }
            }
        }
        Ok(out)
    } else {
        Err(LoadErr)
    }
}

struct ParseErr;
fn parse(line: &str) -> Result<TimeFreq, ParseErr> {
    let spl: Vec<&str> = line.split(' ').collect();
    if spl.len() < 2 {
        Err(ParseErr)
    } else {
        let time_str = spl[0];
        let freq_str = spl[1];
        match (time_str.parse::<f32>(), freq_str.parse::<f32>()) {
            (Ok(time), Ok(freq)) => Ok(TimeFreq { time, freq }),
            _ => Err(ParseErr),
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
