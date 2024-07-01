#!/usr/bin/env bash

echo "Start of delete script Execution for top-n-deposits"

./delete.sh

echo "End of delete script Execution for top-n-deposits"

echo "Start of commit script Execution for top-n-deposits"

./commit.sh

echo "End of commit script Execution for top-n-deposits"

echo "Start of Import script Execution for top-n-deposits"

cd Import/
./ImportData.sh

echo "End of Import script Execution for top-n-deposits"

echo "Start of commit script Execution for top-n-deposits"

cd ..
./commit.sh

echo "End of commit script Execution for top-n-deposits"
