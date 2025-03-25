use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};
use crate::structfile::Transcript;
use std::io::Write;

/*
 Authom Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-3-25 
*/


pub fn acmgannotate(pathacmg: &str) -> Result<String, Box<dyn Error>> {
    
    let acmgopen = File::open(pathacmg).expect("file not present");
    let acmgread = BufReader::new(acmgopen);
    let mut priortranscript: Vec<Transcript> = Vec::new();

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
        priortranscript.push(Transcript{
           variant: linecut[3].to_string(),
           id: linegenome.clone(),
           additional: linecut[5].to_string(),
        });
        }
    }

    let mut filewrite = File::create("variant-transcript-annotation.txt").expect("file not present");

    for i in priortranscript.iter(){
      writeln!(filewrite, "{}\t{:?}\t{}", i.variant, i.id.join(","), i.additional).expect("line not present");

    }
        Ok("The ids have been written to the file".to_string())
   }
