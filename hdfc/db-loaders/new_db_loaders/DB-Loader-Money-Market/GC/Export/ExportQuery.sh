
#!/usr/bin/env bash

echo "Start of deleting the old money-market.txt file"

echo "Start of spooling 'Money Market' data"

sqlplus -s $CON_STR_GC << ENDOFSQL
@/home/dbuser/programs/DB-Loader-Money-Market/GC/Export/export.sql $INPUT $1
ENDOFSQL
