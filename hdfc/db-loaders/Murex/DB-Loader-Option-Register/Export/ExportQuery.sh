#!/usr/bin/env bash

echo "Start of deleting the old options.txt file"

rm /data/oracle18c/app/product/18c/dbhome/ExportLog/option.txt

echo "Start of spooling the data"

sqlplus -s balmusr/HdFcBank13\$\# << ENDOFSQL
@export.sql
SPOOL OFF;
ENDOFSQL

echo "End of spooling the data"
