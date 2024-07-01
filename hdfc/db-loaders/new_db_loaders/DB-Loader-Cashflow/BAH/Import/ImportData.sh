#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
cashflows=$"$INPUT/BAH/$1/cashflows_overseas.csv"
cf_log=$"$LOGS/BAH/$1/Cashflow_Generic_$timestamp.log"
dos2unix $cashflows

sqlldr $CON_STR_BH \
data=$cashflows \
control=/home/dbuser/programs/DB-Loader-Cashflow/BAH/Import/CASHFLOW_GENERIC.ctl \
LOG=$cf_log \
BAD=$LOGS/BAH/$1/Cashflow_Generic_$timestamp.BAD

/home/dbuser/programs/BAH/health-checker-w1l.sh \
$INPUT/BAH/$1/DB-Loader-Cashflow.txt \
$cf_log


