# eVaiutilities:

![](https://github.com/IBCHgenomic/eVaiutilities/blob/main/logo.png)

Analyzing and identifying genomic variants is particularly helpful in case of linking the disease onset to the genomic predictions. Genomic variants play a key role in the identification and characterization of the disease onset and also allow for interlinking genomic variability and hypothesis testing. eVai, which is a variant annotation platform provides annotation of the genomic variants using several backhand open source databases and tools. eVaiutilities proposed in this paper, is a data management software for the analysis of the eVai output files. It allows the analysis of the genomic variants further such as analyzing the multiple genomic annotated variants, reference and alternate allele, enabling coordinate search, coordinate search with specified variants and annotation search across a large number of population. The availability of the command line parameters allows for a large scale analysis across the several population files.

# eVaiutilities:

Analyzing and identifying genomic variants is particularly helpful in case of linking the disease onset to the genomic predictions. Genomic variants play a key role in the identifcation and characterization of the disease onset and also allow for interlinking genomic variability and hypothesis testing. eVai, which is a variant annotation platform provides annotation of the genomic variants using several backhand open source databases and tools. eVaiutilities proposed in this paper, is a data management software for the analysis of the eVai output files. It allows the analysis of the genomic variants further such as analyzing the multiple genomic anntoated variants, reference and alternate allele, enabling cocordinate search, coordinate search with specificed variants and annotation search across a large number of population. The availability of the command line parameters allows for a large scale analysis across the several population files.

- eVai-utilities for data management of the eVai results. It supports all versions of eVai.
- It analyzes the eVai output files for variant annotation to filtering, preparating user reports and database and annotator and more. See the example folder for the older and the newer version files.
- It provides the population scale variant searching given a folder according to the variant, annotation, transcript, gene, and coordinates.

-- added a conda version for the conda users.

```
git clone https://github.com/IBCHgenomic/eVaiutilities.git
cd eVaiutilities-conda-version
conda env create -n rusteVai && conda install -n rusteVai -f ../condaeVaiutilities.yaml
conda activate rusteVai
cargo build
./target/debug/eVaiutilities
```

cargo build

```

```

annotating and analyzing eVai results.

```
  ************************************************
  Gaurav Sablok, IBCH, PAN, Poznan, Poland,
  https://portal.ichb.pl/laboratory-of-genomics/.
  Email: gsablok@ibch.poznan.pl
  Prof. Luiza Handschuh
  Email: luizahan@ibch.poznan.pl.
  *************************************************
```

Usage: eVaiutilities <COMMAND>

Commands:variant-analyzer                analyzer for the variantsvariant-filter                  filter the variantsvariant-database                create variant databasegtf-analyze                     analyze the corresponding gtfvariant-seq                     prepare the variant seq annotationdownload-genome                 download the human genomeacmg-transcript                 variant-transcriptidssequence-profile                sequence profilingpopulation-variant-search       search for the variant across populationcoordinate-search               search according to coordinatescoordinate-search-variant       search according to coordinates and variantannotation-search               search according to annotationpathogenicity-filter            pathogenicity filterpopulation-variant-searcholder  search for the variant across population older versioncoordinate-searcholder          search according to coordinates older versioncoordinate-searc-variantholder  search according to coordinates and variant older versionannotation-searcholder          search according to annotation older versionpathogenicity-filterolder       pathogenicity filter older versiontranscript-search               search by the specific transcripttranscript-searcholder          search by the specific transcript olderalt-allele                      search for the ref allelealt-ref-allele                  search according to ref allele and alt allelealt-allele-older                search for the ref allele older versionalt-ref-allele-older            search according to ref allele and alt allele older versionhelp                            Print this message or the help of the given subcommand(s)

Options:-h, --help     Print help-V, --version  Print version

```

```

variant-analyzer ./data-repo/sample1.tsv ./single/sampletest.tsvvariant-filter ./data-repo/sample1.tsv ./single/sampletest.tsv TTTgtf-analyze ./single/samplegtf.gtfvariant-seq ./single/sample1.tsv ./single/sample.fastadownload-genome yesacmg-transcript ./single/sample1.tsvsequence-profile ./single/sample1.tsv ./data-repo/sample.fasta 10 10population-variant-search newversion-population G samplenamecoordinate-search newversion-population 14464 16495 samplenameannotation-search newversion-population AL645608.6 samplenamepathogenicity-filter newversion-population 0.0 1.5 samplenamepopulation-variant-searcholder oldversion-population G samplenamecoordinate-searcholder oldversion-population 14464 16495 samplenameannotation-searcholder oldversion-population WASH7P samplenamepathogenicity-filterolder oldversion-population 0 1.5 samplenametranscript-search newversion-population ENST00000635509.2 samplenamealt-allele newversion-population T samplenamealt-allele-older oldversion-population T samplenamealt-ref-allele newversion-population A T samplenamealt-ref-allele-older oldversion-population A T samplename

```
- Acknowledgements: MOSAIC platform, developed as part of the ECBiG-MOSAIC project (POIR.04.02.00-00-D017/20), co-financed by the European Regional Development Fund (ERDF) under the Smart Growth Operational Programme 2014-2020, Measure 4.2 for the development of modern research infrastructure in the science sector.
- Project PI and Informal queries: Prof. Luiza Handschuh: luizahan@ibch.poznan.pl.
- eVaiultities code related queries: Dr. Gaurav Sablok: gsablok@ibch.poznan.pl.

Gaurav Sablok Instytut Chemii Bioorganicznej Polskiej Akademii Nauk ul. Noskowskiego 12/14 | 61-704, Poznań Poland
```
