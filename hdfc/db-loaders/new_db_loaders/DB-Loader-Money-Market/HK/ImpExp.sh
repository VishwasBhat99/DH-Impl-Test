#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Money-Market/HK/Import/ImportData.sh $1

/home/dbuser/programs/DB-Loader-Money-Market/HK/Export/ExportQuery.sh $1

echo "End of Export script Execution for Money Market"
