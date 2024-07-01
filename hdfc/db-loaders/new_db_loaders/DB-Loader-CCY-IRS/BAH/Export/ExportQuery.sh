#!/usr/bin/env bash

echo "Start of spooling the data"

sqlplus -s $CON_STR_BH << ENDOFSQL
@/home/dbuser/programs/DB-Loader-CCY-IRS/BAH/Export/export.sql $1
ENDOFSQL

echo "End of spooling the data"
