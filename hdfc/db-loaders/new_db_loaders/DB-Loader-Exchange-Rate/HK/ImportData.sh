#!/usr/bin/env bash

file_path=$"$PREPROCESS/HK/$1/ExchangeRates.txt"
sqlldr $CON_STR_HK control=/home/dbuser/programs/DB-Loader-Exchange-Rate/HK/ExchangeRate.ctl data=$file_path LOG=$LOGS/HK/$1/ExchangeRate.log BAD=$LOGS/HK/$1/ExchangeRate.BAD
