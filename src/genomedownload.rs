use cmd_lib::*;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;
/*
 Authom Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-3-19
*/

pub fn downloadgenome(input: &str) -> Result<String, Box<dyn Error>> {
    if input == "yes" {
        let _ = fs::create_dir("./download").unwrap();
        let newpath = Path::new("./download");
        let _ = env::set_current_dir(newpath);

        let _ = Command::new("wget")
            .arg("-F")
            .arg("https://www.ncbi.nlm.nih.gov/grc/human")
            .output()
            .expect("command to fail");

        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/GCA_000001405.15_GRCh38_genomic.fna.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/ GCA_000001405.15_GRCh38_genomic.gff.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/GCA_000001405.15_GRCh38_genomic.gaps.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/GCA_000001405.15_GRCh38_genomic.gbff.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/GCA_000001405.15_GRCh38_genomic.gff.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/GCA_000001405.15_GRCh38_protein.gpff.gz")
            .output()
            .expect("command to fail");

        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_feature_count.txt.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_feature_table.txt.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_genomic.fna.gz",).output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_genomic.gbff.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_genomic.gff.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_genomic.gtf.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_genomic_gaps.txt.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_protein.faa.gz",).output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_protein.gpff.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_rna_from_genomic.fna.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_translated_cds.faa.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_cds_from_genomic.fna.gz").output()
            .expect("command to fail");

        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_feature_count.txt.gz >> download.txt)
            .expect("run command failed");
        let _ =
            run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_cds_from_genomic.fna.gz >> download.txt)
                .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_feature_table.txt.gz >> download.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_genomic.fna.gz >> download.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_genomic_gaps.txt.gz >> download.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_genomic.gbff.gz >> download.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_genomic.gff.gz >> download.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_genomic.gtf.gz >> download.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_protein.faa.gz >> download.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_protein.gpff.gz >> download.txt)
            .expect("run command failed");
        let _ =
            run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_rna_from_genomic.fna.gz >> download.txt)
                .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_translated_cds.faa.gz >> download.txt)
            .expect("run command failed");

        let mut md5sumarray: HashMap<String, String> = HashMap::new();

        let filenames: Vec<_> = vec![
            "GCA_000001405.29_GRCh38.p14_cds_from_genomic.fna.gz",
            "GCA_000001405.29_GRCh38.p14_feature_count.txt.gz",
            "GCA_000001405.29_GRCh38.p14_feature_table.txt.gz",
            "GCA_000001405.29_GRCh38.p14_genomic.fna.gz",
            "GCA_000001405.29_GRCh38.p14_genomic_gaps.txt.gz",
            "GCA_000001405.29_GRCh38.p14_genomic.gbff.gz",
            "GCA_000001405.29_GRCh38.p14_genomic.gff.gz",
            "GCA_000001405.29_GRCh38.p14_genomic.gtf.gz",
            "GCA_000001405.29_GRCh38.p14_protein.faa.gz",
            "GCA_000001405.29_GRCh38.p14_protein.gpff.gz",
            "GCA_000001405.29_GRCh38.p14_rna_from_genomic.fna.gz",
            "GCA_000001405.29_GRCh38.p14_translated_cds.faa.gz",
        ];

        let md5sumvalues: Vec<_> = vec![
            "f7db7fd4066d6fce37b08931857f9d2b",
            "9f3ce2e0aef6feda7e4d502e9e9dc3bd",
            "24e692b246c2277bb8b05f94645854f5",
            "22907ad69ddfae66071bf9cf99b3e8de",
            "462d5d39c74678926f2ec86b3485324c",
            "02c7df80888b6aaea837f566c8a820c8",
            "6642265d15316bb25e70d1ee149a51fd",
            "a48db386de8673c41b602aa79932b6f2",
            "a40f2415ddbc6596a3186ee4982230ef",
            "77b28fadc74ad17440fc25492c90a2c1",
            "59e14460cf8200208a759d59c818f79b",
            "e021683af78cf0989273590a691ee1bd",
        ];

        for i in 0..filenames.len() {
            md5sumarray.insert(filenames[i].to_string(), md5sumvalues[i].to_string());
        }

        let fileopen = File::open("download.txt").expect("file not present");
        let fileread = BufReader::new(fileopen);
        let mut filevecnames: Vec<String> = Vec::new();
        let mut filevecmd5sum: Vec<String> = Vec::new();

        for i in fileread.lines() {
            let line = i.expect("file not present");
            let linevec = line.split("\t").collect::<Vec<_>>();
            filevecmd5sum.push(linevec[1].to_string());
            filevecnames.push(linevec[0].to_string());
        }

        for i in filevecmd5sum.iter() {
            for j in md5sumvalues.iter() {
                if i == j {
                    continue;
                } else if i != j {
                    println!("The downloaded files are not complete");
                }
            }
        }
    }

    Ok("The downloaded version of the human genome has been written to the disc".to_string())
}
