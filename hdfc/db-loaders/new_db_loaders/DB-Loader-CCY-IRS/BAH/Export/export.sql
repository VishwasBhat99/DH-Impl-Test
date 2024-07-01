set termout off
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL $PREPROCESS/BAH/&1/ccy-irs.txt;
select A1.Entity
||'|'||A1.TradeId
||'|'||A1.Contractid
||'|'||A1.StructureIdLinkId
||'|'||A1.ComponentTypology
||'|'||A1.PackageTypology
||'|'||A1.ContractTypology
||'|'||A1.Desk
||'|'||A1.Book
||'|'||A1.Folder
||'|'||A1.TradingBanking
||'|'||A1.CounterpartyGroupCode
||'|'||A1.CounterpartyChildCode
||'|'||A1.CounterpartyName
||'|'||A1.InternalExternal
||'|'||TO_CHAR(A1.TradeDate,'DD-MM-')||'20'||substr(to_char(A1.TradeDate,'YYYY'),3)
||'|'||TO_CHAR(A1.StartDate,'DD-MM-')||'20'||substr(to_char(A1.StartDate,'YYYY'),3)
||'|'||TO_CHAR(A1.EndDate,'DD-MM-')||'20'||substr(to_char(A1.EndDate,'YYYY'),3)
||'|'||A1.CurrencyPair
||'|'||A1.RecLegCurrency
||'|'||A1.OriginalNotionalRecLeg
||'|'||A1.OriginalNotionalRecLeginINR
||'|'||A1.OutstandingNotionalRecLeg
||'|'||A1.OutstandingNotionalRecLegInINR
||'|'||A1.PayLegCurrency
||'|'||A1.OriginalNotionalPayLeg
||'|'||A1.OriginalNotionalPayLegInINR
||'|'||A1.OutstandingNotionalPayLeg
||'|'||A1.OutstandingNotionalPayLeginINR
||'|'||A1.DealSide
||'|'||A1.PayLegIndex
||'|'||A1.PayIntRate
||'|'||A1.SpreadPayleg
||'|'||A1.RecLegIndex
||'|'||A1.RecIntRate
||'|'||A1.SpreadRecleg
||'|'||A1.RecSideAccuralInINR
||'|'||A1.RecSideMTMInINR
||'|'||A1.FutureCashProceedsCurrency
||'|'||A1.FutureCashProceedsINR
||'|'||A1.MarketValueFinanced
||'|'||A1.NetMTMinUSD
||'|'||A1.NetMTMinINR
||'|'||A1.PaySidePV01InINR
||'|'||A1.RecSidePV01inINR
||'|'||A1.NetPV01inINR
||'|'||A1.PaySideModifiedDuration
||'|'||A1.ReceiveSideModifiedDuration
||'|'||A1.ModifiedDurationofthedeal
||'|'||A1.PaylegExchangeRate
||'|'||A1.ReclegExchangeRate
||'|'||TO_CHAR(A1.PayResetDate,'DD-MM-')||'20'||substr(to_char(A1.PayResetDate,'YYYY'),3)
||'|'||TO_CHAR(A1.RecResetDate,'DD-MM-')||'20'||substr(to_char(A1.RecResetDate,'YYYY'),3)
||'|'||TO_CHAR(A1.PayPaymentDate,'DD-MM-')||'20'||substr(to_char(A1.PayPaymentDate,'YYYY'),3)
||'|'||TO_CHAR(A1.RecPaymentDate,'DD-MM-')||'20'||substr(to_char(A1.RecPaymentDate,'YYYY'),3)
||'|'||A1.IndexRecLeg
||'|'||A1.IndexPayLeg
||'|'||A1.DayCountConventionRecleg
||'|'||A1.DayCountConventionPayLeg
||'|'||A1.PayResetFrequency
||'|'||A1.RecResetFrequency
||'|'||A1.PayPaymentFrequency
||'|'||A1.RecPaymentFrequency
||'|'||A1.DealStatus
||'||||||'||V1.FLOWAMOUNT
||'|'||TO_CHAR(V1.CASHFLOWDATE,'DD-MM-')||'20'||substr(to_char(V1.CASHFLOWDATE,'YYYY'),3)
||'|'||V1.FLOWCURRENCY||'|||||'
from CCY_IRS_SWAP_MASTER A1 
INNER JOIN VW_CCY_IRS_SWAP_CASHFLOW V1 ON A1.TradeID=V1.Trade
ORDER BY A1.Entity;
SPOOL OFF;
set termout on
