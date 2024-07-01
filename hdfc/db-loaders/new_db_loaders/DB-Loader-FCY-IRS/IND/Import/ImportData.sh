#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-FCY-IRS/IND/Import/truncate.sh
timestamp=$( date +%d%m%Y_%H%M%S )
master=$"$INPUT/IND/$1/master_fcyirs.csv"
master_log=$"$LOGS/IND/$1/FCY_IRS_MASTER_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_IND \
data=$master \
control=/home/dbuser/programs/DB-Loader-FCY-IRS/IND/Import/FCY_IRS_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/IND/$1/FCY_IRS_MASTER_$timestamp.BAD 

/home/dbuser/programs/IND/health-checker-w1l.sh \
$INPUT/IND/$1/DB-Loader-FCYIRS.txt \
$master_log 

