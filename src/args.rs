use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(name = "variantannotator", version = "1.0", about = "variantannotator")]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// analyzer for the variants
    VariantAnalyzer {
        /// provide the path to the acmg file
        acmgfile: String,
        /// provide the path to the tsv file
        tsvfile: String,
    },
    /// filter the variants
    VariantFilter {
        /// provides the path to the acmg file
        acmgfile: String,
        /// provides the path to the tsv file
        tsvfile: String,
        /// provide the variant
        variant: String,
    },
    /// create variant database
    VariantDatabase {
        /// please provide the path to the acmg file
        acmgfile: String,
        /// please provide the path to the tsv file
        tsvfile: String,
    },
    /// analyze the corresponding gtf
    GTFAnalyze {
        /// path to the gtf file
        gtffile: String,
    },
    /// prepare the variant seq annotation.
    VariantSeq {
        /// provide the ACMG file
        acmgfile: String,
        /// provide fasta file
        fastafile: String,
    },
    /// download the human genome
    DownloadGenome { input: String },
    /// variant-transcriptids
    ACMGTranscript {
        /// provide the ACMG file
        acmgfile: String,
    },
    /// sequence profiling
    SequenceProfile {
        /// provide the acmg file
        acmgfile: String,
        /// provide the fasta file
        fastafile: String,
        /// upstream location
        upstream: usize,
        /// downstream location
        downstream: usize,
        /// variant
        variant: String,
    },
    /// search for the variant across population
    PopulationVariantSearch {
        /// provide the acmg directory
        acmgdir: String,
        /// provide the variant
        variant: String,
        /// analysis name
        name: String, 
    },
    /// search according to coordinates
    CoordinateSearch {
        /// provide the acmg file
        acmgfile: String,
        /// provide the coordinate
        start: usize,
        /// provide the end coordinate
        end: usize,
        /// analysis name
        name: String,
    },
    /// search according to annotation
    AnnotationSearch {
        /// provide the path to acmg file
        acmgfile: String,
        /// search the annotation
        genename: String,
        /// analysis name
        name: String, 
    },
    /// pathogenicity filter
    PathogenicityFilter {
        /// provide the acmg file
        acmgfile: String,
        /// value
        value: f32,
    },
}
