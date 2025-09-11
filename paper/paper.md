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

[eVai](https://www.engenome.com/product/) is a variant annotation platform that leverages multiple open-source databases and tools to annotate genomic variants. It has been widely adopted in both industry and [academic research](https://www.engenome.com/resources/?category=our-publications
). However, because eVai lacks native command-line support for high-throughput, large-scale population analyses, its use in population-scale studies can be limited. To address this limitation, I developed eVaiutilities, a command-line tool that processes annotations produced by eVai as well as those from other platforms following a compatible format. It enables high-throughput analysis of multiple population files and supports deeper exploration of genomic variation at the population level. eVaiutilities integrates analyses of annotated variants across populations, examination of reference and alternate alleles, coordinate-based searches, and large-scale annotation searches across extensive datasets. This extension allows scalable and reproducible population-level analyses and has the potential to accelerate research on disease-associated variants and support the development of more effective disease-combatting strategies.

# Statement of need

Human genomics remains a compelling area of research, with extensive efforts dedicated to identifying and characterizing genetic variants. In addition to open-source initiatives in variant annotation [@McLaren2016-jo; @Pedersen2023-co], several proprietary platforms have been developed to classify and link variants to genetic and metabolic disorders, spanning a wide spectrum of clinical relevance. One such platform is eVai, a high-throughput variant annotator that provides hierarchical classification of genomic variants. However, eVai is primarily browser-based and therefore lacks native support for high-throughput searches and for categorical or functional analyses of annotated variants across large-scale datasets. To address these limitations, we developed eVaiutilities, which takes eVai output and enables genome-scale analyses of annotated variants. It supports population-wide and variant-level searches across multiple samples and datasets, providing a scalable and efficient solution. eVaiutilities is implemented in Rust (version 1.89.0), a high-performance and memory-safe programming language.

# Figures
![Interface of eVaiutilities](eVaiutilities.png)

# eVaiutilities

Comprehensive annotation of human genomic variants is essential for biological interpretation and for linking variants to disease phenotypes. Numerous tools have been developed that annotate variants using pre-established databases [@Wang2010-mh; @Obenchain2014-jj]. Alongside significant advances in open-source genomics [@McLaren2016-jo; @Pedersen2023-co], parallel efforts have emerged within industry to support variant identification and annotation. eVai is one such platform, providing comparative analyses of variants across multiple tools and databases and generating TSV files with mapped annotations. While these standard analyses facilitate variant interpretation, large-scale variant searches additionally require high speed and scalability. To meet these needs, we developed eVaiutilities in the memory-safe language Rust. eVaiutilities extends eVai by enabling population-scale and customized analyses of annotated variants, including summarization and reporting of annotations, transcript sequence spanning around variants, and categorical variant filtering. Key functions include population-wide analysis of specific variants, coordinate-based searches across populations, variant searches within specified genomic coordinates, sequence-specific analyses, and reference and alternate allele queries. Together, these features provide a scalable and versatile platform for managing and interrogating eVai outputs. eVaiutilities is freely available on [GitHub](https://github.com/genomicssport/eVaiutilities) and as a Rust [crate](https://crates.io/crates/eVaiutilities).


# Acknowledgements

I thank Julia Romanowska and reviewers for working on the text revisions of the paper, which has significantly improved the reading content of the eVaiutilties. 

# References
