# phyloevolve

 - rust based alignment kit for genome, metagenome, pangenome. 
- It gives you all the alignments functionalities and also gives you colour coded alignment for each alignment 
     and sub-alignment region. check the alignment view and alignment-clipview.

 ```
 cargo build
 ```
 ```
 _               _           _____  __     __   ___    _      __     __  _____
_ __   | |__    _   _  | |   ___   | ____| \ \   / /  / _ \  | |     \ \   / / | ____|
| '_ \  | '_ \  | | | | | |  / _ \  |  _|    \ \ / /  | | | | | |      \ \ / /  |  _|
| |_) | | | | | | |_| | | | | (_) | | |___    \ V /   | |_| | | |___    \ V /   | |___
| .__/  |_| |_|  \__, | |_|  \___/  |_____|    \_/     \___/  |_____|    \_/    |_____|
|_|              |___/

rust-phylogenomicstools
************************************************
Author Gaurav Sablok,
Email: codeprog@icloud.com
************************************************

Usage: phyloEvolve <COMMAND>

Commands:
alignmerge          merge all the alignment into a single string
alignmergeinterval  alignmerge the specific region of the alignment
same-alignment      remove same alignment
alignmentstats      alignmentstats
filter-site         filter the sites with the given base
filter-all          removes same bases across all the alignment
filter-block        collineratiy block based alignment filtering
alignment-view      allows for the view of the alignment
alignment-clipview  allows for the embedded view of the clipped Alignment
sitereplace         replace the specific sites based on the site probability
protein-stat        estimate the protein stats
indelreplace        indel substituter
motif-search        search for the specific protein and the nucleotide motifs
up-down-stream      specific part of the alignment and the upstream and the downstream of alignment
site-alignment      extract the places of the specified base for LD analysis
plotter             frequency plot for the alignments
nucleotidecolour    color coded map
proteomecolour      proteome heatmap
help                Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version
 ```

  Gaurav Sablok \
  codeprog@icloud.com
