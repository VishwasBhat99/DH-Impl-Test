#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
/home/dbuser/programs/DB-Loader-Reciprocal-Lending/BAH/Import/truncate.sh
master=$( ls $INPUT/BAH/$1/reciprocal-lending.csv )
master_log=$"$LOGS/BAH/$1/Rec_Lend_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_BH \
control=/home/dbuser/programs/DB-Loader-Reciprocal-Lending/BAH/Import/rec_lend.ctl \
data=$master \
LOG=$master_log \
BAD=$LOGS/BAH/$1/Rec_Lend_$timestamp.BAD

/home/dbuser/programs/BAH/health-checker-w1l.sh \
$INPUT/BAH/$1/DB-Loader-Rec-Lend.txt \
$master_log

