#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Option-Register/GC/Import/truncate.sh
timestamp=$( date +%d%m%Y_%H%M%S )
master=$( ls  $INPUT/GC/$1/master_opt.csv )
master_log=$"$LOGS/GC/$1/Option_Master_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_GC \
data=$master \
control=/home/dbuser/programs/DB-Loader-Option-Register/GC/Import/OPTION_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/GC/$1/Option_Master_$timestamp.BAD

/home/dbuser/programs/GC/health-checker-w1l.sh \
$INPUT/GC/$1/DB-Loader-Option-Master.txt \
$master_log

