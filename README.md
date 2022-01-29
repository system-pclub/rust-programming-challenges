# The code, analysis scripts and results for ICSE 2022 Artifact Evaluation

Version: 1.1\
Update:  Jan 29, 2022\
Paper:   Learning and Programming Challenges of Rust: A Mixed-Methods Study (#1162)

This document is to help users reproduce the results we reported in our submission. 
It contains the following descriptions:

## 1. Artifact Expectation

The collected data and the scripts to analyze the data are released in this repository.
The detailed study results are released in two Spreadsheet documents. 
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
The detailed study results are summarized in the Google Doc
[empirical-study](https://docs.google.com/spreadsheets/d/1_uipSVvq0l8MLYN4XXqHP1hgcPp1wvNDnqJ4eu0GpZE/edit#gid=534399057). 
A copy `empirical-study.xlsx` is saved in this repository.

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

### 3.2 Which Safety Rules are Difficult?
The results of this section is presented in tab Section#3.2.
#### 3.2.1 Complex Lifetime Computation
There are three categories of complex lifetime computations: Intra-procedural Lifetime Computation, Inter-procedural Lifetime Computation, Simple Syntax Errors.
The information of which category each violation belongs to is listed in Columns G--I of tab Section#3.2.
Columns K--N shows the detailed sub-categories of violations under Intra-procedural Lifetime Computation.
Columns P--T shows the detailed sub-categories of violations under Inter-procedural Lifetime Computation.


#### 3.2.2 Violating Ownership Rules
There are two categories of ownership violations, Move Rule Violations and Borrowing Rule Violations.
The information of which category each violation belongs to is listed in Columns K--L of tab Section#3.2.
Columns Y--Z shows the detailed sub-categories of violations belonging to Move Rule Violations.
Columns AB--AH shows the detailed sub-categories of violations belonging to Borrowing Rule Violations.


#### 3.2.3 How Violations are Fixed? 
The category of how each violation is fixed is listed in Columns AJ--AM.

### 3.3 When a Safety Rule is More Confusing?
In this section, we applied the LDA model to identify potential situations where a safety rule is more confusing.
With the help of the LDA model, we identified the involved code constructs in the violations.

We then computed the lift metric on these code constructs and violations. The details of the LDA model and the lift metric
are listed below.

#### 3.3.1 LDA Model
We first identify Stack Overflow questions in the large dataset related to categories of "lifetime", "borrow" and "move", with the help of their tags. 

The number of questions under each tag (results in line 554 and 555 in the paper) are listed in Column A and B, tab Section#3.3.1.
Tags of different categories are highlighted with different colors, which are indicated in column D.

Next, we decide the topic numbers for the LDA model, by choosing the ones with the best coherence value (line 564-566 in the paper).
The results are presented in Column H--O.

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


#### 3.3.2 Lift Correlation
The categorization of each violation's related code construct is listed in tab Section#3.3.2.

We selected code constructs with more than ten questions in the small dataset, and presented their lift metrics in cells AB140--AH157 in tab Section#3.3.2.

The Python script to compute lift is located in `small-dataset/lift/lift.csv.py`.
Executing
```
cd small-dataset/lift
python3 lift.csv.py
``` 
shows the table in tab Section#3.3.2 on standard output.

### 3.4 Evaluating Compiler Error Messages
We first conducted a cognitive task analysis to identify the process of comprehending compile errors.
Then we identified whether the error message is helpful, and what information they lack.
#### 3.4.1 Cognitive Task Analysis

The interview protocol is located at `cognitive-task-analysis/interview.docx`.

After the interview, we summarized the process into a table, which is located at  `cognitive-task-analysis/outcome.xlsx`.

#### 3.4.2 Studying Violations
The results are presented in column AN:AO in tab Section#3.4.

# 4. Surveying Rust Programmers (Section 4 in the paper)
The data and results are summarized in a Google Doc [here](https://docs.google.com/spreadsheets/d/1QUN3NEk5zPHWS96cV--AM113ZHP18-9Z1_z5cV6B9b4/edit#gid=0). 
All tabs mentioned in this section are in this document, unless otherwise specified.
A copy of the document is saved in this repository named `user-study.xlsx`.

Some results are computed with R markdown documents.
Besides the copies in the repository, the Rmd (R Markdown) files are also available in the virtual machine.
Users can open the files in RStudio and click the "Knit" button from RStudio after opening the Rmd file to compute and view the results.

You may also install the necessary packages and execute them from a local machine, but the reproducibility is not guaranteed.

## 4.1 Methodology (The Survey)
We provided multiple formats for the survey:
1. Qualtrics Project file, located in `survey-project/Qualtrics_project.qsf`. This file can be imported to Qualtrics and can facilitate future works on surveying programmers.
2. A brief description of the survey, located in `survey-project/survey-description.pdf`.
3. An example survey response that has a similar appearance to the actual online survey: `survey-project/example-response.pdf`

Document
`survey-project/pc-pd-rubric.docx` contains PC, PD, their variants and enhanced error messages, and rubrics for Q6.

## 4.2 Survey Results

The responses are  in tab valid-responses.

### 4.2.1 Phase 1
####  Demographic Information 
The demographic information are presented in line 832-839 in the paper.
To protect participants' privacy, we cannot release their original answers. 
The statistical results are presented in 
`survey-results/phase1.pdf`.

#### Rust Experience
The results in line 846-859 are presented in the following document.
`survey-results/phase1-rust-questions.Rmd`
### 4.2.2 Phase 2
Discussions of PA and PB in line 857 to 861 are presented in tab phase-2-PA-PB.

ANOVA and Chi-Square Tests for Q1-Q6 are presented in 
`survey-results/phase2.Rmd`.

Lines 1007 to 1021 in the paper present the effect of enhanced error messages. ANOVA Tests for the effect of Enhanced Error Messages are presented in 
`survey-results/phase2-err-msg.Rmd`.

The results in Table 3 are presented in `survey-results/table3.Rmd`.

The related responses discussed in Line 917 to 927 in the paper are marked in column E in tab phase-2-PC.

The related responses discussed in Line in line 951 to 971 in the paper for PC are marked in column H in tab phase-2-PC.
They are further discussed in line 994-995, which are marked in column M.
For PD, they are marked in column G, tab phase-2-PD.

The five responses mentioned in line 998 are marked in column L, tab phase-2-PD.

### 4.2.3 Phase 3
To protect participants' privacy, we only present the statistical results in `survey-results/phase3.pdf`.

