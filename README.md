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
```

 - Acknowledgements: Funded through Development of a universal fast-response platform, based on RNA technology,ensuring the national drug and epidemiological safety. 2021/ABM/05/00004-00
 - Project PI and Informal queries: Prof. Luiza Handschuh: luizahan@ibch.poznan.pl.
 - eVaiultities code related queries: Dr. Gaurav Sablok: gsablok@ibch.poznan.pl.

 Gaurav Sablok \
 Instytut Chemii Bioorganicznej \
 Polskiej Akademii Nauk \
 ul. Noskowskiego 12/14 | 61-704, Poznań \
 Poland

