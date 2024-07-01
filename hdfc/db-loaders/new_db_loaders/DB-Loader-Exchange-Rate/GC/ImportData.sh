#!/usr/bin/env bash

file_path=$"$PREPROCESS/GC/$1/ExchangeRates.txt"
sqlldr $CON_STR_IND control=/home/dbuser/programs/DB-Loader-Exchange-Rate/GC/ExchangeRate.ctl data=$file_path LOG=$LOGS/GC/$1/ExchangeRate.log BAD=$LOGS/GC/$1/ExchangeRate.BAD
