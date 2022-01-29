# The code, analysis scripts and results for ICSE 2022 Artifact Evaluation

Version: 1.1\
Update:  Jan 28, 2022\
Paper:   Learning and Programming Challenges of Rust: A Mixed-Methods Study (#1162)

This document is to help users reproduce the results we reported in our submission. 
It contains the following descriptions:

## 1. Artifact Expectation

The collected data and the scripts to analyze the data are released in this repository.
The detailed study results are released in a Excel document. 
Our scripts are implemented either in Python or in R. 
All Python scripts can be executed by Python 3.6 or a higher version. 
All R scripts can be executed by R 3.6 or a higher version.
We also prepare a VM image with pre-installed Python 
and R to ease the reproduction
of our data analytics. 
Users can open the image with VirtualBox 6.1 or a higher VirtualBox version. 

## 2. Artifact Overview

In our paper, we first empirically study Rust-related Stack Overflow questions
to understand the programming challenges of Rust. We then conduct an online
survey to validate the study results. 

For artifact evaluation, we release 
- (1) our submitted paper,
- (2) collected Stack Overflow questions and their detailed labels,
- (3) our designed survey, 
- (4) survey answers of real-world Rust programmers,
- (5) scripts that conduct statistical analysis, 
- and (6) intermediate analysis results.  

In the following sections, we will follow the paper content to explain 
how each number in our paper
is supported by this artifact. 


## 3. Studying Stack Overflow Questions (Section 3 of the paper)
The detailed study results are summarized in the Excel document located in
TODO of this repository.

[empirical-study](https://docs.google.com/spreadsheets/d/1_uipSVvq0l8MLYN4XXqHP1hgcPp1wvNDnqJ4eu0GpZE/edit#gid=534399057). 

All tabs mentioned in this section
are in this document, unless otherwise specified. 


### 3.1 Methodology
File `large-dataset/raw-data.json` contains the original HTML web pages
of the questions in the large dataset. 
File `large-dataset/raw-data-html-tag-removed.json`
contains the version where HTML tags are removed.

The information of the sampled 100 Rust-related questions in Section 3.1.1 of the paper 
is in tab Section#3.1.1.

The information of the sampled 100 questions of the small 
dataset (Section 3.1.2 of the paper)
is in Columns B--E of tab Section#3.2.
The original web pages of the questions
are saved in file `small-dataset/question-web-pages`. 
The code snippets to reproduce compiler errors 
are under directory `small-dataset/reproduced-code-snippets`.

### Section 3.2.1 Complex Lifetime Computation
There are three categories of complex lifetime computations: Intra-procedural Lifetime Computation, Inter-procedural Lifetime Computation, Simple Syntax Errors.
The information of which category each violation belongs to is listed in Columns G--I of tab Section#3.2.
Tab Section#3.2.1-Intra-procedural shows the detailed sub-categories of violations under Intra-procedural Lifetime Computation.
Tab Section#3.2.1-Inter-procedual shows the detailed sub-categories of violations under Inter-procedural Lifetime Computation.


### Section 3.2.2 Violating Ownership Rules
There are two categories of ownership violations, Move Rule Violations and Borrowing Rule Violations.
The information of which category each violation belongs to is listed in Columns K--L of tab Section#3.2.
Tab Section#3.2.2-Move shows the detailed sub-categories of violations belonging to Move Rule Violations.
Tab Section#3.2.2-Borrow shows the detailed sub-categories of violations belonging to Borrowing Rule Violations.


### Section 3.2.3 How Violations are Fixed? 
The category of how each violation is fixed is listed in Columns N--Q.

## Section 3.3 When a Safety Rule is More Confusing?
In this section, we applied the LDA model to identify potential situations where a safety rule is more confusing.
With the help of the LDA model, we identified the involved code constructs in the violations.
They are listed in tab Section#3.3.

We then computed the lift metric on these code constructs and violations. The details of the LDA model and the lift metric
are listed below.

### Section 3.3.1  LDA Model
We first identify Stack Overflow questions in the large dataset related to categories of "lifetime", "borrow" and "move", with the help of their tags. 

The number of questions under each tag (results in line 554 and 555 in the paper) are listed in tab Section#3.3-tag-count.
Tags of different categories are highlighted with different colors, which are indicated in column D.

Next, we decide the topic numbers for the LDA model, by choosing the ones with the best coherence value (line 564-566 in the paper).
The results are presented in tab Section#3.3-topic-number-coherence.

The results of each question category are presented in tab Section#3.3.1-LDA-lifetime, Section#3.3.1-LDA-borrow and Section#3.3.1-LDA-move.
The examples presented in line 569-572 are highlighted in each tab.

The Python scripts to compute the results are located in `large-dataset/lda/`.
There are three Python source code files, `so_lda_lifetime.py`, `so_lda_borrow.py`, and `so_lda_move.py`.

For example, executing 
```
cd large-dataset/lda/
python3 so_lda_lifetime.py
``` 
will generate files `lda_result_5_topics.txt` to `lda_result_30_topics.txt` in the same directory.
Then, using a shell command 
```
for x in {5..30}; do head -n 1 lda_result_${x}_topics.txt; done
```
can check the coherence value under each topic number.

It also generates files `top_topics_5_lifetime.csv` to `top_topics_30_lifetime.csv`.
The result of `top_topics_5_lifetime.csv` corresponds to tab Section#3.3.1-LDA-lifetime.


### Section 3.3.2
We selected code constructs with more than ten questions in the small dataset, and presented their lift metrics in tab Section#3.3.2.

The Python script is located in `small-dataset/lift/lift.csv.py`.
Executing
```
cd small-dataset/lift
python3 lift.csv.py
``` 
shows the table in tab Section#3.3.2 on standard output.

## Section 3.4

### Cognitive Task Analysis
Interview Protocol: `icse2022-ae20/cognitive-task-analysis/interview.docx`

Outcome: `icse2022-ae20/cognitive-task-analysis/outcome.xlsx`

### Results
See column AN:AO in sheet "Section 3.2" and sheet "Section 3.4".

# Section 4
## The Survey
Qualtrics Project file: `icse2022-ae20/survey-project/Qualtrics_project.qsf` (This file can be imported to Qualtrics)

Survey Description: `icse2022-ae20/survey-project/survey-description.pdf`

PC, PD and their variants, with rubrics for Q6:
`icse2022-ae20/survey-project/pc-pd-rubric.docx`

## Participants' Responses

https://docs.google.com/spreadsheets/d/1QUN3NEk5zPHWS96cV--AM113ZHP18-9Z1_z5cV6B9b4/edit#gid=0

## Statistical Results

### Demographic Information
To protect participants' privacy, we only present the statistical results.
`icse2022-ae20/survey-results/phase1.pdf`

### Section 4.2.1
https://colab.research.google.com/drive/1dEUSHeLGEV3b6oFAkjnb3wPcautIb44I?usp=sharing#scrollTo=TBxxu1U5WPxs

### Section 4.2.2 
The Rmd (R Markdown) files are available both in the virtual machine and the RStudio Cloud project here:
https://rstudio.cloud/project/3466569
To execute and view the files, click the "Knit" button from RStudio after opening the Rmd file.
You may also install package "ez" and execute them from a local machine, but the reproduction of the results are not
guaranteed.
#### Statistical tests
`icse2022-ae20/survey-results/phase2.Rmd` and `icse2022-ae20/survey-results/phase2-err-msg.Rmd`.
#### Table 3
`icse2022-ae20/survey-results/table3.Rmd`.
#### Q1 selecting tuples in PC
See sheet "phase 2 PC - Q1 selected tuple". 
Corresponding answers are highlighted.
#### Q3 wrong choices
See sheet "phase 2 PC - Q3 wrong choices" and "phase 2 PD - Q3 wrong choices".
Corresponding answers are highlighted.
#### Q6 wrong understanding
See sheet "phase 2 PD1 - Q6 wrong understanding".
Corresponding answers are highlighted.
### Section 4.2.3 
To protect participants' privacy, we only present the statistical results.
`icse2022-ae20/survey-results/phase3.pdf`

