#!/usr/bin/env bash
/home/dbuser/programs/DB-Loader-Security-Closing-Stock/HK/Import/Truncate.sh
master=$( ls $INPUT/HK/$1/close_stock.csv )
timestamp=$( date +%d%m%Y_%H%M%S )
master_log=$"$LOGS/HK/$1/Sec_Close_Stock_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_HK \
control=/home/dbuser/programs/DB-Loader-Security-Closing-Stock/HK/Import/sec_close_stock.ctl \
data=$master \
LOG=$master_log \
BAD=$LOGS/HK/$1/Sec_Close_Stock_$timestamp.BAD

/home/dbuser/programs/HK/health-checker-w1l.sh \
$INPUT/HK/$1/DB-Loader-Sec-Close-Stock.txt \
$master_log
