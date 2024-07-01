#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Money-Market/IND/Import/ImportData.sh $1

/home/dbuser/programs/DB-Loader-Money-Market/IND/Export/ExportQuery.sh $1

echo "End of Export script Execution for Money Market"
