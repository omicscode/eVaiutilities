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
Variant annotation and identifying variants linked to diseases can help us postulate and device strategies for combating diseases. [eVai](https://www.engenome.com/product/), which is a variant annotation platform provides annotation of the genomic variants using several backend open source databases and tools and has been widely used across industry as well as academics [research](https://www.engenome.com/resources/?category=our-publications). eVaiutilities proposed in this paper, is a variant analyzer for population scale analysis for variant annotation. It not only analyzes the variant annnotation from the eVai but also the other variant annotation platforms for which the annotation format is similar as the format specified for the variant annotation. It allows the analysis of the genomic variants further such as analyzing the annotated variants at the population scale, reference and alternate allele, enabling coordinate search, coordinate search with specified variants and annotation search across a large number of population. The availability of the command line parameters allows for a large scale analysis across the several population files. eVAI has been used by many people and has been very well cited, so the lack of the use of the command line for the high-throughput analysis promoted to develop eVaiutilities. Email: gsablok@ibch.poznan.pl

# Statement of need

Human genomics has always been an intriguing and gleaming topic of interest. Many efforts have been laid for understanding and identifying variants. Apart from the open source genomics efforts [@McLaren2016-jo;@Pedersen2023-co] in variant annotation, proprietary efforts have been developed for the identification and annotation of the variants. The identified variants have been classified and linked to genetic and metabolic disorders, with severity ranging from pathogenic to highly pathogenic. One such effort is the [eVai](https://www.engenome.com/product/), which is a high throughput variant annotator, and provides the hierarchical classification of the genomic variants. However, the limitation of the eVai is that it is a browser based software and hence lack high-throughput search and categorical and functional analysis of annotated variants from sequences associated with variants. eVaiutilities allows, you to use the output from the eVai and performs genome scale analysis of the eVai annotated variants. It allows you to search across populations, variants and spanning information across multiple samples. It is encoded in RUST, which is a memory safe language and provides scalability and uses RUST version: rustc 1.89.0.

# Figures
![Interface of eVaiutilities](eVaiutilities.png)

# eVaiutilities

Variant identification and annotation across the human genome has wide biological significance.  Many tools have been developed for the annotation of the variants that focuses on the annotation of the variants using the several pre-established database [@Wang2010-mh;@Obenchain2014-jj]. Apart from the significant development in the open source genomics [@McLaren2016-jo;@Pedersen2023-co], parallel efforts have been laid down for the variant identification and annotation in industrial research. [eVai](https://www.engenome.com/product/), provides customary analysis of variants by comparing the variants across the several tools and databases and provides TSV files with the annotations mapped. However, customary analysis is needed to make interpretation of identified variants easier. Additionally, for large scale variant searching, speed and scalability is needed. To address all these issues, eVaiutilities has been built using the RUST memory safe language features. eVaiutilities provides a compendium of options to allow for the searching of the variants across the population and generate customary analysis. It allows you to analyze variant annotation, summarize variant annotation and generate variant report and sequence annotation. eVaiutilities features such as the transcript sequence to sequence spanning around the variants, allow the ease of the access to generate the methodological approaches for the variants of interest and also allows for the categorical variant filtering. There are several features present in the eVaiutilities such as population wide analysis of a particular variant, coordinate search for variants across population, variant search across the specific coordinates with the specified variant across population, sequence specific analysis, reference and alternate allele search across the population. All the aforementioned features makes eVaiutilties an easy to use data management software for doing the customary analysis with the eVai. eVaiutilities is available from [eVaiutilitites](https://github.com/genomicssport/eVaiutilities) and is also available as a crate from https://crates.io/crates/eVaiutilities. 

# Acknowledgements

I thank Julia Romanowska for working on the text revisions of the paper, which has significantly improved the reading content of the eVaiutilties. 

# References
