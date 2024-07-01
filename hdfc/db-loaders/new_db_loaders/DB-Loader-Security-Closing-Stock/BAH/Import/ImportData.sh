#!/usr/bin/env bash
/home/dbuser/programs/DB-Loader-Security-Closing-Stock/BAH/Import/Truncate.sh
master=$( ls $INPUT/BAH/$1/close_stock.csv )
timestamp=$( date +%d%m%Y_%H%M%S )
master_log=$"$LOGS/BAH/$1/Sec_Close_Stock_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_BH \
control=/home/dbuser/programs/DB-Loader-Security-Closing-Stock/BAH/Import/sec_close_stock.ctl \
data=$master \
LOG=$master_log \
BAD=$LOGS/BAH/$1/Sec_Close_Stock_$timestamp.BAD

/home/dbuser/programs/BAH/health-checker-w1l.sh \
$INPUT/BAH/$1/DB-Loader-Sec-Close-Stock.txt \
$master_log
