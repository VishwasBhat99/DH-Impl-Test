#!/usr/bin/env bash

echo "Start of spooling the Finnone Loans data"

rm -f $INPUT/IND/$1/FinLoans.txt

sqlplus -s $CON_STR_IND << ENDOFSQL
@/home/dbuser/programs/DB-Loader-Finnone-Structured-Loans/Export/export.sql $1
ENDOFSQL

echo "End of spooling the Finnone Loans data"
