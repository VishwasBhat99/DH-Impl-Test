OPTIONS (SKIP=1)
LOAD DATA
INFILE '../master.csv' "STR '\r\n'"
TRUNCATE
INTO TABLE FRA_MASTER FIELDS TERMINATED BY ','
 TRAILING NULLCOLS 
(
GenPK,
AuthUser,
BAnkOrNonBank,
Book,
CountryPartyId,
CountryPartyName,
DealStatus,
Desk,
EndDate DATE ,
FixedRate DATE,
FixingDate DATE,
IndexName DATE,
InputUser,
MtmFCY,
MtmInr,
NotionalCcy,
NotionalInr,
OsNotionalInrPay,
OsNotionalInrRec,
OsNotionalUsdPay,
OsNotionalUsdRec,
Pv01Inr,
ParentId,
PorS.
SettlementDate DATE,
StartDate DATE,
StructId,
TradeDate,
TradeId,
traderId,
MarketerId,
JobStrmId,
JobRunId,
BusinessDate timestamp,
LoadTs timestamp
)
