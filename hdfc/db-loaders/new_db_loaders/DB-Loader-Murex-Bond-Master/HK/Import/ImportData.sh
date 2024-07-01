#!/usr/bin/env bash

timestamp=$( date +%d%m%Y_%H%M%S)
/home/dbuser/programs/DB-Loader-Murex-Bond-Master/HK/Import/truncate.sh

file=$( ls /UAT_SH_INPUTDATA/HK/$1/Bond_Master_ADF_* )
master_log=$"$LOGS/HK/$1/Bond_Master_$timestamp.log"
dos2unix $file

sqlldr $CON_STR_HK \
control=/home/dbuser/programs/DB-Loader-Murex-Bond-Master/HK/Import/bond_master.ctl \
data=$file \
LOG=$master_log \
BAD=$LOGS/HK/$1/bond_master_BAD.BAD

/home/dbuser/programs/HK/health-checker-w1l.sh \
$INPUT/HK/$1/DB-Loader-Bond-Master.txt \
$master_log \

exit 0
