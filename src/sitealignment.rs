use crate::filesearch::read;
use std::error::Error;

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn sitespecific(path: &str, head: &str, base: &str) -> Result<String, Box<dyn Error>> {
    let (header, sequence) = read(path).unwrap();
    let mut information: Vec<Vec<usize>> = Vec::new();
    for i in 0..sequence.len() {
        if header[i].to_string() == head.to_string() {
            let seqi = sequence[i].chars().map(String::from).collect::<Vec<_>>();
            let mut seqsite: Vec<_> = Vec::new();
            let _ = seqi.iter().enumerate().map(|(x, y)| {
                if *y == *base {
                    seqsite.push(x)
                }
            });
            information.push(seqsite);
        }
    }
    Ok(
        "The specific information for those bases in the entire alignment have been written"
            .to_string(),
    )
}
