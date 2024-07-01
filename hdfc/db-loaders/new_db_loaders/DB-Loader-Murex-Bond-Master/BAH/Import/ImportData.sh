#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S)
/home/dbuser/programs/DB-Loader-Murex-Bond-Master/BAH/Import/truncate.sh

file=$( ls /UAT_SH_INPUTDATA/BAH/$1/Bond_Master_ADF_* )
master_log=$"$LOGS/BAH/$1/Bond_Master_$timestamp.log"
dos2unix $file

sqlldr $CON_STR_BH \
control=/home/dbuser/programs/DB-Loader-Murex-Bond-Master/BAH/Import/bond_master.ctl \
data=$file \
LOG=$master_log \
BAD=$LOGS/BAH/$1/bond_master_BAD.BAD

/home/dbuser/programs/BAH/health-checker-w1l.sh \
$INPUT/BAH/$1/DB-Loader-Bond-Master.txt \
$master_log \

exit 0
