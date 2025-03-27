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
    Analyzer {
        /// provide the path to the acmg file
        acmgfile: String,
        /// provide the path to the tsv file
        tsvfile: String,
    },
    /// filter the variants
    Filter {
        /// provides the path to the acmg file
        acmgfile: String,
        /// provides the path to the tsv file
        tsvfile: String,
        /// provide the variant
        variant: String,
    },
    /// create variant database
    Database {
        /// please provide the path to the acmg file
        acmgfile: String,
        /// please provide the path to the tsv file
        tsvfile: String,
        /// please provide the name of the database
        databasename: String,
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
    },
}
