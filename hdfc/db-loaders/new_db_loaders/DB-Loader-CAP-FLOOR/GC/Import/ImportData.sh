#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-CAP-FLOOR/GC/Import/truncate.sh
timestamp=$( date +%d%m%y_%H%M%S )
master=$"$INPUT/GC/$1/master_cap.csv"
master_log=$"$LOGS/GC/$1/Cap_Floor_Master_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_GC \
data=$master \
control=/home/dbuser/programs/DB-Loader-CAP-FLOOR/GC/Import/cap-floor-master.ctl \
LOG=$master_log \
BAD=$LOGS/GC/$1/Cap_Floor_Master_$timestamp.BAD

/home/dbuser/programs/GC/health-checker-w1l.sh \
$INPUT/GC/$1/DB-Loader-Cap-Floor.txt \
$master_log


