#!/usr/bin/env bash

echo "Start of Import script Execution for FRA"

cd Import/
./ImportData.sh

echo "End of Import script Execution for FRA"

echo "Start of Export script Execution for FRA"

cd ../Export/
./ExportQuery.sh

echo "End of Export script Execution for FRA"
