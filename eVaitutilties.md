- eVaiutilties: Analyzing and identifying genomic variants is particularly helpful in case of linking the disease onset to the genomic predictions. Genomic variants play a key role in the identification and characterization of the disease onset and also allow for interlinking genomic variability and hypothesis testing. eVai, which is a variant annotation platform provides annotation of the genomic variants using several backhand open source databases and tools. eVaiutilities proposed in this paper, is a data management software for the analysis of the eVai output files. It allows the analysis of the genomic variants further such as analyzing the multiple genomic annotated variants, reference and alternate allele, enabling coordinate search, coordinate search with specified variants and annotation search across a large number of population. The availability of the command line parameters allows for a large scale analysis across the several population files.

- Below you will find a detailed usage of the eVaiutilities.

- The variant analyzer allows you to merge all the information present in the ACMG file and the TSV file for the annotate variants into a single file and removes the redundant information.This allows for a single file analysis.
```
Usage: eVaiutilities variant-analyzer <ACMGFILE> <TSVFILE>
Arguments:
  <ACMGFILE>  provide the path to the acmg file
  <TSVFILE>   provide the path to the tsv file

variant-analyzer ./testfiles/single-population/sample1.tsv ./single/sampletest.tsv
```

- This option allows for the filtering of the variants according to a defined variant and allows for the filtering of the variant from a single annotated variant. It takes a ACMG and TSV file and a variant which needs to be filtered.

```
Usage: eVaiutilities variant-filter <ACMGFILE> <TSVFILE> <VARIANT>
Arguments:
  <ACMGFILE>  provides the path to the acmg file
  <TSVFILE>   provides the path to the tsv file
  <VARIANT>   provide the variant

variant-filter ./testfiles/single-population/sample1.tsv ./single/sampletest.tsv TTT

```

- This option allows for the analysis of the corresponding GTF and gives you the corresponding annotated exons of the annotated genes.It collates all the exons present in the gene.
analyze the corresponding gtf

```
Usage: eVaiutilities gtf-analyze <GTFFILE>
Arguments:
  <GTFFILE>  path to the gtf file

gtf-analyze ./testfiles/single-population/samplegtf.gtf
```

- This gives you the annotated sequence associated with the variants given the fasta sequence.
prepare the variant seq annotation.

```
Usage: eVaiutilities variant-seq <ACMGFILE> <FASTAFILE>
Arguments:
  <ACMGFILE>   provide the ACMG file
  <FASTAFILE>  provide fasta file

variant-seq ./testfiles/single-population/sample1.tsv ./single/sample.fasta
```

- This option allows you to download the latest version of the genome from the GENCODE.

```
Usage: eVaiutilities download-genome <INPUT>
Arguments:
  <INPUT>
 download-genome yes
```

- This option gives you the associated transcript ids with the variants in the ACMG transcripts.

```
Usage: eVaiutilities acmg-transcript <ACMGFILE>
Arguments:
  <ACMGFILE>  provide the ACMG file
acmg-transcript ./testfiles/single-population/sample1.tsv
```

- This allows you to search for the variant and then extract the upstream and downstream information associated with the particular variant.

```
Usage: eVaiutilities sequence-profile <ACMGFILE> <FASTAFILE> <UPSTREAM> <DOWNSTREAM> <VARIANT>
Arguments:
  <ACMGFILE>    provide the acmg file
  <FASTAFILE>   provide the fasta file
  <UPSTREAM>    upstream location
  <DOWNSTREAM>  downstream location
  <VARIANT>     variant
sequence-profile ./testfiles/single-population/sample1.tsv ./data-repo/sample.fasta 10 10
```

- This allows you to search for a particular variant across a population. This takes a directory containing all the ACMG files correponding to a particular population.

```
Usage: eVaiutilities population-variant-search <ACMGDIR> <VARIANT> <NAME>
Arguments:
  <ACMGDIR>  provide the acmg directory
  <VARIANT>  provide the variant
  <NAME>     analysis name
population-variant-search ./testfiles/newversion-population G samplename
```

- This option allows you to search for a variant between the coordinates. In the above example, we provide a directory containing the ACMG files and then search for the variant between these coordinates across all those files and output a filename with the same samplename.

```
Usage: eVaiutilities coordinate-search <ACMGDIR> <START> <END> <NAME>
Arguments:
  <ACMGDIR>  provide the acmg directory
  <START>    provide the coordinate
  <END>      provide the end coordinate
  <NAME>     analysis name
coordinate-search ./testfiles/newversion-population 14464 16495 samplename
```

- This allows you to search for a variant with the specificed directory containing all the ACMG files and then associated with the specific transcript id across all those files.

```
Usage: eVaiutilities annotation-search <ACMGDIR> <GENENAME> <NAME>
Arguments:
  <ACMGDIR>   provide the acmg directory
  <GENENAME>  search the annotation
  <NAME>      analysis name
annotation-search ./testfiles/newversion-population AL645608.6 samplename
```

- This allows you to filter a directory with the ACMG files and you to have provide the range of the pathogenicity filter with the start and the end value and the sample name as the file to be written.

