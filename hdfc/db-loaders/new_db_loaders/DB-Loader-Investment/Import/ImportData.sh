#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )

/home/dbuser/programs/DB-Loader-Investment/Import/truncate.sh

master=$INPUT/IND/$1/master-inv.csv
cashflow=$INPUT/IND/$1/cashflows-inv.csv
master_log=$"$LOGS/IND/$1/SEC_INVST_CASHFLOW_$timestamp.log"
cf_log=$"$LOGS/IND/$1/SEC_INVST_CASHFLOW_$timestamp.log"
dos2unix $master
dos2unix $cashflow

sqlldr $CON_STR_IND \
data=$master \
control=/home/dbuser/programs/DB-Loader-Investment/Import/SEC_INVST_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/IND/$1/SEC_INVST_MASTER_$timestamp.BAD

sqlldr $CON_STR_IND \
data=$cashflow \
control=/home/dbuser/programs/DB-Loader-Investment/Import/SEC_INVST_CASHFLOW.ctl \
LOG=$cf_log \
BAD=$LOGS/IND/$1/SEC_INVST_CASHFLOW_$timestamp.BAD

/home/dbuser/programs/IND/health-checker-w2l.sh \
$INPUT/IND/$1/DB-Loader-Sec-Invst.txt \
$master_log \
$cf_log 

