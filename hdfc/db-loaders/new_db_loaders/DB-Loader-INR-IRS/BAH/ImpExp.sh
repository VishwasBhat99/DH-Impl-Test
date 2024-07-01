#!/usr/bin/env bash

echo "Start of Import script Execution"

/home/dbuser/programs/DB-Loader-INR-IRS/BAH/Import/ImportData.sh $1

echo "End of Import script Execution"

echo "End of Export script Execution"

/home/dbuser/programs/DB-Loader-INR-IRS/BAH/Export/ExportQuery.sh $1

echo "End of Export script Execution"
