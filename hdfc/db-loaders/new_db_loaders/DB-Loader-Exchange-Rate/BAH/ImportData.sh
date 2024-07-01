#!/usr/bin/env bash

file_path=$"$PREPROCESS/BAH/$1/ExchangeRates.txt"
sqlldr $CON_STR_BH control=/home/dbuser/programs/DB-Loader-Exchange-Rate/BAH/ExchangeRate.ctl data=$file_path LOG=$LOGS/BAH/$1/ExchangeRate.log BAD=$LOGS/BAH/$1/ExchangeRate.BAD

echo "Exchange Rate Loaded Successfully!"
