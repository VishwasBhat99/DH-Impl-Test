#!/usr/bin/env bash

echo "Start of Import script Execution for Overseas Loans"

cd Import/
./ImportData.sh

echo "End of Import script Execution for Overseas Loans"

echo "Start of Export script Execution for Overseas Loans"

cd ../Export/
./ExportQuery.sh

echo "End of Export script Execution for Overseas Loans"
