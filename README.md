# eVaiutilities: 
Analyzing and identifying genomic variants is particularly helpful in case of linking the disease onset to the genomic predictions. Genomic variants play a key role in the identification and characterization of the disease onset and also allow for interlinking genomic variability and hypothesis testing. eVai, which is a variant annotation platform provides annotation of the genomic variants using several backhand open source databases and tools. eVaiutilities proposed in this paper, is a data management software for the analysis of the eVai output files. It allows the analysis of the genomic variants further such as analyzing the multiple genomic annotated variants, reference and alternate allele, enabling coordinate search, coordinate search with specified variants and annotation search across a large number of population. The availability of the command line parameters allows for a large scale analysis across the several population files. 

- eVai-utilities for data management of the eVai results. It supports all versions of eVai.
- It analyzes the eVai output files for variant annotation to filtering, preparating user reports and database and annotator and more. See the example folder for the older and the newer version files.
- It provides the population scale variant searching given a folder according to the variant, annotation, transcript, gene, and coordinates.

```
 cargo build
```

```
 annotating and analyzing eVai results.

      ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      Funded through Development of a universal fast-response platform,
      based on RNA technology,ensuring the national drug and epidemiological safety.
      2021/ABM/05/00004-00 to
      Prof. Luiza Handschuh: luizahan@ibch.poznan.pl.
      *************************************************

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
  help                            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

```
 variant-analyzer ./data-repo/sample1.tsv ./single/sampletest.tsv
 variant-filter ./data-repo/sample1.tsv ./single/sampletest.tsv TTT
 gtf-analyze ./single/samplegtf.gtf
 variant-seq ./single/sample1.tsv ./single/sample.fasta
 download-genome yes
 acmg-transcript ./single/sample1.tsv
 sequence-profile ./single/sample1.tsv ./data-repo/sample.fasta 10 10
 population-variant-search newversion-population G samplename
 coordinate-search newversion-population 14464 16495 samplename 
 annotation-search newversion-population AL645608.6 samplename
 pathogenicity-filter newversion-population 0.0 1.5 samplename
 population-variant-searcholder oldversion-population G samplename
 coordinate-searcholder oldversion-population 14464 16495 samplename
 annotation-searcholder oldversion-population WASH7P samplename
 pathogenicity-filterolder oldversion-population 0 1.5 samplename
 transcript-search newversion-population ENST00000635509.2 samplename
 alt-allele newversion-population T samplename
 alt-allele-older oldversion-population T samplename
 alt-ref-allele newversion-population A T samplename
 alt-ref-allele-older oldversion-population A T samplename
``
<img src=https://fundacja.ibch.poznan.pl/wp-content/uploads/2020/06/logo.png alt="alt text">`

 - Acknowledgements: MOSAIC platform, developed as part of the ECBiG-MOSAIC project (POIR.04.02.00-00-D017/20), co-financed by the European Regional Development Fund (ERDF) under the Smart Growth Operational Programme 2014-2020, Measure 4.2 for the development of modern research infrastructure in the science sector. 
 - Project PI and Informal queries: Prof. Luiza Handschuh: luizahan@ibch.poznan.pl.
 - eVaiultities code related queries: Dr. Gaurav Sablok: gsablok@ibch.poznan.pl.

 Gaurav Sablok \
 Instytut Chemii Bioorganicznej \
 Polskiej Akademii Nauk \
 ul. Noskowskiego 12/14 | 61-704, Poznań \
 Poland

