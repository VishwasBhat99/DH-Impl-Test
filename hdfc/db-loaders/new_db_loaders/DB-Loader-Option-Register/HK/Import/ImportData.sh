#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Option-Register/HK/Import/truncate.sh
timestamp=$( date +%d%m%Y_%H%M%S )
master=$( ls  $INPUT/HK/$1/master_opt.csv )
master_log=$"$LOGS/HK/$1/Option_Master_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_HK \
data=$master \
control=/home/dbuser/programs/DB-Loader-Option-Register/HK/Import/OPTION_MASTER.ctl \
LOG=$master_log \
BAD=$LOGS/HK/$1/Option_Master_$timestamp.BAD

/home/dbuser/programs/HK/health-checker-w1l.sh \
$INPUT/HK/$1/DB-Loader-Option-Master.txt \
$master_log


