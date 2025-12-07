use crate::astruct::Alignment;
use crate::astruct::AlignmentStat;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn alignmentstats(path: &str) -> Result<String, Box<dyn Error>> {
    let alignmentopen = File::open(path).expect("file not found");
    let alignmentread = BufReader::new(alignmentopen);

    let mut header: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();
    let mut mergeseq: Vec<Alignment> = Vec::new();
    for i in alignmentread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            header.push(line);
        } else if !line.starts_with(">") {
            sequence.push(line);
        }
    }
    for i in 0..header.len() {
        mergeseq.push(Alignment {
            head: header[i].clone(),
            seq: sequence[i].clone(),
            length: sequence[i].len(),
        })
    }
    let mut newheader: Vec<AlignmentStat> = Vec::new();
    for i in mergeseq.iter() {
        let headpush: String = i.head.clone();
        let seqpush: String = i.seq.clone();
        let seqchara: Vec<_> = seqpush
            .chars()
            .filter(|x| *x == 'A' || *x != 'T' && *x != 'G' && *x != 'C' && *x != 'N' && *x != '-')
            .collect::<Vec<_>>();
        let seqchart: Vec<_> = seqpush
            .chars()
            .filter(|x| *x == 'T' && *x != 'A' && *x != 'G' && *x != 'C' && *x != 'N' && *x != '-')
            .collect::<Vec<_>>();
        let seqcharg: Vec<_> = seqpush
            .chars()
            .filter(|x| *x == 'G' && *x != 'A' && *x != 'T' && *x != 'C' && *x != 'N' && *x != '-')
            .collect::<Vec<_>>();
        let seqcharc: Vec<_> = seqpush
            .chars()
            .filter(|x| *x == 'C' && *x != 'T' && *x != 'A' && *x != 'G' && *x != 'N' && *x != '-')
            .collect::<Vec<_>>();
        let seqcharn: Vec<_> = seqpush
            .chars()
            .filter(|x| *x == 'N' && *x != 'A' && *x != 'T' && *x == 'G' && *x != 'C' && *x != '-')
            .collect::<Vec<_>>();
        let seqcharalt: Vec<_> = seqpush
            .chars()
            .filter(|x| *x == '-' && *x != 'A' && *x != 'T' && *x != 'G' && *x != 'C' && *x != 'N')
            .collect::<Vec<_>>();
        newheader.push(AlignmentStat {
            name: headpush,
            length: i.seq.len(),
            basea: seqchara.len(),
            baset: seqchart.len(),
            baseg: seqcharg.len(),
            basec: seqcharc.len(),
            basen: seqcharn.len(),
            baseabsent: seqcharalt.len(),
            gccontent: (seqcharg.len() as f32 + seqcharc.len() as f32)
                / (seqchara.len() as f32
                    + seqchart.len() as f32
                    + seqcharg.len() as f32
                    + seqcharc.len() as f32),
        });
    }

    let mut filewrite = File::create("alignment-stats.txt").expect("file not found");
    for i in newheader.iter() {
        writeln!(
            filewrite,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
            i.name,
            i.length,
            i.basea,
            i.baset,
            i.baseg,
            i.basec,
            i.basen,
            i.baseabsent,
            i.gccontent
        )
        .expect("line not found");
    }

    Ok("the alignment stats for the given alignment are as follows".to_string())
}
