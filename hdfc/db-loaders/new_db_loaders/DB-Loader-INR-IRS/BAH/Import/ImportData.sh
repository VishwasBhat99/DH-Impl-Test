#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-INR-IRS/BAH/Import/truncate.sh
timestamp=$( date +%d%m%Y_%H%M%S )
master=$"$INPUT/BAH/$1/master_inrirs.csv"
master_log=$"$LOGS/BAH/$1/INR_IRS_Master_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_BH \
data=$master \
control=/home/dbuser/programs/DB-Loader-INR-IRS/BAH/Import/INR_IRS_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/BAH/$1/INR_IRS_Master_$timestamp.BAD

/home/dbuser/programs/BAH/health-checker-w1l.sh \
$INPUT/BAH/$1/DB-Loader-INRIRS.txt \
$master_log


