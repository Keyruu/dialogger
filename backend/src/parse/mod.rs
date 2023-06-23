use std::{fs::File, io::Read};

use encoding_rs::WINDOWS_1252;

#[derive(Debug)]
pub struct Sub {
    pub id: usize,
    pub start: String,
    pub end: String,
    pub text: String,
}

// write a function that parses a srt file with a windows encoding
pub fn parse_srt(path: &str) -> Vec<Sub> {
    let mut file = File::open(path).unwrap();

    // Read the bytes into a Vec<u8>
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    // Choose the encoding
    let encoding = WINDOWS_1252; // replace this with your custom encoding

    // Decode the bytes into a string
    let (decoded_str, _, _) = encoding.decode(&bytes);
    let mut subs = Vec::new();
    for sub in decoded_str.split("\r\n\r\n") {
        let mut lines = sub.lines();
        let next = lines.next();
        if next.is_none() {
            continue;
        }
        let id = next.unwrap().parse::<usize>().unwrap();
        let times = lines.next().unwrap();
        let mut times = times.split(" --> ");
        let start = times.next().unwrap().to_string();
        let end = times.next().unwrap().to_string();
        let text = lines.collect::<Vec<_>>().join(" ");
        subs.push(Sub {
            id,
            start,
            end,
            text,
        });
    }

    dbg!(&subs);

    subs
}
