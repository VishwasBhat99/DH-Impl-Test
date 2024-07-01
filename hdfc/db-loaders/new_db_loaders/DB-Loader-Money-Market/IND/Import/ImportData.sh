#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
/home/dbuser/programs/DB-Loader-Money-Market/IND/Import/truncate.sh
master=$( ls $INPUT/IND/$1/master-mm.csv )
cashflows=$( ls $INPUT/IND/$1/cashflows-mm.csv )
master_log=$"$LOGS/IND/$1/Borr_Lend_Master_$timestamp.log"
cf_log=$"$LOGS/IND/$1/Borr_Lend_cashflow_$timestamp.log"
dos2unix $master
dos2unix $cashflows

sqlldr $CON_STR_IND \
data=$master \
control=/home/dbuser/programs/DB-Loader-Money-Market/IND/Import/MMMaster.ctl \
LOG=$master_log \
BAD=$LOGS/IND/$1/Borr_Lend_Masteri_$timestamp.BAD

sqlldr $CON_STR_IND \
data=$cashflows \
control=/home/dbuser/programs/DB-Loader-Money-Market/IND/Import/MMCashflows.ctl \
LOG=$cf_log \
BAD=$LOGS/IND/$1/Borr_Lend_Cashflow_$timestamp.BAD

/home/dbuser/programs/IND/health-checker-w2l.sh \
$INPUT/IND/$1/DB-Loader-Money-Market.txt \
$master_log \
$cf_log
