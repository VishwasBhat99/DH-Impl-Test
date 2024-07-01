#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S)
/home/dbuser/programs/DB-Loader-Murex-Bond-Master/GC/Import/truncate.sh

file=$( ls /UAT_SH_INPUTDATA/GC/$1/Bond_Master_ADF_* )
master_log=$"$LOGS/GC/$1/Bond_Master_$timestamp.log"
dos2unix $file 

sqlldr $CON_STR_GC \
control=/home/dbuser/programs/DB-Loader-Murex-Bond-Master/GC/Import/bond_master.ctl \
data=$file \
LOG=$master_log \
BAD=$LOGS/GC/$1/bond_master_BAD.BAD

/home/dbuser/programs/GC/health-checker-w1l.sh \
$INPUT/GC/$1/DB-Loader-Bond-Master.txt \
$master_log \

exit 0
