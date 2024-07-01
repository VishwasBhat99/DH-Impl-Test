#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
cashflows=$"$INPUT/HK/$1/cashflows_overseas.csv"
cf_log=$"$LOGS/HK/$1/Cashflow_Generic_$timestamp.log"
dos2unix $cashflows

sqlldr $CON_STR_HK \
data=$cashflows \
control=/home/dbuser/programs/DB-Loader-Cashflow/HK/Import/CASHFLOW_GENERIC.ctl \
LOG=$cf_log \
BAD=$LOGS/HK/$1/Cashflow_Generic_$timestamp.BAD

/home/dbuser/programs/HK/health-checker-w1l.sh \
$INPUT/HK/$1/DB-Loader-Cashflow.txt \
$cf_log


