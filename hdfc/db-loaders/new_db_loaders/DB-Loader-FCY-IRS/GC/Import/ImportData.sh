#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-FCY-IRS/GC/Import/truncate.sh
timestamp=$( date +%d%m%Y_%H%M%S )
master=$"$INPUT/GC/$1/master_fcyirs.csv"
master_log=$"$LOGS/GC/$1/FCY_IRS_MASTER_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_GC \
data=$master \
control=/home/dbuser/programs/DB-Loader-FCY-IRS/GC/Import/FCY_IRS_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/GC/$1/FCY_IRS_MASTER_$timestamp.BAD

/home/dbuser/programs/GC/health-checker-w1l.sh \
$INPUT/GC/$1/DB-Loader-FCYIRS.txt \
$master_log

