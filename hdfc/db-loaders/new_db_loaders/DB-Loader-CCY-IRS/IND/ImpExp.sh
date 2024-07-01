#!/usr/bin/env bash

echo "Start of Import script Execution"

/home/dbuser/programs/DB-Loader-CCY-IRS/IND/Import/ImportData.sh $1

echo "End of Import script Execution"

echo "End of Export script Execution"

/home/dbuser/programs/DB-Loader-CCY-IRS/IND/Export/ExportQuery.sh $1

echo "End of Export script Execution"
