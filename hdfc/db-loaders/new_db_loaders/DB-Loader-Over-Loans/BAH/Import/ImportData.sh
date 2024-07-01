#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
/home/dbuser/programs/DB-Loader-Over-Loans/BAH/Import/truncate.sh
master=$( ls $INPUT/BAH/$1/master.csv )
cashflows=$( ls $INPUT/BAH/$1/cashflows.csv )
master_log=$"$LOGS/BAH/$1/Over_Loans_Master_$timestamp.log"
cf_log=$"$LOGS/BAH/$1/Over_Loans_Cashflow_$timestamp.log"

sqlldr $CON_STR_BH \
data=$master \
control=/home/dbuser/programs/DB-Loader-Over-Loans/BAH/Import/OVER_LOANS_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/BAH/$1/Over_Loans_Master_$timestamp.BAD

sqlldr $CON_STR_BH \
data=$cashflows \
control=/home/dbuser/programs/DB-Loader-Over-Loans/BAH/Import/OVER_LOANS_CASHFLOW.ctl \
LOG=$cf_log \
BAD=$LOGS/BAH/$1/Over_Loans_Cashflow_$timestamp.BAD

/home/dbuser/programs/BAH/health-checker-w2l.sh \
$INPUT/BAH/$1/DB-Loader-Over-Loans.txt \
$master_log \
$cf_log

