#!/usr/bin/env bash

echo "Start of Overseas Loans HK Import script Execution"

/home/dbuser/programs/DB-Loader-Over-Loans/HK/Import/ImportData.sh $1

echo "Start of Overseas Loans HK Export script Execution"

/home/dbuser/programs/DB-Loader-Over-Loans/HK/Export/ExportQuery.sh $1
