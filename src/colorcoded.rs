use crate::filesearch::read;
use colored::Colorize;
use std::error::Error;

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn readcodecolor(path: &str) -> Result<String, Box<dyn Error>> {
    let (header, sequence) = read(path).unwrap();

    let mut vecsize: Vec<(String, usize, usize, usize, usize)> = Vec::new();

    for i in 0..sequence.len() {
        let iholda: usize = sequence[i]
            .chars()
            .map(String::from)
            .filter(|x| x == "A")
            .collect::<Vec<_>>()
            .len();

        let iholdt: usize = sequence[i]
            .chars()
            .map(String::from)
            .filter(|x| x == "T")
            .collect::<Vec<_>>()
            .len();
        let iholdg: usize = sequence[i]
            .chars()
            .map(String::from)
            .filter(|x| x == "G")
            .collect::<Vec<_>>()
            .len();
        let iholdc: usize = sequence[i]
            .chars()
            .map(String::from)
            .filter(|x| x == "C")
            .collect::<Vec<_>>()
            .len();
        vecsize.push((header[i].clone(), iholda, iholdt, iholdg, iholdc));
    }

    for i in vecsize.iter() {
        if !i.0.is_empty() {
            print!(
                "{}\t{}-{}-{}-{}",
                i.0,
                i.1.to_string().on_yellow().bold(),
                i.2.to_string().on_blue().bold(),
                i.3.to_string().on_red().bold(),
                i.4.to_string().on_magenta().bold()
            );
        } else {
            continue;
        }
        println!();
    }

    Ok("The colour coded results are as follows".to_string())
}
