use crate::filesearch::read;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn motifsearchall(path: &str, motif: &str) -> Result<String, Box<dyn Error>> {
    let (header, sequence) = read(path).unwrap();
    let mut returnvec: Vec<(String, usize, usize)> = Vec::new();
    for i in 0..sequence.len() {
        if sequence[i].find(motif) == None {
            continue;
        } else if sequence[i].find(motif) != None {
            let motifstart: usize = sequence[i].find(motif).unwrap();
            let motifend: usize = motifstart + motif.len();
            let motiftuple = (header[i].clone(), motifstart, motifend);
            returnvec.push(motiftuple);
        }
    }

    let mut filewrite = File::create("searched-motif.txt").expect("file not found");
    for i in returnvec.iter() {
        writeln!(filewrite, "{}\t{}\t{}", i.0, i.1, i.2).expect("file not found");
    }

    for i in returnvec.iter() {
        println!("{}\t{}\t{}", i.0, i.1, i.2);
    }

    Ok("The results are as follows:".to_string())
}
