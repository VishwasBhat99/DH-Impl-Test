set termout off
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL $PREPROCESS/BAH/&1/fcy-irs.txt;
select A1.Entity
||'|'||A1.TradeId
||'|'||A1.Contractid
||'|'||A1.Folder
||'|'||A1.TradingBanking
||'|'||A1.InternalExternal
||'|'||A1.CounterpartyName
||'|'||TO_CHAR(A1.TradeDate,'DD-MM-')||'20'||substr(to_char(A1.TradeDate,'YYYY'),3)
||'|'||TO_CHAR(A1.StartDate,'DD-MM-')||'20'||substr(to_char(A1.StartDate,'YYYY'),3)
||'|'||TO_CHAR(A1.EndDate,'DD-MM-')||'20'||substr(to_char(A1.EndDate,'YYYY'),3)
||'|'||A1.DealCurrency
||'|'||A1.OriginalNotional
||'|'||A1.OriginalNotionalinINR
||'|'||A1.RecCurrentNotional
||'|'||A1.RecCurrentNotionalinINR
||'|'||A1.PayCurrentNotional
||'|'||A1.PayCurrentNotionalinINR
||'|'||A1.ContingentNotional
||'|'||A1.DealSide
||'|'||A1.PayLegIndex
||'|'||A1.PayIntRate
||'|'||A1.SpreadPayleg
||'|'||A1.RecLegIndex
||'|'||A1.RecIntRate
||'|'||A1.SpreadRecleg
||'|'||A1.PaySideAccrual
||'|'||A1.PaySideMTM
||'|'||A1.PaySideGMTM
||'|'||A1.RecSideAccrual
||'|'||A1.NetAccrualinINR
||'|'||A1.NetAccrualinUSD
||'|'||A1.FutureCashProceedsCurrency
||'|'||A1.FutureCashProceeds
||'|'||A1.FutureCashProceedsINR
||'|'||A1.NETMTM
||'|'||A1.NETMTMinINR
||'|'||A1.NETMTMinUSD
||'|'||A1.NETGMTMinINR
||'|'||A1.NETGMTMinUSD
||'|'||A1.NetBCVAadjustedGMTMinINR
||'|'||A1.PaySidePV01inINR
||'|'||A1.RecSidePV01inINR
||'|'||A1.NetPV01inINR
||'|'||A1.PaySideModifiedDuration
||'|'||A1.ExchangeRate
||'|'||TO_CHAR(A1.PayResetDate,'DD-MM-')||'20'||substr(to_char(A1.PayResetDate,'YYYY'),3)
||'|'||TO_CHAR(A1.RecResetDate,'DD-MM-')||'20'||substr(to_char(A1.RecResetDate,'YYYY'),3)
||'|'||TO_CHAR(A1.PayPaymentDate,'DD-MM-')||'20'||substr(to_char(A1.PayPaymentDate,'YYYY'),3)
||'|'||TO_CHAR(A1.RecPaymentDate,'DD-MM-')||'20'||substr(to_char(A1.RecPaymentDate,'YYYY'),3)
||'|'||A1.OriginalTenor
||'|'||A1.ResidualTenor
||'|'||A1.PayResetFrequency
||'|'||A1.RecResetFrequency
||'|'||A1.PayPaymentFrequency
||'|'||A1.RecPaymentFrequency
||'|'||A1.DealStatus
||'|'||A1.InputId
||'|'||A1.AuthoriserID
||'||||||||'||V1.FLOWAMOUNT
||'|'||TO_CHAR(V1.CASHFLOWDATE,'DD-MM-')||'20'||substr(to_char(V1.CASHFLOWDATE,'YYYY'),3)
||'|'||V1.FLOWCURRENCY||'|||||'
from FCY_IRS_SWAP_MASTER A1
INNER JOIN VW_FCY_IRS_SWAP_CASHFLOW V1 ON A1.TradeId=V1.TRADE
ORDER BY V1.TRADE;
SPOOL OFF;
set termout on
