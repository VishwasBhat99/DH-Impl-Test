#!/usr/bin/env bash

echo "Start of Import script Execution for Murex Security Composition"

/home/dbuser/programs/DB-Loader-Sec-Comp/Import/ImportData.sh $1

echo "End of Import script Execution for Murex Security Composition"

echo "Start of Export script Execution for Murex Security Composition"

/home/dbuser/programs/DB-Loader-Sec-Comp/Export/ExportQuery.sh $1

echo "End of Export script Execution for Murex Security Composition"
