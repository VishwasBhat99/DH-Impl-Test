#/usr/bin/env bash

echo "Start of spooling the data"
sqlplus -s $CON_STR_IND << ENDOFSQL
@/home/dbuser/programs/DB-Loader-Reciprocal-Lending/IND/Export/export.sql $INPUT $1
ENDOFSQL

echo "End of spooling the data"
