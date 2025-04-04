# eVai-utilities
 - eVai-utilities for open genomics.
 - It supports all versions of eVai.
 - It analyzes the eVai output files for variant annotation to filtering, preparating user reports and database and annotator and more.
 - It analyses the data from both older and newer version. See the example folder for the older and the newer version files.
 - It provides the population scale variant searching given a folder according to the variant, annotation, transcript, gene, and coordinates.
 - In a new release you will have Fst, population maps and all integrated in the single RUST code.

**NEW VERSION COMING NEXT WEEK STABLE RELEASE**.

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
  annotation-search               search according to annotation
  pathogenicity-filter            pathogenicity filter
  population-variant-searcholder  search for the variant across population older version
  coordinate-searcholder          search according to coordinates older version
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
 ibch/eVai-analyzer » eVaianalyzer variant-analyzer ./data-repo/sample1.tsv ./single/sampletest.tsv
 ibch/eVai-analyzer » eVaianalyzer variant-filter ./data-repo/sample1.tsv ./single/sampletest.tsv TTT
 ibch/eVai-analyzer » eVaianalyzer gtf-analyze ./single/samplegtf.gtf
 ibch/eVai-analyzer » eVaianalyzer variant-seq ./single/sample1.tsv ./single/sample.fasta
 ibch/eVai-analyzer » eVaianalyzer download-genome yes
 ibch/eVai-analyzer » eVaianalyzer acmg-transcript ./single/sample1.tsv
 ibch/eVai-analyzer » eVaianalyzer sequence-profile ./single/sample1.tsv ./data-repo/sample.fasta 10 10
 ibch/eVai-analyzer » eVaianalyzer population-variant-search ./newversion-population G samplename
 ibch/eVai-analyzer » eVaianalyzer coordinate-search ./newversion-population 14464 16495 samplename 
 ibch/eVai-analyzer » eVaianalyzer annotation-search ./newversion-population AL645608.6 samplename
 ibch/eVai-analyzer » eVaianalyzer pathogenicity-filter ./newversion-population 1.5 samplename
 ibch/eVai-analyzer » eVaianalyzer population-variant-search ./oldversion-population G samplename 
 ibch/eVai-analyzer » eVaianalyzer coordinate-search ./oldversion-population 14464 16495 samplename
 ibch/eVai-analyzer » eVaianalyzer annotation-search ./oldversion-population WASH7P samplename
 ibch/eVai-analyzer » eVaianalyzer pathogenicity-filterolder ./oldversion-population 1.5 samplename
 ibch/eVai-analyzer » transcript-search ./newversion-population ENST00000635509.2 samplename
```

 Gaurav Sablok \
 Instytut Chemii Bioorganicznej \
 Polskiej Akademii Nauk \
 ul. Noskowskiego 12/14 | 61-704, Poznań \
 Poland

