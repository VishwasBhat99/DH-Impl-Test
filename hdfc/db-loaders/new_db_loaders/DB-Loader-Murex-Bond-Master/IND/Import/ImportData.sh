#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S )
/home/dbuser/programs/DB-Loader-Murex-Bond-Master/IND/Import/truncate.sh
file=$( ls $INPUT/IND/$1/Bond_Master_ADF_* )
master_log=$"$LOGS/IND/$1/Bond_Master_$timestamp.log"
dos2unix $file 

sqlldr $CON_STR_IND  \
control=/home/dbuser/programs/DB-Loader-Murex-Bond-Master/IND/Import/bond_master.ctl \
data=$file \
LOG=$master_log \
BAD=$LOGS/IND/$1/Bond_Master_$timestamp.BAD

/home/dbuser/programs/IND/health-checker-w1l.sh \
$INPUT/IND/$1/DB-Loader-Bond-Master.txt \
$master_log

exit 0
