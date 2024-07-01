#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-INR-IRS/HK/Import/truncate.sh
timestamp=$( date +%d%m%Y_%H%M%S )
master=$"$INPUT/HK/$1/master_inrirs.csv"
master_log=$"$LOGS/HK/$1/INR_IRS_Master_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_HK \
data=$master \
control=/home/dbuser/programs/DB-Loader-INR-IRS/HK/Import/INR_IRS_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/HK/$1/INR_IRS_Master_$timestamp.BAD

/home/dbuser/programs/HK/health-checker-w1l.sh \
$INPUT/HK/$1/DB-Loader-INRIRS.txt \
$master_log


