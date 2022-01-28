# The code, analysis scripts and results for ASPLOS 2022 Artifact Evaluation

Version: 1.1\
Update:  Jan 28, 2022\
Paper:   Learning and Programming Challenges of Rust: A Mixed-Methods Study

This document is to help users reproduce the results we reported in our submission. 
It contains the following descriptions:

## 0. Artifact Expectation

The collected data and the scripts of our paper are released in this repository.
The detailed study results are released in an excel file. 
Our scripts are implemented either in Python or in R. 
All Python scripts can be exercised by Python-3.6 or a higher version. 
All R scripts can be exercised by R-3.6 or a higher version.
We also prepare a virtual machine image with pre-installed Python 
and R to ease the reproduction
of our data analytics. 
Users can open the image with VirtualBox 6.1 or a higher VirtualBox version. 

## 1. Artifact Overview

In our paper, we first empirically study Rust-related Stack Overflow questions
to understand the programming challenges of Rust. We then conduct an online
survey to validate the study results. 

For artifact evaluation, we release 
- (1) our submitted paper,
- (2) collected Stack Overflow questions and their detailed labels,
- (3) our designed survey, 
- (4) survey answers from real-world Rust programmers,
- (5) scripts that conduct statistical analysis, 
- and (6) intermediate analysis results.  

In the following sections, we will follow the paper content to explain 
how each number in our paper
is supported by this artifact. 


## 2. Section 3
The empirical study results is presented in the google document below:
https://docs.google.com/spreadsheets/d/1_uipSVvq0l8MLYN4XXqHP1hgcPp1wvNDnqJ4eu0GpZE/edit#gid=534399057

We call this document as "Empirical Table" in the rest of the document.

## Section 3.1
### The large dataset
Original web page content:
`icse2022-ae20/large-dataset/raw-data.json`

After removing HTML tags:
`icse2022-ae20/large-dataset/raw-data-html-tag-removed.json`
#### The sample 100 questions and results from the large dataset
See sheet "Section 3.1.1 sampled 100 questions" in the empirical table.

### The small dataset
Copy of the original web pages:
`icse2022-ae20/small-dataset/question-web-pages`

Reproduced Rust code snippets:

`icse2022-ae20/small-dataset/reproduced-code-snippets`



Violation count in Section 3.1.2:

See D126:E134 in sheet "Section 3.2" of "Empirical Table".


## Section 3.2
### Section 3.2.1
In "Empirical Table", see G:I in sheet "Section 3.2", sheet "Section 3.2.1 - Intra-procedural" and sheet "Section 3.2.1 - Inter-procedual".

### Section 3.2.2
In "Empirical Table", see K:L in sheet "Section 3.2", and sheet "Section 3.2.2 - Move Rule".

### Section 3.2.3
In "Empirical Table", see N:Q in sheet "Section 3.2", and sheet "Section 3.2.2 - Borrowing Rule".

## Section 3.3

### Section 3.3.1
"We find 790, 907, and 28 questions respectively for these rule
groups in the large dataset":
https://docs.google.com/spreadsheets/d/1u0hVmvPmwQ6ExT1kshX_m_xuePRKZfovZFwmd7S_vKw/edit#gid=0

The LDA results are presented in sheet 'Section 3.3.1 LDA-"lifetime"',
'Section 3.3.1 LDA-"borrow"', and 'Section 3.3.1 LDA-"move"'.
Each sheet uses topic numbers 28, 6, and 9 respectively.

"For example, 415 questions contain the topic about how
to use lifetime annotations in a struct or a trait, seven questions
contain the topic about how to borrow elements from a collection or
a container, and four questions are about how to move an object to a
function or a closure."

See the cells marked in green. The 415 questions of lifetime annotations
are summarized by using topic number = 8.


### Section 3.3.2
Results: sheet "Section 3.3.2".


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

