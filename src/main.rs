mod args;
mod genomeanalyzer;
mod structfile;
mod variantaccumulation;
mod variantdatabase;
mod variantfilter;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::genomeanalyzer::genomemap;
use crate::variantaccumulation::variantpopulate;
use crate::variantdatabase::variantdatabase;
use crate::variantfilter::variantanalyzer;
use clap::Parser;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-3-12
*/

fn main() {
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::Analyzer { acmgfile, tsvfile } => {
            let command = genomemap(acmgfile, tsvfile).unwrap();
            println!("The file has been converted: {:?}", command);
        }
        Commands::Filter {
            acmgfile,
            tsvfile,
            variant,
        } => {
            let command = variantanalyzer(acmgfile, tsvfile, variant).unwrap();
            println!("The filtered variant file has been written:{}", command);
        }
        Commands::Populate { acmgfile, tsvfile } => {
            let command = variantpopulate(acmgfile, tsvfile).unwrap();
            println!("The variant accumulation file has been written:{}", command);
        }
        Commands::Database {
            acmgfile,
            tsvfile,
            databasename,
        } => {
            let command = variantdatabase(acmgfile, tsvfile, databasename).unwrap();
            println!("The variant database has been created:{}", command);
        }
    }
}
