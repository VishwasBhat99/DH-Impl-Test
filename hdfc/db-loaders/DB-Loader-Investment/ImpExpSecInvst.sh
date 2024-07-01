#!/usr/bin/env bash

echo "Start of Import script Execution for securitisation investments"

cd Import/
./ImportData.sh

echo "End of Import script Execution for securitisation investments"

echo "Start of Export script Execution for securitisation investments"

cd ../Export/
./ExportQuery.sh

echo "End of Export script Execution for securitisation investments"
