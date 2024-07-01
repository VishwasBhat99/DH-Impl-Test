#!/usr/bin/env bash

FILENAME=$SH_SUMMARY/$3/BA/$1/balm_data.txt

sqlplus -s $CON_STR_BAUSR << ENDOFSQL
@$SCRIPTS/$3/loader-scripts/BA/SQL/load_ba_data.sql $3 $2
ENDOFSQL

$SCRIPTS/$3/loader-scripts/BA/commit.sh
