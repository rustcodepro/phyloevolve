use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn proteomestats(path: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut header: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();
    let _proteinarr: Vec<String> = vec![
        "ALA".to_string(),
        "ARG".to_string(),
        "ASN".to_string(),
        "ASP".to_string(),
        "CYS".to_string(),
        "GLU".to_string(),
        "GLN".to_string(),
        "GLY".to_string(),
        "HIS".to_string(),
        "ILE".to_string(),
        "LEU".to_string(),
        "LYS".to_string(),
        "MET".to_string(),
        "PHE".to_string(),
        "PRO".to_string(),
        "SER".to_string(),
        "THR".to_string(),
        "TRP".to_string(),
        "TYR".to_string(),
        "VAL".to_string(),
    ];
    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            header.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            sequence.push(line);
        }
    }

    let mut finalcount: Vec<Vec<(String, usize)>> = Vec::new();
    for i in sequence.iter() {
        let mut ausize: usize = 0usize;
        let mut rusize: usize = 0usize;
        let mut nusize: usize = 0usize;
        let mut dusize: usize = 0usize;
        let mut cusize: usize = 0usize;
        let mut eusize: usize = 0usize;
        let mut qusize: usize = 0usize;
        let mut gusize: usize = 0usize;
        let mut husize: usize = 0usize;
        let mut iusize: usize = 0usize;
        let mut lusize: usize = 0usize;
        let mut kusize: usize = 0usize;
        let mut musize: usize = 0usize;
        let mut fusize: usize = 0usize;
        let mut pusize: usize = 0usize;
        let mut susize: usize = 0usize;
        let mut tusize: usize = 0usize;
        let mut wusize: usize = 0usize;
        let mut yusize: usize = 0usize;
        let mut vusize: usize = 0usize;
        let ihold: Vec<String> = i.chars().map(String::from).collect::<Vec<_>>();
        let mut iholdtuple: Vec<(String, usize)> = Vec::new();
        for i in ihold.iter() {
            if i == "A" {
                ausize += 1usize;
            } else if i == "R" {
                rusize += 1usize;
            } else if i == "N" {
                nusize += 1usize;
            } else if i == "D" {
                dusize += 1usize;
            } else if i == "C" {
                cusize += 1usize;
            } else if i == "E" {
                eusize += 1usize;
            } else if i == "Q" {
                qusize += 1usize;
            } else if i == "G" {
                gusize += 1usize;
            } else if i == "H" {
                husize += 1usize;
            } else if i == "I" {
                iusize += 1usize;
            } else if i == "L" {
                lusize += 1usize;
            } else if i == "K" {
                kusize += 1usize;
            } else if i == "M" {
                musize += 1usize;
            } else if i == "F" {
                fusize += 1usize;
            } else if i == "P" {
                pusize += 1usize;
            } else if i == "S" {
                susize += 1usize;
            } else if i == "T" {
                tusize += 1usize;
            } else if i == "W" {
                wusize += 1usize;
            } else if i == "Y" {
                yusize += 1usize;
            } else if i == "V" {
                vusize += 1usize;
            } else {
                continue;
            }
        }

        iholdtuple.push(("A".to_string(), ausize));
        iholdtuple.push(("R".to_string(), rusize));
        iholdtuple.push(("N".to_string(), nusize));
        iholdtuple.push(("D".to_string(), dusize));
        iholdtuple.push(("C".to_string(), cusize));
        iholdtuple.push(("E".to_string(), eusize));
        iholdtuple.push(("Q".to_string(), qusize));
        iholdtuple.push(("G".to_string(), gusize));
        iholdtuple.push(("H".to_string(), husize));
        iholdtuple.push(("I".to_string(), iusize));
        iholdtuple.push(("L".to_string(), lusize));
        iholdtuple.push(("K".to_string(), kusize));
        iholdtuple.push(("M".to_string(), musize));
        iholdtuple.push(("F".to_string(), fusize));
        iholdtuple.push(("P".to_string(), pusize));
        iholdtuple.push(("S".to_string(), susize));
        iholdtuple.push(("T".to_string(), tusize));
        iholdtuple.push(("W".to_string(), wusize));
        iholdtuple.push(("Y".to_string(), yusize));
        iholdtuple.push(("V".to_string(), vusize));
        finalcount.push(iholdtuple);
    }

    let mut filewrite = File::create("proteome-stats.txt").expect("file not found");
    for i in 0..finalcount.len() {
        writeln!(
            filewrite,
            "{:?}\t{:?}\t{:?}",
            header[i], sequence[i], finalcount[i]
        )
        .expect("file not found");
    }

    Ok("proteome stats have been written".to_string())
}
