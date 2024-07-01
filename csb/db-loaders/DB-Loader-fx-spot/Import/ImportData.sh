#!/usr/bin/env bash

echo "Loading data..."

timestamp=$( date +%d%m%Y_%H%M%S )
bcp SecDealData in $CSB_ROOT/fx-spot/DB-Loader/Import/FXSpotData.txt -S $CSB_DBSERVER -d $CSB_DBNAME -U $CSB_USERNAME -P $CSB_PASS -e $CSB_ROOT/fx-spot/DB-Loader/Import/FXSpotData.log -c -t  '|'
bcp SecDealCF in $CSB_ROOT/fx-spot/DB-Loader/Import/FXSpotCF.txt -S $CSB_DBSERVER -d $CSB_DBNAME -U $CSB_USERNAME -P $CSB_PASS -e $CSB_ROOT/fx-spot/DB-Loader/Import/FXSpotCF.log -c -t  '|'

echo "Data Loaded Successfully!"