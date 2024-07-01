#!/usr/bin/env bash

echo "Loading data..."

timestamp=$( date +%d%m%Y_%H%M%S )
bcp SecDealData in $CSB_ROOT/close-stock/DB-Loader/Import/sec_deal_data.txt -S $CSB_DBSERVER -d $CSB_DBNAME -U $CSB_USERNAME -P $CSB_PASS -e $CSB_ROOT/close-stock/DB-Loader/Import/sec_deal_data.log -c -t  '|'
bcp SecDealCF in $CSB_ROOT/close-stock/DB-Loader/Import/sec_deal_cf.txt -S $CSB_DBSERVER -d $CSB_DBNAME -U $CSB_USERNAME -P $CSB_PASS -e $CSB_ROOT/close-stock/DB-Loader/Import/sec_deal_cf.log -c -t  '|'

echo "Data Loaded Successfully!"