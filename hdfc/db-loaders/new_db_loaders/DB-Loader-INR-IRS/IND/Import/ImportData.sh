#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-INR-IRS/IND/Import/truncate.sh
timestamp=$( date +%d%m%Y_%H%M%S )
master=$"$INPUT/IND/$1/master_inrirs.csv"
master_log=$"$LOGS/IND/$1/INR_IRS_Master_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_IND \
data=$master \
control=/home/dbuser/programs/DB-Loader-INR-IRS/IND/Import/INR_IRS_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/IND/$1/INR_IRS_Master_$timestamp.BAD 

/home/dbuser/programs/IND/health-checker-w1l.sh \
$INPUT/IND/$1/DB-Loader-INRIRS.txt \
$master_log 

