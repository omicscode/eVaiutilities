---
title: "eVaiutilities: Genomics population scale utilities for eVai"
tags:
  - RUST
  - human population genomics
  - variant analysis
  - varaint annotation
  - variant search
authors:
  - name: Gaurav Sablok
    orcid: 0000-0002-4157-9405
    affiliation: 1
  - name: Malgorzata Marcinkowska-Swojak,
    orcid: 0000-0001-8809-335X
    affiliation: 1
  - name: Sylwia Nawrocka,
    affiliation: 1
  - name: Pawel Wojciechowski,
    orcid: 0000-0001-8020-9493
    affiliation: "1,2"
  - name: Luiza Handschuh
    orcid: 0000-0001-9803-6877
    affiliation: 1
affiliations:
  - name: Institute of Bioorganic Chemistry, Polish Academy of Sciences,Noskowskiego 12/14, 61-704, Poznan, Poland
    index: 1
  - name:  Institute of Computing Science, Poznan University of Technology,60-965 Poznan, Poland
    index: 2
date: 10 April 2025
bibliography: paper.bib
---

# Summary
Analyzing and identifying genomic variants is particularly helpful when linking the _disease onset_ to the genomic predictions. Genomic variants play a key role in the identification and 
characterization of the _disease onset_ and also allow for interlinking genomic variability and hypothesis testing. eVai, a variant annotation platform, provides annotation of the genomic variants 
using several backend open source databases and tools. eVaiutilities proposed in this paper, is a ~~post-eVAI~~ analyzer for population scale analysis for eVAI variant annotation. It allows the 
_analysis of the genomic variants further such as analyzing the multiple genomic annotated variants_, reference and alternate allele, enabling coordinate search, coordinate search with specified 
variants and annotation search across _a large number of population_. The availability of the command line parameters allows for a large scale analysis across the several population files. _eVAI 
has been used by many people and has been very well cited_, however, it does not allow for any high-throughput analyses. Thus, we developed eVaiutilities, command-line tool that enables many 
analyses at a time, and has more functionality than eVai.

# Statement of need

Human genomics has always been an intriguing and gleaming topic of interest. Many efforts have been laid for understanding and identifying variants. Apart from the _open source genomics efforts_,
proprietary efforts have been developed for the identification and annotation of the variants. _The identified variants have been classified and linked to genetic and metabolic disorders_, with 
severity ranging from pathogenic to highly pathogenic. One such effort is the eVai ([https://www.engenome.com/product/](https://www.engenome.com/product/)), which is a _high throughput_ variant 
annotator, and provides the hierarchical classification of the genomic variants. However, the limitation of the eVai output files is that the information is spread across multiple files and that 
the _missing of the genome information and sub-sequence analysis on the annotated variants_. eVaiutilities _uses the output from the eVai_ and performs genome scale analysis of the eVai annotated 
variants. It allows you to search across populations, variants and spanning information across multiple samples. It is encoded in RUST, which is a memory safe language and provides scalability and 
uses RUST version: `rustc 1.89.0`.

# Figures
![Interface of eVaiutilities](eVaiutilities.png)

# eVaiutilities

Variant identification and annotation across the human genome has wide biological significance and is of utmost importance taking into account the _wide growth of the disease onset that are 
happening worldwide_.  Many tools have been developed for the annotation of the variants that use several established databases [@Wang2010-mh;@Obenchain2014-jj]. Apart from the significant
development in the open source genomics [@McLaren2016-jo;@Pedersen2023-co], parallel efforts have been laid down for the variant identification and annotation in industrial research. eVai available
through [https://www.engenome.com/product/](https://www.engenome.com/product/), provides customary analysis of variants by comparing the variants across the several tools and databases and provides
TSV files with the annotations mapped. However, customary analysis is needed to make _personal genomics_ easier. Additionally, for large scale variant searching, _speed and scalability is needed_.
To address all these issues, eVaiutilities has been built using the RUST language. eVaiutilities provides a compendium of options to allow for the searching of the variants across the population 
and generate customary analysis. It _provides from variant report generation to the user specified database and sequence annotation_. eVaiutilities _features such as_ the transcript sequence to 
sequence spanning around the variants, allow the ease of the access to generate the _molecular methods_ for the variants of interest and also allows for the categorical variant filtering. There are
several features present in the eVaiutilities, such as population wide analysis of a particular variant, coordinate search for variants across population, variant search across the specific 
coordinates with the specified variant across population, sequence specific analysis, reference and alternate allele search across the population. All the aforementioned features makes eVaiutilties
an easy to use data management software for doing the customary analysis with the eVai. eVaiutilities is available from [GitHub](https://github.com/IBCHgenomic/eVaiutilities) and as a crate from 
[https://crates.io/crates/eVaiutilities](https://crates.io/crates/eVaiutilities). 

# References
