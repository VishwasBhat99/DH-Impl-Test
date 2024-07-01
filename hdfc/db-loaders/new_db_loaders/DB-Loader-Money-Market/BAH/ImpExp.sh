#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Money-Market/BAH/Import/ImportData.sh $1

/home/dbuser/programs/DB-Loader-Money-Market/BAH/Export/ExportQuery.sh $1

echo "End of Export script Execution for Money Market"
