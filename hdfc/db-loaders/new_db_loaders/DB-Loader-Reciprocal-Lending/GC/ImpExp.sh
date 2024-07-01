#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Reciprocal-Lending/GC/Import/ImportData.sh $1

/home/dbuser/programs/DB-Loader-Reciprocal-Lending/GC/Export/ExportQuery.sh $1

