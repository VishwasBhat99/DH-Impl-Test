#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Sec-Comp/Import/truncate.sh
timestamp=$( date +%d%m%Y_%H%M%S )
sec_comp=$( ls $INPUT/IND/$1/SECURITY_COMPOSITION.csv )
sec_comp_cf=$( ls $INPUT/IND/$1/SEC_COMP_CASHFLOWS.csv )
sec_comp_matt=$( ls $INPUT/IND/OPERATIONAL_FILES/security_composition_matt.csv )
master_log=$"$LOGS/IND/$1/Sec_Comp_Master_$timestamp.log"
cf_log=$"$LOGS/IND/$1/Sec_Comp_Cashflow_$timestamp.log"
sec_comp_opt=$"$LOGS/IND/$1/Sec_Comp_Matt_$timestamp.log"
dos2unix $sec_comp
dos2unix $sec_comp_cf
dos2unix $sec_comp_matt

sqlldr $CON_STR_IND \
data=$sec_comp \
control=/home/dbuser/programs/DB-Loader-Sec-Comp/Import/SECURITY-COMPOSITION.ctl \
LOG=$master_log \
BAD=$LOGS/IND/$1/Sec_Comp_Master_$timestamp.BAD

sqlldr $CON_STR_IND \
data=$sec_comp_cf \
control=/home/dbuser/programs/DB-Loader-Sec-Comp/Import/SECURITY-COMPOSITION-CASHFLOW.ctl \
LOG=$cf_log \
BAD=$LOGS/IND/$1/Sec_Comp_Cashflow_$timestamp.BAD

sqlldr $CON_STR_IND \
data=$sec_comp_matt \
control=/home/dbuser/programs/DB-Loader-Sec-Comp/Import/SECURITY-COMPOSITION-MATURITY.ctl \
LOG=$sec_comp_opt \
BAD=$LOGS/IND/$1/Sec_Comp_Matt_$timestamp.BAD

/home/dbuser/programs/IND/health-checker-w3l.sh \
$INPUT/IND/$1/DB-Loader-Sec-Comp.txt \
$master_log \
$cf_log \
$sec_comp_opt
