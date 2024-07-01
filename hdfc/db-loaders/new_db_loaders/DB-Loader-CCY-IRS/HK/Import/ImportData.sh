#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-CCY-IRS/HK/Import/truncate.sh
timestamp=$( date +%d%m%Y_%H%M%S )
master_log=$"$LOGS/HK/$1/CCY_IRS_Master_$timestamp.log"
master=$"$INPUT/HK/$1/master_ccirs.csv"
dos2unix $master

sqlldr $CON_STR_HK \
data=$master \
control=/home/dbuser/programs/DB-Loader-CCY-IRS/HK/Import/CCY_IRS_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/HK/$1/CCY_IRS_Master_$timestamp.BAD

/home/dbuser/programs/HK/health-checker-w1l.sh \
$INPUT/HK/$1/DB-Loader-CCYIRS.txt \
$master_log

