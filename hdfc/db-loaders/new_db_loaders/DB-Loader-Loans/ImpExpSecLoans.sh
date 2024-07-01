#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Loans/Import/ImportData.sh $1

/home/dbuser/programs/DB-Loader-Loans/Export/ExportQuery.sh $1

echo "Securitisation Loans File Exported Successfully."
