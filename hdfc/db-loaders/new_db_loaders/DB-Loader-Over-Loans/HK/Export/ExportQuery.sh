#!/usr/bin/env bash



echo "Start of spooling the data"

sqlplus -s $CON_STR_HK << ENDOFSQL
@/home/dbuser/programs/DB-Loader-Over-Loans/HK/Export/export.sql $INPUT $1
ENDOFSQL
echo "End of spooling the data"
