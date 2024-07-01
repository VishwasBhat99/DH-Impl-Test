#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-CAP-FLOOR/HK/Import/truncate.sh
timestamp=$( date +%d%m%y_%H%M%S )
master=$"$INPUT/HK/$1/master_cap.csv"
master_log=$"$LOGS/HK/$1/Cap_Floor_Master_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_HK \
data=$master \
control=/home/dbuser/programs/DB-Loader-CAP-FLOOR/HK/Import/cap-floor-master.ctl \
LOG=$master_log \
BAD=$LOGS/HK/$1/Cap_Floor_Master_$timestamp.BAD

/home/dbuser/programs/HK/health-checker-w1l.sh \
$INPUT/HK/$1/DB-Loader-Cap-Floor.txt \
$master_log


