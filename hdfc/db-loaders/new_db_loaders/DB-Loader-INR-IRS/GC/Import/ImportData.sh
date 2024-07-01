#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-INR-IRS/GC/Import/truncate.sh
timestamp=$( date +%d%m%Y_%H%M%S )
master=$"$INPUT/GC/$1/master_inrirs.csv"
master_log=$"$LOGS/GC/$1/INR_IRS_Master_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_GC \
data=$master \
control=/home/dbuser/programs/DB-Loader-INR-IRS/GC/Import/INR_IRS_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/GC/$1/INR_IRS_Master_$timestamp.BAD

/home/dbuser/programs/GC/health-checker-w1l.sh \
$INPUT/GC/$1/DB-Loader-INRIRS.txt \
$master_log


