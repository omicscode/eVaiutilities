use crate::structfile::ExonCollate;
use crate::structfile::GRange;
use crate::structfile::GeneMapper;
use std::env::args;
use std::error::Error;
use std::fmt::write;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-3-12
*/

pub fn analyzegtf(pathgtf: &str) -> Result<String, Box<dyn Error>> {
    let fileread = File::open(pathgtf).expect("file not found");
    let fileread = BufReader::new(fileread);

    let mut g_range: Vec<GRange> = Vec::new();
    let mut genemapper: Vec<GeneMapper> = Vec::new();
    let mut linecollect: Vec<String> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("file not present");
        if line.starts_with("#") {
            continue;
        }
        if !line.starts_with("#") {
            let linevec = line
                .split("\t")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            g_range.push(GRange {
                id: linevec[0].clone(),
                name: linevec[1].clone(),
                genetype: linevec[2].clone(),
                start: linevec[3].clone(),
                end: linevec[4].clone(),
                index: linevec[5].clone(),
                strand: linevec[6].clone(),
                indexend: linevec[7].clone(),
                collectable: linevec[8].clone(),
            });
            genemapper.push(GeneMapper {
                gene: linevec[2].clone(),
                start: linevec[3].parse::<usize>().unwrap(),
                end: linevec[4].parse::<usize>().unwrap(),
            });
            linecollect.push(line);
        }
    }

    let mut exonvector: Vec<ExonCollate> = Vec::new();
    for i in genemapper.iter() {
        for j in linecollect.iter() {
            let linecollectvec = j
                .split("\t")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            let mut exonvec: Vec<(String, usize, usize)> = Vec::new();
            if linecollectvec[2] == "exon"
                && linecollectvec[3].parse::<usize>().unwrap() == i.start
                && linecollect[4].parse::<usize>().unwrap() <= i.end
            {
                exonvec.push((
                    linecollectvec[2].clone(),
                    linecollectvec[3].parse::<usize>().unwrap(),
                    linecollectvec[4].parse::<usize>().unwrap(),
                ));
            }
            for i in exonvec.iter() {
                exonvector.push(ExonCollate {
                    name: i.0.clone(),
                    start: i.1,
                    end: i.2,
                });
            }
        }
    }

    Ok("The gtf have been analyzed".to_string())
}
