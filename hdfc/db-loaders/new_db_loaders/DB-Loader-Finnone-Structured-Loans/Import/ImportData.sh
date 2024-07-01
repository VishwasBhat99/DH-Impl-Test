#!/usr/bin/env bash
/home/dbuser/programs/DB-Loader-Finnone-Structured-Loans/Import/truncate.sh
master=$"$INPUT/IND/$1/FF_FN_ALM_LOAN_EXTRACT_MOR_$1.txt"
cashflows=$"$INPUT/IND/$1/FF_FN_ALM_LOAN_SCHEDULE_CF_MOR_$1.txt"
timestamp=$( date +%d%m%Y_%H%M%S )
dos2unix $master
dos2unix $cashflows

master_log=$"$LOGS/IND/$1/Fin_Struct_Loans_Master_$timestamp.log"
cf_log=$"$LOGS/IND/$1/Fin_Struct_Loans_Cashflow_$timestamp.log"

sqlldr $CON_STR_IND \
control=/home/dbuser/programs/DB-Loader-Finnone-Structured-Loans/Import/Fin_Loans_Master.ctl \
data=$master \
LOG=$master_log \
BAD=$LOGS/IND/$1/Fin_Loans_Master_$timestamp.BAD

sqlldr $CON_STR_IND \
control=/home/dbuser/programs/DB-Loader-Finnone-Structured-Loans/Import/Fin_Loans_Cashflows.ctl \
data=$cashflows \
LOG=$cf_log \
BAD=$LOGS/IND/$1/Fin_Loans_Cashflows_$timestamp.BAD

/home/dbuser/programs/IND/health-checker-w2l.sh \
$INPUT/IND/$1/DB-Loader-Fin-Struct-Loans.txt \
$master_log \
$cf_log

