#!/usr/bin/env bash

echo "Start of delete script Execution for subsidary-lcr"

./delete.sh

echo "End of delete script Execution for subsidary-lcr"

echo "Start of commit script Execution for subsidary-lcr"

./commit.sh

echo "End of commit script Execution for subsidary-lcr"

echo "Start of Import script Execution for subsidary-lcr"

cd Import/
./ImportData.sh

echo "End of Import script Execution for subsidary-lcr"

echo "Start of commit script Execution for subsidary-lcr"

cd ..
./commit.sh

echo "End of commit script Execution for subsidary-lcr"
