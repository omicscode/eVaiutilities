mod acmgtranscript;
mod annotation;
mod annotationolder;
mod args;
mod coordinate;
mod coordinateolder;
mod fastanalyzer;
mod genomeanalyzer;
mod genomedownload;
mod gtfanalyzer;
mod pathogenicity;
mod pathogenicityolder;
mod populationvariant;
mod populationvariantolder;
mod sequenceprofile;
mod structfile;
mod variantdatabase;
mod variantfilter;
use crate::acmgtranscript::acmgannotate;
use crate::annotation::annotationsearch;
use crate::annotationolder::annotationsearcholder;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::coordinate::coordinatesearch;
use crate::coordinateolder::coordinatesearcholder;
use crate::fastanalyzer::fastagtf;
use crate::genomeanalyzer::genomemap;
use crate::genomedownload::downloadgenome;
use crate::gtfanalyzer::analyzegtf;
use crate::pathogenicity::pathogenicityscore;
use crate::pathogenicityolder::pathogenicityscoreolder;
use crate::populationvariant::population;
use crate::populationvariantolder::populationolder;
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
            variant,
        } => {
            let command = sequence(acmgfile, fastafile, *upstream, *downstream, variant).unwrap();
            println!("The sequences have been written:{:?}", command);
        }
        Commands::PopulationVariantSearch {
            acmgdir,
            variant,
            name,
        } => {
            let command = population(acmgdir, variant, name.to_string()).unwrap();
            println!("The command has finished:{}", command);
        }
        Commands::CoordinateSearch {
            acmgdir,
            start,
            end,
            name,
        } => {
            let command = coordinatesearch(acmgdir, *start, *end, name).unwrap();
            println!("The command has been finished:{}", command);
        }
        Commands::AnnotationSearch {
            acmgdir,
            genename,
            name,
        } => {
            let command = annotationsearch(acmgdir, genename, name).unwrap();
            println!("The command has been finished:{}", command);
        }
        Commands::PathogenicityFilter {
            acmgdir,
            value,
            name,
        } => {
            let command = pathogenicityscore(acmgdir, *value, name).unwrap();
            println!("The command has completed:{}", command);
        }
        Commands::PopulationVariantSearcholder {
            acmgdir,
            variant,
            name,
        } => {
            let command = populationolder(acmgdir, variant, name.to_string()).unwrap();
            println!("The command has finished:{}", command);
        }
        Commands::CoordinateSearcholder {
            acmgdir,
            start,
            end,
            name,
        } => {
            let command = coordinatesearcholder(acmgdir, *start, *end, name).unwrap();
            println!("The command has been finished:{}", command);
        }
        Commands::AnnotationSearcholder {
            acmgdir,
            genename,
            name,
        } => {
            let command = annotationsearcholder(acmgdir, genename, name).unwrap();
            println!("The command has been finished:{}", command);
        }
        Commands::PathogenicityFilterolder {
            acmgdir,
            value,
            name,
        } => {
            let command = pathogenicityscoreolder(acmgdir, *value, name).unwrap();
            println!("The command has completed:{}", command);
        }
    }
}
