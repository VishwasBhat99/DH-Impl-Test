#!/usr/bin/env bash

echo "Start of spooling the data"

sqlplus -s $CON_STR_IND << ENDOFSQL
@/home/dbuser/programs/DB-Loader-Option-Register/IND/Export/export.sql $1
SPOOL OFF;
ENDOFSQL

echo "End of spooling the data"
