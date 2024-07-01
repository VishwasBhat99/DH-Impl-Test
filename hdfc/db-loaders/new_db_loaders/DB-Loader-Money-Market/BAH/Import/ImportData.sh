#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
/home/dbuser/programs/DB-Loader-Money-Market/BAH/Import/truncate.sh
master=$( ls $INPUT/BAH/$1/master-mm.csv )
cashflows=$( ls $INPUT/BAH/$1/cashflows-mm.csv )
master_log=$"$LOGS/BAH/$1/Borr_Lend_Master_$timestamp.log"
cf_log=$"$LOGS/BAH/$1/Borr_Lend_cashflow_$timestamp.log"
dos2unix $master
dos2unix $cashflows

sqlldr $CON_STR_BH \
data=$master \
control=/home/dbuser/programs/DB-Loader-Money-Market/BAH/Import/MMMaster.ctl \
LOG=$master_log \
BAD=$LOGS/BAH/$1/Borr_Lend_Masteri_$timestamp.BAD

sqlldr $CON_STR_BH \
data=$cashflows \
control=/home/dbuser/programs/DB-Loader-Money-Market/BAH/Import/MMCashflows.ctl \
LOG=$cf_log \
BAD=$LOGS/BAH/$1/Borr_Lend_Cashflow_$timestamp.BAD

/home/dbuser/programs/BAH/health-checker-w2l.sh \
$INPUT/BAH/$1/DB-Loader-Money-Market.txt \
$master_log \
$cf_log

