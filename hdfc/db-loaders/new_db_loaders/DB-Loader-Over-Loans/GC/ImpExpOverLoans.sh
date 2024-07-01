#!/usr/bin/env bash

echo "Start of Overseas Loans GC Import script Execution"

/home/dbuser/programs/DB-Loader-Over-Loans/GC/Import/ImportData.sh $1

echo "Start of Overseas Loans GC Export script Execution"

/home/dbuser/programs/DB-Loader-Over-Loans/GC/Export/ExportQuery.sh $1
