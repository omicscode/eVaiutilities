# eVai-utilities
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
     **************************************************

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
 pathogenicity-filter newversion-population 1.5 samplename
 population-variant-searcholder oldversion-population G samplename
 coordinate-searcholder oldversion-population 14464 16495 samplename
 annotation-searcholder oldversion-population WASH7P samplename
 pathogenicity-filterolder oldversion-population 1.5 samplename
 transcript-search newversion-population ENST00000635509.2 samplename
```

 Gaurav Sablok \
 Instytut Chemii Bioorganicznej \
 Polskiej Akademii Nauk \
 ul. Noskowskiego 12/14 | 61-704, Poznań \
 Poland

