#!/usr/bin/env bash

echo "Start of spooling the data"

sqlplus -s $CON_STR_BH << ENDOFSQL
@/home/dbuser/programs/DB-Loader-CAP-FLOOR/GC/Export/export.sql $1
ENDOFSQL

echo "End of spooling the data"
