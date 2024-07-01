#!/usr/bin/env bash

echo "Finnone Loans DB Loader..."

cd Import/
./ImportData.sh

cd ../Export/
./ExportQuery.sh
