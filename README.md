# eVai-analyzer
 - eVaianalyzer for open genomics
 - from variant annotation to filtering, preparating user reports and database and restapi human genomics
 
 ```
 cargo build
 ```

 ```
 variantannotator

 Usage: eVaianalyzer <COMMAND>

 Commands:
  analyzer         analyzer for the variants
  filter           filter the variants
  database         create variant database
  gtf-analyze      analyze the corresponding gtf
  variant-seq      prepare the variant seq annotation
  download-genome  download the human genome
  acmg-transcript  variant-transcriptids
  help             Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version


 ```

 - variantfiler 
 
 ```
 ibch/eVai-analyzer » ./target/debug/eVaianalyzer analyzer ./data-repo/sample1.tsv ./data-repo/sampletest.tsv
 ibch/eVai-analyzer » ./target/debug/eVaianalyzer filter ./data-repo/sample1.tsv ./data-repo/sampletest.tsv TTT
 ibch/eVai-analyzer » ./target/debug/eVaianalyzer acmg-transcript ./data-repo/sample1.tsv 
 ibch/eVai-analyzer » ./target/debug/eVaianalyzer gtf-analyze ./data-repo/samplegtf.gtf
 ibch/eVai-analyzer » ./target/debug/eVaianalyzer download-genome yes
 ```

 Gaurav Sablok \
 Instytut Chemii Bioorganicznej \
 Polskiej Akademii Nauk \
 ul. Noskowskiego 12/14 | 61-704, Poznań \
 Poland


