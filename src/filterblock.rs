use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn filterblockalignment(path: &str, block: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut header: Vec<String> = Vec::new();
    let mut seq: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not present");
        if line.starts_with(">") {
            header.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            seq.push(line)
        }
    }

    let blockinner = block.parse::<usize>().unwrap();
    let mut finalvecseq: Vec<Vec<String>> = Vec::new();

    /*
    implementing the block collineraity score here
    */

    for i in 0..seq.len() - 1 {
        let mut score1: usize = 0usize;
        let mut score2: usize = 0usize;

        let mut refinedfilter1: Vec<_> = Vec::new();
        let mut refinedfilter2: Vec<_> = Vec::new();

        for seq in seq[i]
            .chars()
            .map(|x| String::from(x))
            .collect::<Vec<_>>()
            .windows(blockinner)
            .collect::<Vec<_>>()
            .into_iter()
        {
            for seqiter in seq[i + 1]
                .chars()
                .map(|x| String::from(x))
                .collect::<Vec<_>>()
                .windows(blockinner)
                .collect::<Vec<_>>()
                .into_iter()
            {
                for count1 in seq.iter() {
                    for count2 in seqiter.iter() {
                        if count1 == count2 {
                            score1 += 1
                        } else if count1 != count2 {
                            score2 += 1
                        }
                        if score2 < score1 {
                            continue;
                        } else if score2 > score1 {
                            refinedfilter1.push(seq.join("").to_string());
                            refinedfilter2.push(seqiter.join("").to_string());
                        }
                    }
                }
                finalvecseq.push(refinedfilter1.clone());
                finalvecseq.push(refinedfilter2.clone());
            }
        }
    }
    Ok("The block alignment has been filtered and writted".to_string())
}
