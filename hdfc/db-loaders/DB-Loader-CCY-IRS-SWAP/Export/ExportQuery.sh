#!/usr/bin/env bash

echo "Start of deleting the old CCYIRSSWAP.txt file"

rm /data/oracle18c/app/product/18c/dbhome/ExportLog/CCYIRSSWAP.txt

echo "End of deleting the old CCYIRSSWAP.txt file"

echo "Start of spooling the data for CCY IRS SWAP"

sqlplus -s balmusr/HdFcBank13\$\# << ENDOFSQL
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL /data/oracle18c/app/product/18c/dbhome/ExportLog/CCYIRSSWAP.txt;
select A1.DealNo
||'|'||to_char(A1.StartDate,'DD-MM-')||'20'||substr(to_char(A1.startdate,'YYYY'),3)
||'|'||to_char(A1.MatDate,'DD-MM-')||'20'||substr(to_char(A1.dealstartdate,'YYYY'),3)
||'|'||A1.PayCurrentNotional
||'|'||A1.RecCurrentNotional
||'|||'||to_char(A1.TradeDate,'DD-MM-')||'20'||substr(to_char(A1.TradeDate,'YYYY'),3)
||'||'||A1.InrPV01
||'|'||A1.FcyMTM
||'|'||A1.InrMTM
||'|'||A1.BankOrNonBank
||'|'||A1.BookName
||'|'||A1.Desk
||'|'||A1.PayIntRate
||'|'||A1.RecIntRate
||'|||'||A1.CustTyp
||'||'||to_char(A1.PayResetDate,'DD-MM-')||'20'||substr(to_char(A1.PayResetdate,'YYYY'),3)
||'||'||to_char(A1.RecResetdate,'DD-MM-')||'20'||substr(to_char(A1.RecResetDate,'YYYY'),3)
||'|'||A1.Payccy
||'|'||A1.RecCcy
||'|'||A1.ExratePay
||'|'||A1.PxrateRec
||'|'||A1.PayResetFreq
||'|'||A1.RecResetFreq
from CCY_IRS_SWAP_MASTER A1;
SPOOL OFF;
ENDOFSQL

echo "End of spooling the data for CCY IRS SWAP"
