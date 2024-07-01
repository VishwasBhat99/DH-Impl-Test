#!/usr/bin/env bash

file_path=$"$PREPROCESS/IND/$1/ExchangeRates.txt"
sqlldr $CON_STR_IND \
control=/home/dbuser/programs/DB-Loader-Exchange-Rate/IND/ExchangeRate.ctl \
data=$file_path \
LOG=$LOGS/IND/$1/ExchangeRate.log \
BAD=$LOGS/IND/$1/ExchangeRate.BAD
