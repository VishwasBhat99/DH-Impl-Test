#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
cashflows=$"$INPUT/GC/$1/cashflows_overseas.csv"
cf_log=$"$LOGS/GC/$1/Cashflow_Generic_$timestamp.log"
dos2unix $cashflows

sqlldr $CON_STR_GC \
data=$cashflows \
control=/home/dbuser/programs/DB-Loader-Cashflow/GC/Import/CASHFLOW_GENERIC.ctl \
LOG=$cf_log \
BAD=$LOGS/GC/$1/Cashflow_Generic_$timestamp.BAD

/home/dbuser/programs/GC/health-checker-w1l.sh \
$INPUT/GC/$1/DB-Loader-Cashflow.txt \
$cf_log


