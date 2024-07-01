#!/usr/bin/env bash

echo "Start of dleting the old INRIRSSWAP.txt file"

rm /data/oracle18c/app/product/18c/dbhome/ExportLog/INRIRSSWAP.txt

echo "End of dleting the old INRIRSSWAP.txt file"

echo "Start of spooling the data for INR IRS SWAP"

sqlplus -s balmusr/HdFcBank13\$\# << ENDOFSQL
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL /data/oracle18c/app/product/18c/dbhome/ExportLog/INRIRSSWAP.txt;
select A1.TradeId
||'|'||to_char(A1.StartDate,'DD-MM-')||'20'||substr(to_char(A1.startdate,'YYYY'),3)
||'|'||to_char(A1.EndDate,'DD-MM-')||'20'||substr(to_char(A1.dealstartdate,'YYYY'),3)
||'|'||A1.PayCurrentNotional
||'|'||A1.RecCurrentNotional
||'|'||C1.Amount
||'|'||TO_CHAR(C1.Valdate,'DD-MM-')||'20'||substr(to_char(A1.ValDate,'YYYY'),3)
||'|'||A1.AccruedInterest
||'|'||to_char(A1.TradeDate,'DD-MM-')||'20'||substr(to_char(A1.TradeDate,'YYYY'),3)
||'||'||A1.PV01
||'||'||A1.MTM
||'|'||A1.BankOrNonBank
||'|'||A1.Book
||'|'||A1.Desk
||'|'||A1.PayIntRate
||'|'||A1.RecIntRate
||'|||'||A1.CustTyp
||'|'||to_char(A1.Paypaymentdate,'DD-MM-')||'20'||substr(to_char(A1.PayPaymentDate,'YYYY'),3)
||'|'||to_char(A1.PayResetDate,'DD-MM-')||'20'||substr(to_char(A1.PayResetdate,'YYYY'),3)
||'|'||to_char(A1.RecPaymentDate,'DD-MM-')||'20'||substr(to_char(A1.RecPaymentDate,'YYYY'),3)
||'|'||to_char(A1.RecResetdate,'DD-MM-')||'20'||substr(to_char(A1.RecResetDate,'YYYY'),3)
||'|INR|INR|1|1||'
from INR_IRS_SWAP_MASTER A1 INNER JOIN INR_IRS_SWAP_CASHFLOW C1
ON A1.TradeId=C1.TradeId
ORDER BY C1.TradeId;
SPOOL OFF;
ENDOFSQL

echo "End of spooling the data for INR IRS SWAP"
