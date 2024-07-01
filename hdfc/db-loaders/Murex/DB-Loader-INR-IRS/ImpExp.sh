#!/usr/bin/env bash

echo "Start of Import script Execution..."

cd Import/
./ImportData.sh

echo "End of Import script Execution"

echo "Start of Export script Execution..."

cd ../Export/
./ExportQuery.sh

echo "End of Export script Execution"
