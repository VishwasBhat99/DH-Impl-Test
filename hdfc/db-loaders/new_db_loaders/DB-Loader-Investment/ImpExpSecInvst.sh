#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Investment/Import/ImportData.sh $1

/home/dbuser/programs/DB-Loader-Investment/Export/ExportQuery.sh $1

echo "Securitisation Investments File Exported Successfully."
