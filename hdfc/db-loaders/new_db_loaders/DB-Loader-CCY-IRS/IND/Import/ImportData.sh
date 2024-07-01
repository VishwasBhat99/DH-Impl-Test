#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-CCY-IRS/IND/Import/truncate.sh
timestamp=$( date +%d%m%Y_%H%M%S )
master_log=$"$LOGS/IND/$1/CCY_IRS_Master_$timestamp.log"
master=$"$INPUT/IND/$1/master_ccirs.csv"
dos2unix $master

sqlldr $CON_STR_IND \
data=$master \
control=/home/dbuser/programs/DB-Loader-CCY-IRS/IND/Import/CCY_IRS_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/IND/$1/CCY_IRS_Master_$timestamp.BAD

/home/dbuser/programs/IND/health-checker-w1l.sh \
$INPUT/IND/$1/DB-Loader-CCYIRS.txt \
$master_log 
