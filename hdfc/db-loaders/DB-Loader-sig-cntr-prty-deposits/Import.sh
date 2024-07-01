#!/usr/bin/env bash

echo "Start of delete script Execution for sig-cntrprty-deposits"

./delete.sh

echo "End of delete script Execution for sig-cntrprty-deposits"

echo "Start of commit script Execution for sig-cntrprty-deposits"

./commit.sh

echo "End of commit script Execution for sig-cntrprty-deposits"

echo "Start of Import script Execution for sig-cntrprty-deposits"

cd Import/
./ImportData.sh

echo "End of Import script Execution for sig-cntrprty-deposits"

echo "Start of commit script Execution for sig-cntrprty-deposits"

cd ..
./commit.sh

echo "End of commit script Execution for sig-cntrprty-deposits"
