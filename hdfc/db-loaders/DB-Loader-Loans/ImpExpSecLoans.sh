#!/usr/bin/env bash

echo "Start of Import script Execution for securitisation loans"

cd Import/
./ImportData.sh

echo "End of Import script Execution for securitisation loans"

echo "Start of Export script Execution for securitisation loans"

cd ../Export/
./ExportQuery.sh

echo "End of Export script Execution for securitisation loans"
