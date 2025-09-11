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
date: 10 April 2025
bibliography: paper.bib
---

# Summary

[eVai](https://www.engenome.com/product/) is a variant annotation platform that leverages multiple open-source databases and tools to annotate genomic variants. It has been widely adopted in both industry and academic research (https://www.engenome.com/resources/?category=our-publications). However, eVai lacks native command-line support for high-throughput, large-scale population analyses, which limits its use in population-scale studies. To address this limitation, we developed eVaiutilities, a command-line tool that can process annotations produced by eVai as well as those from other platforms following a compatible format. It enables high-throughput analysis of multiple population files and deeper exploration of genomic variation at the population level. Key functions include the analysis of annotated variants across populations, examination of reference and alternate alleles, coordinate-based searches, and large-scale annotation searches across extensive datasets. By filling this gap, eVaiutilities extends the utility of eVai to scalable and reproducible population-level analyses and has the potential to accelerate research on disease-associated variants and support the development of more effective disease-combatting strategies. Email: codeprog@icloud.com

# Statement of need

Human genomics remains a compelling area of research, with extensive efforts dedicated to identifying and characterizing genetic variants. In addition to open-source initiatives in variant annotation [@McLaren2016-jo; @Pedersen2023-co], several proprietary platforms have been developed to classify and link variants to genetic and metabolic disorders, ranging from benign to highly pathogenic. One such platform is eVai, a high-throughput variant annotator that provides hierarchical classification of genomic variants. However, eVai is primarily browser-based and therefore lacks native support for high-throughput searches and for categorical or functional analyses of annotated variants across large-scale datasets. To address these limitations, we developed eVaiutilities, which takes eVai output and enables genome-scale analyses of annotated variants. It supports population-wide and variant-level searches across multiple samples and datasets, providing a scalable and efficient solution. eVaiutilities is implemented in Rust (version 1.89.0), a high performance and memory safe programming language.

# Figures
![Interface of eVaiutilities](eVaiutilities.png)

# eVaiutilities

Variant identification and annotation across the human genome have broad biological significance, and numerous tools have been developed that annotate variants using pre-established databases [@Wang2010-mh; @Obenchain2014-jj]. Alongside significant advances in open-source genomics [@McLaren2016-jo; @Pedersen2023-co], parallel efforts have emerged within industry to support variant identification and annotation. [eVai](https://www.engenome.com/product/) is one such platform, providing comparative analyses of variants across multiple tools and databases and generating TSV files with mapped annotations. While these standard analyses facilitate variant interpretation, large-scale variant searches additionally require high speed and scalability. To address these needs, we developed eVaiutilities, implemented in the memory-safe language Rust. eVaiutilities extends eVai by enabling population-scale and customized analyses of annotated variants, including summarization and reporting of annotations, transcript sequence spanning around variants, and categorical variant filtering. Key functions include population-wide analysis of specific variants, coordinate-based searches across populations, variant searches within specified genomic coordinates, sequence-specific analyses, and reference and alternate allele queries. Together, these features make eVaiutilities a scalable and versatile platform for managing and interrogating eVai outputs. eVaiutilities is freely available on [GitHub](https://github.com/genomicssport/eVaiutilities) and as a Rust [crate](https://crates.io/crates/eVaiutilities).


# Acknowledgements

I thank Julia Romanowska for working on the text revisions of the paper, which has significantly improved the reading content of the eVaiutilties. 

# References
