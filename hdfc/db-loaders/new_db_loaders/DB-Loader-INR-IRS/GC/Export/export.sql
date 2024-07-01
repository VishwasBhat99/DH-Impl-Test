set termout off
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL $PREPROCESS/GC/&1/inr-irs.txt;
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
||'||||||||'||V1.FLOWAMOUNT
||'|'||TO_CHAR(V1.CASHFLOWDATE,'DD-MM-')||'20'||substr(to_char(V1.CASHFLOWDATE,'YYYY'),3)
||'|'||V1.FLOWCURRENCY||'|||||'
from INR_IRS_SWAP_MASTER A1 INNER JOIN VW_INR_IRS_SWAP_CASHFLOW V1
ON A1.TradeId=V1.TRADE
ORDER BY V1.TRADE;
SPOOL OFF;
set termout on
