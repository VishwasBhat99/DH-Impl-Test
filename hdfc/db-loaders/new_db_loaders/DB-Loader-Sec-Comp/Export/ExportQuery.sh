#!/usr/bin/env bash

echo "Start of spooling the data for Murex Security Composition"

rm INPUT/IND/&1/murex-sec-comp.txt
sqlplus -s $CON_STR_IND << ENDOFSQL
@/home/dbuser/programs/DB-Loader-Sec-Comp/Export/export.sql $1
ENDOFSQL

echo "End of spooling the data for Murex Security Composition"

