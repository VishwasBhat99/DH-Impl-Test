#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
/home/dbuser/programs/DB-Loader-Reciprocal-Lending/HK/Import/truncate.sh
master=$( ls $INPUT/HK/$1/reciprocal-lending.csv )
master_log=$"$LOGS/HK/$1/Rec_Lend_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_HK \
control=/home/dbuser/programs/DB-Loader-Reciprocal-Lending/HK/Import/rec_lend.ctl \
data=$master \
LOG=$master_log \
BAD=$LOGS/HK/$1/Rec_Lend_$timestamp.BAD

/home/dbuser/programs/HK/health-checker-w1l.sh \
$INPUT/HK/$1/DB-Loader-Rec-Lend.txt \
$master_log

