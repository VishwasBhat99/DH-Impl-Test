#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-CAP-FLOOR/BAH/Import/truncate.sh
timestamp=$( date +%d%m%y_%H%M%S )
master=$"$INPUT/BAH/$1/master_cap.csv"
master_log=$"$LOGS/BAH/$1/Cap_Floor_Master_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_BH \
data=$master \
control=/home/dbuser/programs/DB-Loader-CAP-FLOOR/BAH/Import/cap-floor-master.ctl \
LOG=$master_log \
BAD=$LOGS/BAH/$1/Cap_Floor_Master_$timestamp.BAD

/home/dbuser/programs/BAH/health-checker-w1l.sh \
$INPUT/BAH/$1/DB-Loader-Cap-Floor.txt \
$master_log


