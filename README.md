# eVaiutilities:

- Analyzing and identifying genomic variants is particularly helpful in case of linking the disease onset to the genomic predictions. Genomic variants play a key role in the identification and characterization of the disease onset and also allow for interlinking genomic variability and hypothesis testing. eVai, which is a variant annotation platform provides annotation of the genomic variants using several backhand open source databases and tools. eVaiutilities proposed in this paper, is a data management software for the analysis of the eVai output files. It allows the analysis of the genomic variants further such as analyzing the multiple genomic annotated variants, reference and alternate allele, enabling coordinate search, coordinate search with specified variants and annotation search across a large number of population. The availability of the command line parameters allows for a large scale analysis across the several population files.

- eVaiutilities for data management of the eVai results. It supports all versions of eVai and uses RUSt version rustc 1.89.0 (29483883e 2025-08-04).
- It analyzes the eVai output files for variant annotation to filtering, preparating user reports and database and annotator and more. See the example folder for the older and the newer version files.
- It provides the population scale variant searching given a folder according to the variant, annotation, transcript, gene, and coordinates.
- It uses this version of the human genome from [GENCODE](https://www.gencodegenes.org/human/)

- [Scientific Rust](https://www.youtube.com/watch?app=desktop&v=dru-2Cn-RTQ)

- In this new version, it plots the variants accumulated also.

<img src="https://github.com/genomicssport/eVaiutilities/blob/main/testfiles/barplot.svg" >

```
 cargo build
```

```
__     __          _   _   _   _     _   _   _   _     _
___  \ \   / /   __ _  (_) | | | | | |_  (_) | | (_) | |_  (_)   ___   ___
/ _ \  \ \ / /   / _` | | | | | | | | __| | | | | | | | __| | |  / _ \ / __|
|  __/   \ V /   | (_| | | | | |_| | | |_  | | | | | | | |_  | | |  __/ \__ \
\___|    \_/     \__,_| |_|  \___/   \__| |_| |_| |_|  \__| |_|  \___| |___/


annotating and analyzing eVai results.

************************************************
Gaurav Sablok, IBCH, PAN, Poznan, Poland,
https://portal.ichb.pl/laboratory-of-genomics/.
Email: gsablok@ibch.poznan.pl
************************************************

Usage: eVaiutilities <COMMAND>

Commands:
variant-analyzer                analyzer for the variants
variant-filter                  filter the variants
variant-database                create variant database
gtf-analyze                     analyze the corresponding gtf
variant-seq                     prepare the variant seq annotation
download-genome                 download the human genome
acmg-transcript                 variant-transcriptids
sequence-profile                sequence profiling
population-variant-search       search for the variant across population
coordinate-search               search according to coordinates
coordinate-search-variant       search according to coordinates and variant
annotation-search               search according to annotation
pathogenicity-filter            pathogenicity filter
population-variant-searcholder  search for the variant across population older version
coordinate-searcholder          search according to coordinates older version
coordinate-searc-variantholder  search according to coordinates and variant older version
annotation-searcholder          search according to annotation older version
pathogenicity-filterolder       pathogenicity filter older version
transcript-search               search by the specific transcript
transcript-searcholder          search by the specific transcript older
alt-allele                      search for the ref allele
alt-ref-allele                  search according to ref allele and alt allele
alt-allele-older                search for the ref allele older version
alt-ref-allele-older            search according to ref allele and alt allele older version
variant-plotter                 accumulate all variants for the plots
help                            Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version

```
```
 variant-analyzer ./testfiles/single-population/sample1.tsv ./single/sampletest.tsv
 variant-filter ./testfiles/single-population/sample1.tsv ./single/sampletest.tsv TTT
 gtf-analyze ./testfiles/single-population/samplegtf.gtf
 variant-seq ./testfiles/single-population/sample1.tsv ./single/sample.fasta
 download-genome yes
 acmg-transcript ./testfiles/single-population/sample1.tsv
 sequence-profile ./testfiles/single-population/sample1.tsv ./data-repo/sample.fasta 10 10
 population-variant-search ./testfiles/newversion-population G samplename
 coordinate-search ./testfiles/newversion-population 14464 16495 samplename
 annotation-search ./testfiles/newversion-population AL645608.6 samplename
 pathogenicity-filter ./testfiles/newversion-population 0.0 1.5 samplename
 population-variant-searcholder ./testfiles/oldversion-population G samplename
 coordinate-searcholder ./testfiles/oldversion-population 14464 16495 samplename
 annotation-searcholder ./testfiles/oldversion-population WASH7P samplename
 pathogenicity-filterolder ./testfiles/oldversion-population 0 1.5 samplename
 transcript-search ./testfiles/newversion-population ENST00000635509.2 samplename
 alt-allele ./testfiles/newversion-population T samplename
 alt-allele-older ./testfiles/oldversion-population T samplename
 alt-ref-allele ./testfiles/newversion-population A T samplename
 alt-ref-allele-older ./testfiles/oldversion-population A T samplename
 variant-plotter ./testfiles/variant-accumulation
```

- A detailed readme documentaiton as to understand the eVAiutilities.
```
variant-analyzer: analyzer for the variants and create a common variant file, so that you don't have duplicated information coming from the variant annotation.
variant-filter:filter the variant according to the user specified criteria.
variant-database:create variant database for all the annotation files.
gtf-analyze:analyze the corresponding gtf annotation for the variants.
variant-seq:prepare the variant seq annotation for the identified variants.
download-genome:download the human genome and prepare for the annotations.
acmg-transcript:single sample analysis according to variant-transcriptids
sequence-profile:Single sample analysis according for sequence profiling
population-variant-search:: Search for a specific variant across population.
coordinate-search:Filter the single sample analysis according to coordinates.
coordinate-search-variant:Filter the single sample analysis according to coordinates and variant.
annotation-search: Filter the single sample analysis according to annotation.
pathogenicity-filter: Filter the single sample analysis according to pathogenicity.
population-variant-searcholder: Scan the entire population for the specified variant.Here older version means the older version of eVAI.
coordinate-searcholder:search for the population according to coordinates.Here older version means the older version of eVAI.
coordinate-searc-variantholder:search for the population according to coordinates and variant.Here older version means the older version of eVAI.
annotation-searcholder:search for the population for annotation.Here older version means the older version of eVAI.
pathogenicity-filterolder:Filter the entire population for pathogenicity.Here older version means the older version of eVAI.
transcript-search:search for the population for a specific transcript.This supports new version.
transcript-searcholder:search for the population for a specific transcript.Here older version means the older version of eVAI.
alt-allele:search for the population according to ref allele.
alt-ref-allele:search for the population according to ref allele and alt allele
alt-allele-older:search for the population according to ref allele. Here older version means the older version of eVAI.
alt-ref-allele-older:search the population according to ref allele and alt allele. Here older version means the older version of eVAI.
variant-plotter: population scale variant summarizer, which accumulate all variants annotated and prepares a plot for the variant annotation.
```

- To install windows version:


```
rustup component add llvm-tools
rustup target add x86_64-pc-windows-msvc
git clone https://github.com/IBCHgenomic/eVaiutilities.git
cd ensemblcov
cargo xwin build --target x86_64-pc-windows-msvc
```

 Gaurav Sablok \
 Instytut Chemii Bioorganicznej \
 Polskiej Akademii Nauk \
 ul. Noskowskiego 12/14 | 61-704, Poznań \
 Poland
