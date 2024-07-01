#!/usr/bin/env bash

echo "Start of Import script Execution for INR IRS SWAP"

cd Import/
./ImportData.sh

echo "End of Import script Execution for INR IRS SWAP"

echo "Start of Export script Execution for INR IRS SWAP"

cd ../Export/
./ExportQuery.sh

echo "End of Export script Execution for INR IRS SWAP"
