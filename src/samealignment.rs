use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn dealignment(path: &str) -> Result<String, Box<dyn Error>> {
    let alignmentopen = File::open(path).expect("file not found");
    let alignmentread = BufReader::new(alignmentopen);

    let mut alignmenthash_header: HashSet<String> = HashSet::new();
    let mut alignmenthash_seq: HashSet<String> = HashSet::new();

    for i in alignmentread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            alignmenthash_header.insert(line.replace(">", ""));
        }
        if !line.starts_with(">") {
            alignmenthash_seq.insert(line);
        }
    }

    let writehead: Vec<_> = alignmenthash_header.iter().collect::<Vec<_>>();
    let writeseq: Vec<_> = alignmenthash_seq.iter().collect::<Vec<_>>();

    let mut uniquealignment = File::create("filtered-alignment.fasta").expect("file not found");
    for i in 0..writehead.len() {
        writeln!(uniquealignment, ">{:?}\n{:?}", writehead[i], writeseq[i])
            .expect("line not found");
    }

    Ok(
        "The filtered alignment with the same headers and the sequence check have been remvoed"
            .to_string(),
    )
}
