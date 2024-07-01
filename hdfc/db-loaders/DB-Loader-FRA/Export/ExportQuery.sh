#!/usr/bin/env bash

echo "Start of deleting the old FRA.txt file"

rm /data/oracle18c/app/product/18c/dbhome/ExportLog/FRA.txt

echo "End of deleting the old FRA.txt file"

echo "Start of spooling the data for FRA"

sqlplus -s balmusr/HdFcBank13\$\# << ENDOFSQL
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL /data/oracle18c/app/product/18c/dbhome/ExportLog/FRA.txt;
select A1.TradeId
||'|'||to_char(A1.StartDate,'DD-MM-')||'20'||substr(to_char(A1.startdate,'YYYY'),3)
||'|'||to_char(A1.EndDate,'DD-MM-')||'20'||substr(to_char(A1.EndDate,'YYYY'),3)
||'|'||A1.OsNotionalInrPay
||'|'||A1.OsNotionalInrRec
||'|||'||to_char(A1.TradeDate,'DD-MM-')||'20'||substr(to_char(A1.TradeDate,'YYYY'),3)
||'||'||A1.PorS
||'|'||A1.Pv01Inr
||'|'||A1.MtmFcy
||'|'||A1.MtmInr
||'|'||A1.BankOrNonBank
||'|'||A1.Book
||'|'||A1.Desk
||'|'||A1.FixedRate
||'||||||INR|INR|1|1||'
from FRA_MASTER A1;
SPOOL OFF;
ENDOFSQL

echo "End of spooling the data for FRA"
