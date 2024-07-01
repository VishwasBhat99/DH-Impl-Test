#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Money-Market/GC/Import/ImportData.sh $1

/home/dbuser/programs/DB-Loader-Money-Market/GC/Export/ExportQuery.sh $1

echo "End of Export script Execution for Money Market"
