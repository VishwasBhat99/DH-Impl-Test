#!/usr/bin/env bash

file_path=$"../pre-processor/output/exchange-rates.txt"
sqlldr balmusr/HdFcBank13\$\# control=./ExchangeRate.ctl data=$file_path LOG=/data/oracle18c/app/product/18c/dbhome/ImportLog/SEC_LOANS_MASTER.log BAD=/data/oracle18c/app/product/18c/dbhome/ImportLog/SEC_LOANS_MASTER.BAD
