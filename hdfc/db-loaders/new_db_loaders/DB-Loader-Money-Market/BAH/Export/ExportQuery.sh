#!/usr/bin/env bash

echo "Start of deleting the old money-market.txt file"

echo "Start of spooling 'Money Market' data"

sqlplus -s $CON_STR_BH << ENDOFSQL
@/home/dbuser/programs/DB-Loader-Money-Market/BAH/Export/export.sql $INPUT $1
ENDOFSQL
