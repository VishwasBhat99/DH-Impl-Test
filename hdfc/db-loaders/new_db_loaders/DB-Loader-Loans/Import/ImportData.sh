#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
/home/dbuser/programs/DB-Loader-Loans/Import/truncate.sh
master=$INPUT/IND/$1/master-loans.csv
cashflow=$INPUT/IND/$1/cashflows-loans.csv
dos2unix $master
dos2unix $cashflow

sqlldr $CON_STR_IND \
data=$master  \
control=/home/dbuser/programs/DB-Loader-Loans/Import/SEC_LOANS_MASTER.ctl \
LOG=$LOGS/IND/$1/SEC_LOANS_MASTER_$timestamp.log \
BAD=$LOGS/IND/$1/SEC_LOANS_MASTER_$timestamp.BAD

sqlldr $CON_STR_IND \
data=$cashflow \
control=/home/dbuser/programs/DB-Loader-Loans/Import/SEC_LOANS_CASHFLOW.ctl \
LOG=$LOGS/IND/$1/SEC_LOANS_CASHFLOW_$timestamp.log \
BAD=$LOGS/IND/$1/SEC_LOANS_CASHFLOW_$timestamp.BAD

/home/dbuser/programs/IND/health-checker-w2l.sh \
$INPUT/IND/$1/DB-Loader-Sec-Invst.txt \
$LOGS/IND/$1/SEC_LOANS_MASTER_$timestamp.log \
$LOGS/IND/$1/SEC_LOANS_CASHFLOW_$timestamp.log 
