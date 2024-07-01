#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
cashflows=$"$INPUT/IND/$1/cashflows_overseas.csv"
cf_log=$"$LOGS/IND/$1/Cashflow_Generic_$timestamp.log"
dos2unix $cashflows

sqlldr $CON_STR_IND \
data=$cashflows \
control=/home/dbuser/programs/DB-Loader-Cashflow/IND/Import/CASHFLOW_GENERIC.ctl \
LOG=$cf_log \
BAD=$LOGS/IND/$1/Cashflow_Generic_$timestamp.BAD 

/home/dbuser/programs/IND/health-checker-w1l.sh \
$INPUT/IND/$1/DB-Loader-Cashflow.txt \
$cf_log

