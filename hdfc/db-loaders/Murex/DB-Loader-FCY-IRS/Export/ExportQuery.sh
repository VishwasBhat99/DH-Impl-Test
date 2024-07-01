#!/usr/bin/env bash

echo "Start of deleting the old fcy-irs.txt file"

rm /data/oracle18c/app/product/18c/dbhome/ExportLog/fcy-irs.txt

echo "End of deleting the old fcy-irs.txt file"

echo "Start of spooling the data"

sqlplus -s balmusr/HdFcBank13\$\# << ENDOFSQL
@export.sql
ENDOFSQL

echo "End of spooling the data"
