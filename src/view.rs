use colored::Colorize;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn alignment_embedded_common(path: &str) -> Result<String, Box<dyn Error>> {
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    struct Embedded {
        header: String,
        sequence: String,
    }

    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(&fileopen);
    let mut embedded_hold: Vec<Embedded> = Vec::new();
    let mut hold_header: Vec<String> = Vec::new();
    let mut hold_sequence: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            hold_header.push(line);
        } else {
            hold_sequence.push(line);
        }
    }
    for i in 0..hold_header.len() {
        embedded_hold.push(Embedded {
            header: hold_header[i].clone(),
            sequence: hold_sequence[i].clone(),
        })
    }
    let mut finalholdseq_multivector = Vec::new();
    let mut finalholdid_multivector: Vec<String> = Vec::new();
    for i in 0..hold_header.len() {
        let mut intermediatehold = Vec::new();
        for j in hold_sequence[i].chars() {
            intermediatehold.push(j);
        }
        finalholdseq_multivector.push(intermediatehold);
        finalholdid_multivector.push(hold_header[i].clone());
    }

    for i in 0..finalholdseq_multivector.len() {
        for j in 0..finalholdseq_multivector[0].len() {
            if finalholdseq_multivector[i][j].to_string() == "A" {
                print!(
                    "{}",
                    finalholdseq_multivector[i][j]
                        .to_string()
                        .on_yellow()
                        .bold()
                )
            } else if finalholdseq_multivector[i][j].to_string() == "T" {
                print!(
                    "{}",
                    finalholdseq_multivector[i][j].to_string().on_red().bold()
                )
            } else if finalholdseq_multivector[i][j].to_string() == "C" {
                print!(
                    "{}",
                    finalholdseq_multivector[i][j].to_string().on_blue().bold()
                )
            } else if finalholdseq_multivector[i][j].to_string() == "G" {
                print!(
                    "{}",
                    finalholdseq_multivector[i][j].to_string().on_green().bold()
                )
            } else if finalholdseq_multivector[i][j].to_string() == "-" {
                print!(
                    "{}",
                    finalholdseq_multivector[i][j]
                        .to_string()
                        .on_purple()
                        .bold()
                )
            } else {
                continue;
            }
        }
        println!();
    }
    Ok("The alignment has been printed".to_string())
}
