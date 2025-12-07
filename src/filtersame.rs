use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn filtersiteall(path: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);

    let mut head: Vec<String> = Vec::new();
    let mut seq: Vec<String> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            head.push(line);
        } else if !line.starts_with(">") {
            seq.push(line)
        }
    }

    let mut filteredseq: Vec<String> = Vec::new();

    for i in 0..seq.len() - 1 {
        let seqhold = seq[i].clone();
        let seqholdnext = seq[i + 1].clone();
        let mut seqfilter: Vec<String> = Vec::new();
        for i in seqhold.chars().map(|x| String::from(x)) {
            for j in seqholdnext.chars().map(|x| String::from(x)) {
                if i == j {
                    continue;
                } else if i != j {
                    seqfilter.push(i.clone());
                    filteredseq.push(seqfilter.join("").to_string());
                }
            }
        }
    }

    let mut filewrite = File::create("siteremoval.fasta").expect("file not found");
    for i in 0..filteredseq.len() {
        writeln!(filewrite, ">{:?}\n{:?}", head[i], filteredseq[i]).expect("file not found");
    }

    Ok("The files for the neural network with all the similar sites across all the alignments have been written".to_string())
}
