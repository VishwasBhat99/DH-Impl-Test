#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
/home/dbuser/programs/DB-Loader-Money-Market/GC/Import/truncate.sh
master=$( ls $INPUT/GC/$1/master-mm.csv )
cashflows=$( ls $INPUT/GC/$1/cashflows-mm.csv )
master_log=$"$LOGS/GC/$1/Borr_Lend_Master_$timestamp.log"
cf_log=$"$LOGS/GC/$1/Borr_Lend_cashflow_$timestamp.log"
dos2unix $master
dos2unix $cashflows

sqlldr $CON_STR_GC \
data=$master \
control=/home/dbuser/programs/DB-Loader-Money-Market/GC/Import/MMMaster.ctl \
LOG=$master_log \
BAD=$LOGS/GC/$1/Borr_Lend_Masteri_$timestamp.BAD

sqlldr $CON_STR_GC \
data=$cashflows \
control=/home/dbuser/programs/DB-Loader-Money-Market/GC/Import/MMCashflows.ctl \
LOG=$cf_log \
BAD=$LOGS/GC/$1/Borr_Lend_Cashflow_$timestamp.BAD

/home/dbuser/programs/GC/health-checker-w2l.sh \
$INPUT/GC/$1/DB-Loader-Money-Market.txt \
$master_log \
$cf_log

