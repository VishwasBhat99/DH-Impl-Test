#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Finnone-Structured-Loans/Import/ImportData.sh $1

/home/dbuser/programs/DB-Loader-Finnone-Structured-Loans/Export/ExportQuery.sh $1
