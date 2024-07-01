#!/usr/bin/env bash

echo "Start of Import script Execution"

/home/dbuser/programs/DB-Loader-INR-IRS/GC/Import/ImportData.sh $1

echo "End of Import script Execution"

echo "End of Export script Execution"

/home/dbuser/programs/DB-Loader-INR-IRS/GC/Export/ExportQuery.sh $1

echo "End of Export script Execution"
