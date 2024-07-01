#!/usr/bin/env bash
/home/dbuser/programs/DB-Loader-Security-Closing-Stock/GC/Import/Truncate.sh
master=$( ls $INPUT/GC/$1/close_stock.csv )
timestamp=$( date +%d%m%Y_%H%M%S )
master_log=$"$LOGS/GC/$1/Sec_Close_Stock_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_GC \
control=/home/dbuser/programs/DB-Loader-Security-Closing-Stock/GC/Import/sec_close_stock.ctl \
data=$master \
LOG=$master_log \
BAD=$LOGS/GC/$1/Sec_Close_Stock_$timestamp.BAD

/home/dbuser/programs/GC/health-checker-w1l.sh \
$INPUT/GC/$1/DB-Loader-Sec-Close-Stock.txt \
$master_log
