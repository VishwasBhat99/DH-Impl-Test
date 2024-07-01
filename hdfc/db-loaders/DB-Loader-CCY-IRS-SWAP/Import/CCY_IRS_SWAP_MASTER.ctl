OPTIONS (SKIP=1)
LOAD DATA
INFILE '../master.csv' "STR '\r\n'"
TRUNCATE
INTO TABLE CCY_INR_SWAP_MASTER FIELDS TERMINATED BY ','
 TRAILING NULLCOLS 
(
GenPK,
TradeID,
Book,
TradeDate DATE "YYYY-MM-DD hh24:mi:ss",
StartDate DATE "YYYY-MM-DD hh24:mi:ss",
EndDate DATE "YYYY-MM-DD hh24:mi:ss",
CounterpartyId,
CounterpartyName,
NotionalAmt,
PayCurrentNotitional,
RecCurrentNotitional,
RecOrPay,
PayIntRate,
RecIntRate,
NetAccural,
MTM,
GrandMtm,
PV01,
PayResetDate DATE "YYYY-MM-DD hh24:mi:ss",
RecResetDate DATE "YYYY-MM-DD hh24:mi:ss",
PayPaymentDate DATE "YYYY-MM-DD hh24:mi:ss",
RecPaymentDate DATE "YYYY-MM-DD hh24:mi:ss",
BankOrNonbank
Index,
Tenure,
PayResetFreq,
RecResetFreq,
PayPaymentFreq,
RecPaymentFreq,
DealStatus,
InputUser,
AuthorishedUser,
Desk,
Folder,
ProdGroup,
CustTyp,
TraderId,
MarketerId,
JobStrmId,
JobRunId,
BusinessDt DATE "YYYY-MM-DD hh24:mi:ss" ,
LoadTs DATE "YYYY-MM-DD hh24:mi:ss"
)
