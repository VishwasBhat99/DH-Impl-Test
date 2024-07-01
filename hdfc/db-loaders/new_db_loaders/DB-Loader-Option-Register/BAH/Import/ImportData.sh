#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Option-Register/BAH/Import/truncate.sh
timestamp=$( date +%d%m%Y_%H%M%S )
master=$( ls  $INPUT/BAH/$1/master_opt.csv )
master_log=$"$LOGS/BAH/$1/Option_Master_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_BH \
data=$master \
control=/home/dbuser/programs/DB-Loader-Option-Register/BAH/Import/OPTION_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/BAH/$1/Option_Master_$timestamp.BAD

/home/dbuser/programs/BAH/health-checker-w1l.sh \
$INPUT/BAH/$1/DB-Loader-Option-Master.txt \
$master_log


