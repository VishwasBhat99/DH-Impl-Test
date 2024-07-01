#!/usr/bin/env bash

file=$"../../repo-rev.csv"
dos2unix $file

sqlldr $CON_STR_BALMUSR \
control=./repo_rev_repo.ctl \
data=$file \
LOG=../../log-files/Repo_Rev.log \
BAD=../../log-files/Repo_Rev.BAD
