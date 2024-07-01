#!/usr/bin/env bash

echo "Start of deleting the old money-market.txt file"

echo "Start of spooling 'Money Market' data"

sqlplus -s $CON_STR_HK << ENDOFSQL
@/home/dbuser/programs/DB-Loader-Money-Market/HK/Export/export.sql $INPUT $1
ENDOFSQL
