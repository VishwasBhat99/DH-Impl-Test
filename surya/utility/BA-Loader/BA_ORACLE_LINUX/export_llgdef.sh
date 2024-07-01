#!/usr/bin/env bash

FILENAME=$SH_SUMMARY/$3/BA/$1/balm_llgdef.txt
rm -f $FILENAME

sqlplus -s $CON_STR_BALMUSR << ENDOFSQL
@$SCRIPTS/$3/loader-scripts/BA/SQL/export_llgdef.sql $1 $2 $3 $SH_SUMMARY 
ENDOFSQL
