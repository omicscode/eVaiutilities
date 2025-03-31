mod acmgtranscript;
mod annotation;
mod args;
mod coordinate;
mod fastanalyzer;
mod genomeanalyzer;
mod genomedownload;
mod gtfanalyzer;
mod pathogenicity;
mod populationvariant;
mod sequenceprofile;
mod structfile;
mod variantdatabase;
mod variantfilter;
use crate::acmgtranscript::acmgannotate;
use crate::annotation::annotationsearch;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::coordinate::coordinatesearch;
use crate::fastanalyzer::fastagtf;
use crate::genomeanalyzer::genomemap;
use crate::genomedownload::downloadgenome;
use crate::gtfanalyzer::analyzegtf;
use crate::pathogenicity::pathogenicityscore;
use crate::populationvariant::population;
use crate::sequenceprofile::sequence;
use crate::variantdatabase::variantdatabase;
use crate::variantfilter::variantanalyzer;
use clap::Parser;

/*
 Authom Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-3-12
*/

fn main() {
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::VariantAnalyzer { acmgfile, tsvfile } => {
            let command = genomemap(acmgfile, tsvfile).unwrap();
            println!("The file has been converted: {:?}", command);
        }
        Commands::VariantFilter {
            acmgfile,
            tsvfile,
            variant,
        } => {
            let command = variantanalyzer(acmgfile, tsvfile, variant).unwrap();
            println!("The filtered variant file has been written:{}", command);
        }
        Commands::VariantDatabase { acmgfile, tsvfile } => {
            let command = variantdatabase(acmgfile, tsvfile).unwrap();
            println!("The variant database has been created:{}", command);
        }
        Commands::GTFAnalyze { gtffile } => {
            let command = analyzegtf(gtffile).unwrap();
            println!("The gtf has been analyzed:{:?}", command);
        }
        Commands::VariantSeq {
            acmgfile,
            fastafile,
        } => {
            let command = fastagtf(acmgfile, fastafile).unwrap();
            println!("The sequences have been written:{:?}", command);
        }
        Commands::DownloadGenome { input } => {
            let command = downloadgenome(input).unwrap();
            println!("The genome has been downloaded:{:?}", command);
        }
        Commands::ACMGTranscript { acmgfile } => {
            let command = acmgannotate(acmgfile).unwrap();
            println!("The transcript ids have been written:{:?}", command);
        }
        Commands::SequenceProfile {
            acmgfile,
            fastafile,
            upstream,
            downstream,
        } => {
            let command = sequence(acmgfile, fastafile, *upstream, *downstream).unwrap();
            println!("The sequences have been written:{:?}", command);
        }
        Commands::PopulationVariantSearch { acmgdir, variant } => {
            let command = population(acmgdir, variant).unwrap();
            println!("The command has finished:{}", command);
        }
        Commands::CoordinateSearch {
            acmgfile,
            start,
            end,
        } => {
            let command = coordinatesearch(acmgfile, *start, *end).unwrap();
            println!("The command has been finished:{}", command);
        }
        Commands::AnnotationSearch { acmgfile, genename } => {
            let command = annotationsearch(acmgfile, genename).unwrap();
            println!("The command has been finished:{}", command);
        }
        Commands::PathogenicityFilter { acmgfile, value } => {
            let command = pathogenicityscore(acmgfile, *value).unwrap();
            println!("The command has completed:{}", command);
        }
    }
}
