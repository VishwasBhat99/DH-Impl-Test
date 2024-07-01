#!/usr/bin/env bash

echo "Start of Overseas Loans BH Import script Execution"

/home/dbuser/programs/DB-Loader-Over-Loans/BAH/Import/ImportData.sh $1

echo "Start of Overseas Loans BH Export script Execution"

/home/dbuser/programs/DB-Loader-Over-Loans/BAH/Export/ExportQuery.sh $1
