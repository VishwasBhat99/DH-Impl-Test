#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
/home/dbuser/programs/DB-Loader-Over-Loans/GC/Import/truncate.sh
master=$( ls $INPUT/GC/$1/master.csv )
cashflows=$( ls $INPUT/GC/$1/cashflows.csv )
master_log=$"$LOGS/GC/$1/Over_Loans_Master_$timestamp.log"
cf_log=$"$LOGS/GC/$1/Over_Loans_Cashflow_$timestamp.log"

sqlldr $CON_STR_GC \
data=$master \
control=/home/dbuser/programs/DB-Loader-Over-Loans/GC/Import/OVER_LOANS_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/GC/$1/Over_Loans_Master_$timestamp.BAD

sqlldr $CON_STR_GC \
data=$cashflows \
control=/home/dbuser/programs/DB-Loader-Over-Loans/GC/Import/OVER_LOANS_CASHFLOW.ctl \
LOG=$cf_log \
BAD=$LOGS/GC/$1/Over_Loans_Cashflow_$timestamp.BAD

/home/dbuser/programs/GC/health-checker-w2l.sh \
$INPUT/GC/$1/DB-Loader-Over-Loans.txt \
$master_log \
$cf_log

