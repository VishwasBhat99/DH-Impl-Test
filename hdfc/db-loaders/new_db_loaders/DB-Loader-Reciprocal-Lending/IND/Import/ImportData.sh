#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
/home/dbuser/programs/DB-Loader-Reciprocal-Lending/IND/Import/truncate.sh
master=$( ls $INPUT/IND/$1/reciprocal-lending.csv )
master_log=$"$LOGS/IND/$1/Rec_Lend_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_IND \
control=/home/dbuser/programs/DB-Loader-Reciprocal-Lending/IND/Import/rec_lend.ctl \
data=$master \
LOG=$master_log \
BAD=$LOGS/IND/$1/Rec_Lend_$timestamp.BAD

/home/dbuser/programs/IND/health-checker-w1l.sh \
$INPUT/IND/$1/DB-Loader-Rec-Lend.txt \
$master_log 
