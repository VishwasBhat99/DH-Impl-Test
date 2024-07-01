#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
/home/dbuser/programs/DB-Loader-Reciprocal-Lending/GC/Import/truncate.sh
master=$( ls $INPUT/GC/$1/reciprocal-lending.csv )
master_log=$"$LOGS/GC/$1/Rec_Lend_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_GC \
control=/home/dbuser/programs/DB-Loader-Reciprocal-Lending/GC/Import/rec_lend.ctl \
data=$master \
LOG=$master_log \
BAD=$LOGS/GC/$1/Rec_Lend_$timestamp.BAD

/home/dbuser/programs/GC/health-checker-w1l.sh \
$INPUT/GC/$1/DB-Loader-Rec-Lend.txt \
$master_log

