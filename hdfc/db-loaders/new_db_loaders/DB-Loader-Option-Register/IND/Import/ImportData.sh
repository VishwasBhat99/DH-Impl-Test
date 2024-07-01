#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Option-Register/IND/Import/truncate.sh
timestamp=$( date +%d%m%Y_%H%M%S )
master=$( ls  $INPUT/IND/$1/master_opt.csv )
master_log=$"$LOGS/IND/$1/Option_Master_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_IND \
data=$master \
control=/home/dbuser/programs/DB-Loader-Option-Register/IND/Import/OPTION_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/IND/$1/Option_Master_$timestamp.BAD

/home/dbuser/programs/IND/health-checker-w1l.sh \
$INPUT/IND/$1/DB-Loader-Option-Master.txt \
$master_log 
 
