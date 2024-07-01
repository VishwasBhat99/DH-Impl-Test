OPTIONS (SKIP=1)
LOAD DATA
INFILE '../cashflows.csv' "STR '\r\n'"
TRUNCATE
INTO TABLE INR_IRS_SWAP_CASHFLOW FIELDS TERMINATED BY ','
 TRAILING NULLCOLS 
(
Amt,
Book,
Ccy,
Company,
Cust,
Desk,
DmOwnerTable,
EvTyp,
ExtTradeId,
GeneratedPk,
IndexChar,
Notional,
OrigCcy,
SettleCcy,
TradeId,
ValDate DATE "YYYY-MM-DD hh24:mi:ss",
JobStrmId,
JobRunId,
BusinessDt DATE "YYYY-MM-DD hh24:mi:ss",
LoadTs DATE "YYYY-MM-DD hh24:mi:ss"
)
