#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-CAP-FLOOR/IND/Import/truncate.sh
timestamp=$( date +%d%m%y_%H%M%S )
master=$"$INPUT/IND/$1/master_cap.csv"
master_log=$"$LOGS/IND/$1/Cap_Floor_Master_$timestamp.log"
dos2unix $master

sqlldr $CON_STR_IND \
data=$master \
control=/home/dbuser/programs/DB-Loader-CAP-FLOOR/IND/Import/cap-floor-master.ctl \
LOG=$master_log \
BAD=$LOGS/IND/$1/Cap_Floor_Master_$timestamp.BAD

/home/dbuser/programs/IND/health-checker-w1l.sh \
$INPUT/IND/$1/DB-Loader-Cap-Floor.txt \
$master_log 

