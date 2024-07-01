#!/usr/bin/env bash

echo "Start of deleting the old inr-irs.txt file"

rm /data/oracle18c/app/product/18c/dbhome/ExportLog/inr-irs.txt

echo "End of deleting the old inr-irs.txt file"

echo "Start of spooling the data"

sqlplus -s balmusr/HdFcBank13\$\# << ENDOFSQL
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL /data/oracle18c/app/product/18c/dbhome/ExportLog/inr-irs.txt;
select A1.Entity
||'|'||A1.TradeId
||'|'||A1.Contractid
||'|'||A1.Folder
||'|'||A1.TradingBanking
||'|'||A1.InternalExternal
||'|'||A1.CounterpartyName
||'|'||TO_CHAR(A1.TradeDate,'DD-MM-')||'20'||substr(to_char(A1.TradeDate,'YYYY'),3)
||'|'||TO_CHAR(A1.StartDate,'DD-MM-')||'20'||substr(to_char(A1.StartDate,'YYYY'),3)
||'|'||A1.DealCurrency
||'|'||A1.OriginalNotional
||'|'||A1.PayIntRate
||'|'||A1.RecIntRate
||'|'||A1.ExchangeRate
||'|'||TO_CHAR(A1.PayResetDate,'DD-MM-')||'20'||substr(to_char(A1.PayResetDate,'YYYY'),3)
||'|'||TO_CHAR(A1.RecResetDate,'DD-MM-')||'20'||substr(to_char(A1.RecResetDate,'YYYY'),3)
||'|'||TO_CHAR(A1.PayPaymentDate,'DD-MM-')||'20'||substr(to_char(A1.PayPaymentDate,'YYYY'),3)
||'|'||TO_CHAR(A1.RecPaymentDate,'DD-MM-')||'20'||substr(to_char(A1.RecPaymentDate,'YYYY'),3)
||'|'||A1.PayPaymentFrequency
||'|'||A1.RecPaymentFrequency
||'|'||A1.DealStatus
||'|'||A1.InputId
||'|'||C1.Trading_Banking
||'|'||C1.M_BankingB
||'|'||C1.FLOWTYPE
||'|'||C1.FLOWTYPE1
||'|'||C1.FLOWTYPE2
||'|'||C1.FLOWTYPE3
||'|'||C1.FLOWTYPE4
||'|'||C1.FLOWAMOUNT
||'|'||TO_CHAR(C1.CASHFLOWDATE,'DD-MM-')||'20'||substr(to_char(C1.CASHFLOWDATE,'YYYY'),3)
||'|'||C1.FLOWCURRENCY
||'|'||C1.HKDRATE
||'|'||C1.HKDAMOUNT
||'|'||C1.M_H_REP_DT2
||'|'||C1.INRAMOUNT
||'|'||C1.INRRATE
from INR_IRS_SWAP_MASTER A1 INNER JOIN INR_IRS_SWAP_CASHFLOW C1
ON A1.TradeId=C1.TRADE
ORDER BY C1.M_TP_ENTITY;
SPOOL OFF;
ENDOFSQL

echo "End of spooling the data"
