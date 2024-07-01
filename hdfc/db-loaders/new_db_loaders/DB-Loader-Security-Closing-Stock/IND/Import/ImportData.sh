#!/usr/bin/env bash
/home/dbuser/programs/DB-Loader-Security-Closing-Stock/IND/Import/Truncate.sh
master=$( ls $INPUT/IND/$1/close_stock.csv )
timestamp=$( date +%d%m%Y_%H%M%S )
master_log=$"$LOGS/IND/$1/Sec_Close_Stock_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_IND \
control=/home/dbuser/programs/DB-Loader-Security-Closing-Stock/IND/Import/sec_close_stock.ctl \
data=$master \
LOG=$master_log \
BAD=$LOGS/IND/$1/Sec_Close_Stock_$timestamp.BAD

/home/dbuser/programs/IND/health-checker-w1l.sh \
$INPUT/IND/$1/DB-Loader-Sec-Close-Stock.txt \
$master_log 

