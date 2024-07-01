#!/usr/bin/env bash

echo "Exporting the data..."

sqlcmd -S $CSB_DBSERVER -d $CSB_DBNAME -U $CSB_USERNAME -P $CSB_PASS -o $CSB_ROOT/close-stock/pre-processor/input-resources/input-files/input.txt -i $CSB_ROOT/close-stock/DB-Loader/Export/export.sql

echo "Data Exported Successfully!"
