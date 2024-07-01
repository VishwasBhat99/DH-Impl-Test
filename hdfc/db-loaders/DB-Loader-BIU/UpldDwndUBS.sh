#!/usr/bin/env bash

echo "Start of Import script Execution for UBS Loans"

cd Import/
./ImportData.sh

echo "End of Import script Execution for UBS Loans"

echo "Start of Export script Execution for UBS Loans"

cd ../Export/
./ExportQuery.sh

echo "End of Export script Execution for UBS Loans"
