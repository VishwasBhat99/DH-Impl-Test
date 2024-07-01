#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-FCY-IRS/HK/Import/truncate.sh
timestamp=$( date +%d%m%Y_%H%M%S )
master=$"$INPUT/HK/$1/master_fcyirs.csv"
master_log=$"$LOGS/HK/$1/FCY_IRS_MASTER_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_HK \
data=$master \
control=/home/dbuser/programs/DB-Loader-FCY-IRS/HK/Import/FCY_IRS_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/HK/$1/FCY_IRS_MASTER_$timestamp.BAD

/home/dbuser/programs/HK/health-checker-w1l.sh \
$INPUT/HK/$1/DB-Loader-FCYIRS.txt \
$master_log

