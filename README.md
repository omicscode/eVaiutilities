# eVai-analyzer
 - eVai-analyzer for open genomics.
 - It analyzes the eVai output files for variant annotation to filtering, preparating user reports and database and annotator.

 ```
 cargo build
 ```

 ```
 variantannotator

Usage: eVaianalyzer <COMMAND>

Commands:
  variant-analyzer           analyzer for the variants
  variant-filter             filter the variants
  variant-database           create variant database
  gtf-analyze                analyze the corresponding gtf
  variant-seq                prepare the variant seq annotation
  download-genome            download the human genome
  acmg-transcript            variant-transcriptids
  sequence-profile           sequence profiling
  population-variant-search  search for the variant across population
  coordinate-search          search according to coordinates
  annotation-search          search according to annotation
  pathogenicity-filter       pathogenicity filter
  help                       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
 
 ibch/eVai-analyzer » eVaianalyzer variant-analyzer ./data-repo/sample1.tsv ./single/sampletest.tsv
 ibch/eVai-analyzer » eVaianalyzer variant-filter ./data-repo/sample1.tsv ./single/sampletest.tsv TTT
 ibch/eVai-analyzer » eVaianalyzer analyzer ./data-repo/sample1.tsv ./single/sampletest.tsv
 ibch/eVai-analyzer » eVaianalyzer filter ./data-repo/sample1.tsv ./single/sampletest.tsv TTT
 ibch/eVai-analyzer » eVaianalyzer acmg-transcript ./single/sample1.tsv 
 ibch/eVai-analyzer » eVaianalyzer gtf-analyze ./single/samplegtf.gtf
 ibch/eVai-analyzer » eVaianalyzer download-genome yes
 ibch/eVai-analyzer » eVaianalyzer sequence-profile ./single/sample1.tsv ./data-repo/sample.fasta 10 10
 ibch/eVai-analyzer » eVaianalyzer variant-seq ./single/sample1.tsv ./single/sample.fasta
 ibch/eVai-analyzer » eVaianalyzer population-variant-search ./population G samplename
 ibch/eVai-analyzer » eVaianalyzer coordinate-search ./population 14464 16495 samplename 
```

 Gaurav Sablok \
 Instytut Chemii Bioorganicznej \
 Polskiej Akademii Nauk \
 ul. Noskowskiego 12/14 | 61-704, Poznań \
 Poland

