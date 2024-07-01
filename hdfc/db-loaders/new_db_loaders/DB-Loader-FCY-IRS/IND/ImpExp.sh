#!/usr/bin/env bash

echo "Start of Import script Execution..."

/home/dbuser/programs/DB-Loader-FCY-IRS/IND/Import/ImportData.sh $1

echo "End of Import script Execution"

echo "Start of Export script Execution..."

/home/dbuser/programs/DB-Loader-FCY-IRS/IND/Export/ExportQuery.sh $1

echo "End of Export script Execution"