```
Usage: eVaiutilities pathogenicity-filter <ACMGDIR> <START> <END> <NAME>
Arguments:
  <ACMGDIR>  provide the acmg dir
  <START>    start value
  <END>      end value
  <NAME>     analysis name
pathogenicity-filter ./testfiles/newversion-population 0.0 1.5 samplename
```

- This allows you to search for a particular variant across a population. This takes a directory containing all the ACMG files correponding to a particular population. This option is for the older version of the eVai.

```
Usage: eVaiutilities population-variant-searcholder <ACMGDIR> <VARIANT> <NAME>
Arguments:
  <ACMGDIR>  provide the acmg directory
  <VARIANT>  provide the variant
  <NAME>     analysis name
population-variant-searcholder ./testfiles/oldversion-population G samplename
```

- This option allows you to search for a variant between the coordinates. In the above example, we provide a directory containing the ACMG files and then search for the variant between these coordinates across all those files and output a filename with the same samplename. This supports the older version of the eVai.

```
Usage: eVaiutilities coordinate-searcholder <ACMGDIR> <START> <END> <NAME>
Arguments:
  <ACMGDIR>  provide the acmg directory
  <START>    provide the coordinate
  <END>      provide the end coordinate
  <NAME>     analysis name
coordinate-searcholder ./testfiles/oldversion-population 14464 16495 samplename
```
- This allows you to search for a variant with the specificed directory containing all the ACMG files and then associated with the specific transcript id across all those files. This supports the older version of the eVai.

```
Usage: eVaiutilities annotation-searcholder <ACMGDIR> <GENENAME> <NAME>
Arguments:
  <ACMGDIR>   provide the acmg directory
  <GENENAME>  search the annotation
  <NAME>      analysis name
annotation-searcholder ./testfiles/oldversion-population WASH7P samplename
```

- This allows you to filter a directory with the ACMG files and you to have provide the range of the pathogenicity filter with the start and the end value and the sample name as the file to be written. This supports the older version.

```
Usage: eVaiutilities pathogenicity-filterolder <ACMGDIR> <START> <END> <NAME>
Arguments:
  <ACMGDIR>  provide the acmg dir
  <START>    start
  <END>      end
  <NAME>     analysis name
pathogenicity-filterolder ./testfiles/oldversion-population 0 1.5 samplename
```

- This option allows you to search for a specific transcript across the entire population and takes a ACMG folder and the transcript name and a ouptufile name.

```
Usage: eVaiutilities transcript-search <ACMGDIR> <TRANSCRIPT> <NAME>
Arguments:
  <ACMGDIR>     provide the acmg directory
  <TRANSCRIPT>  provide the transcript name
  <NAME>        name of the analysis
transcript-search ./testfiles/newversion-population ENST00000635509.2 samplename
```

- This option allows for the filtering of the entire population study and takes a directory containing ACMG files and a ref allele to look for and a output file name.
```
Usage: eVaiutilities alt-allele <ACMGDIR> <REFALLELE> <NAME>
Arguments:
  <ACMGDIR>    provide the acmg directory
  <REFALLELE>  provide the alt allele
  <NAME>       provide the analysis name
alt-allele ./testfiles/newversion-population T samplename
```

- This option allows for the filtering of the entire population study and takes a directory containing ACMG files and a ref allele to look for and a output file name. This supports the older version of the eVai.

```
Usage: eVaiutilities alt-allele-older <ACMGDIR> <REFALLELE> <NAME>
Arguments:
  <ACMGDIR>    provide the acmg directory
  <REFALLELE>  provide the alt allele
  <NAME>       provide the analysis name
alt-allele-older ./testfiles/oldversion-population T samplename
```

- This option allows you to search for a ref allele that has a specific alternate allele across a entire population and takes a directory containing ACMG files and the ref and the alternate allele to search for and an output filename to which the information should be written.

```
Usage: eVaiutilities alt-ref-allele <ACMGDIR> <REFALLELE> <ALTALLELE> <NAME>
Arguments:
  <ACMGDIR>    provide the acmd directory
  <REFALLELE>  provide the ref allele
  <ALTALLELE>  provide the alt allele
  <NAME>       provide the analysis name
 alt-ref-allele ./testfiles/newversion-population A T samplename
```

- This option allows you to search for a ref allele that has a specific alternate allele across a entire population and takes a directory containing ACMG files and the ref and the alternate allele to search for and an output filename to which the information should be written. This supports the older version.
```
Usage: eVaiutilities alt-ref-allele-older <ACMGDIR> <REFALLELE> <ALTALLELE> <NAME>
Arguments:
  <ACMGDIR>    provide the acmd directory
  <REFALLELE>  provide the ref allele
  <ALTALLELE>  provide the alt allele
  <NAME>       provide the analysis name
 alt-ref-allele-older ./testfiles/oldversion-population A T samplename
```

- This option allows you to plot all the variants across the entire population. It takes a folder containing the folder containing the annotated variants and then accumulate all the variants across that study and plots a bar plot to show the frequency of the variants across that study.

```
Usage: eVaiutilities variant-plotter <PATHFOLDER>
Arguments:
  <PATHFOLDER>  provide the path to the folder
variant-plotter ./testfiles/variant-accumulation
```
