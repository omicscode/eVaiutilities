# eVai-analyzer
 - eVai-analyzer for open genomics.
 - It analyzes the eVai output files for variant annotation to filtering, preparating user reports and database and restapi human genomics.
 - **new version coming with lots of new features**
 
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
  help                       Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version

 
 ibch/eVai-analyzer » ./target/debug/eVaianalyzer variant-analyzer ./data-repo/sample1.tsv ./data-repo/sampletest.tsv
 ibch/eVai-analyzer » ./target/debug/eVaianalyzer variant-filter ./data-repo/sample1.tsv ./data-repo/sampletest.tsv TTT
 ibch/eVai-analyzer » ./target/debug/eVaianalyzer analyzer ./data-repo/sample1.tsv ./data-repo/sampletest.tsv
 ibch/eVai-analyzer » ./target/debug/eVaianalyzer filter ./data-repo/sample1.tsv ./data-repo/sampletest.tsv TTT
 ibch/eVai-analyzer » ./target/debug/eVaianalyzer acmg-transcript ./data-repo/sample1.tsv 
 ibch/eVai-analyzer » ./target/debug/eVaianalyzer gtf-analyze ./data-repo/samplegtf.gtf
 ibch/eVai-analyzer » ./target/debug/eVaianalyzer download-genome yes
 ibch/eVai-analyzer » ./target/debug/eVaianalyzer sequence-profile ./data-repo/sample1.tsv ./data-repo/sample.fasta 10 10
 ibch/eVai-analyzer » ./target/debug/eVaianalyzer variant-seq ./data-repo/sample1.tsv ./data-repo/sample.fasta
 ```

 Gaurav Sablok \
 Instytut Chemii Bioorganicznej \
 Polskiej Akademii Nauk \
 ul. Noskowskiego 12/14 | 61-704, Poznań \
 Poland

