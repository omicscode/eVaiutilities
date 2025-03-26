use crate::structfile::Fasta;
use crate::structfile::FastaUpdown;
use crate::structfile::PriorTranscript;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;
/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-3-18
*/

pub fn fastagtf(
    pathacmg: &str,
    pathfasta: &str,
    sequenceupstream: usize,
    sequencedownstream: usize,
) -> Result<String, Box<dyn Error>> {
    let fastafile = File::open(pathfasta).expect("file not present");
    let fastaread = BufReader::new(fastafile);
    let mut vecfastaid: Vec<String> = Vec::new();
    let mut vecfastaseq: Vec<String> = Vec::new();
    for i in fastaread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            let lineselect: Vec<_> = line.split(" ").collect::<Vec<_>>();
            vecfastaid.push(lineselect[0].to_string().replace(">", ""));
        } else if !line.starts_with(">") {
            vecfastaseq.push(line);
        }
    }
    let mut fastacombine: Vec<Fasta> = Vec::new();
    for i in 0..vecfastaid.len() {
        fastacombine.push(Fasta {
            header: vecfastaid[i].clone(),
            sequence: vecfastaseq[i].clone(),
        });
    }

    let acmgopen = File::open(pathacmg).expect("file not present");
    let acmgread = BufReader::new(acmgopen);
    let mut priortranscript: Vec<PriorTranscript> = Vec::new();
    for i in acmgread.lines() {
        let line = i.expect("line not present");
        if line.starts_with("#") {
            continue;
        } else if !line.starts_with("#") {
            let linecut: Vec<_> = line.split("\t").map(|x| x.to_string()).collect::<Vec<_>>();
            let linegenome: Vec<_> = linecut[9]
                .split("|")
                .collect::<Vec<_>>()
                .into_iter()
                .map(|x| x.replace(":null", ""))
                .collect::<Vec<_>>();
            priortranscript.push(PriorTranscript {
                prior: linecut[5].clone(),
                alternate: linegenome,
            });
        }
    }
 
        let _ = Command::new("wget")
            .arg("https://ftp.ensembl.org/pub/release-113/fasta/homo_sapiens/cds/Homo_sapiens.GRCh38.cds.all.fa.gz")
            .output()
            .expect("result not found");

        let mut sequencewrite: Vec<Fasta> = Vec::new();
        for i in flattentranscript.iter() {
            for j in fastacombine.iter() {
                if *i == j.header.to_string() {
                    sequencewrite.push(Fasta {
                        header: i.clone(),
                        sequence: j.sequence.clone(),
                    })
                }
            }
        }

        let mut sequenceflank: Vec<FastaUpdown> = Vec::new();
        for i in flattentranscript.iter() {
            for j in fastacombine.iter() {
                for idster in priortranscript.iter() {
                    if *i == j.header.to_string() {
                        sequenceflank.push(FastaUpdown {
                            header: i.clone(),
                            sequence: j.sequence.clone(),
                            upstream: j.sequence.to_string()
                                [idster.start - sequenceupstream..sequencedownstream]
                                .to_string(),
                            downstream: j.sequence[sequenceupstream..idster.end].to_string(),
                        })
                    }
                }
            }
        }
    }
    Ok("The sequences have been written".to_string())
}
