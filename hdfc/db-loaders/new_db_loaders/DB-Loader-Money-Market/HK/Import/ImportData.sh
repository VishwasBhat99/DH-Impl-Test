#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
/home/dbuser/programs/DB-Loader-Money-Market/HK/Import/truncate.sh
master=$( ls $INPUT/HK/$1/master-mm.csv )
cashflows=$( ls $INPUT/HK/$1/cashflows-mm.csv )
master_log=$"$LOGS/HK/$1/Borr_Lend_Master_$timestamp.log"
cf_log=$"$LOGS/HK/$1/Borr_Lend_cashflow_$timestamp.log"
dos2unix $master
dos2unix $cashflows

sqlldr $CON_STR_HK \
data=$master \
control=/home/dbuser/programs/DB-Loader-Money-Market/HK/Import/MMMaster.ctl \
LOG=$master_log \
BAD=$LOGS/HK/$1/Borr_Lend_Masteri_$timestamp.BAD

sqlldr $CON_STR_HK \
data=$cashflows \
control=/home/dbuser/programs/DB-Loader-Money-Market/HK/Import/MMCashflows.ctl \
LOG=$cf_log \
BAD=$LOGS/HK/$1/Borr_Lend_Cashflow_$timestamp.BAD

/home/dbuser/programs/HK/health-checker-w2l.sh \
$INPUT/HK/$1/DB-Loader-Money-Market.txt \
$master_log \
$cf_log

