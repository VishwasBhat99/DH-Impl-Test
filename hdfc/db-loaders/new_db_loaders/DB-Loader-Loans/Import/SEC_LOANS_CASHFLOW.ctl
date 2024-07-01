OPTIONS (SKIP=1, READSIZE=80000010, BINDSIZE=80000010, silent=feedback)
LOAD DATA
INFILE '*' "STR '\n'"
TRUNCATE
INTO TABLE SEC_LOANS_CASHFLOW FIELDS TERMINATED BY ','
 TRAILING NULLCOLS 
(
Date1 DATE,
OpeningBalance,
Payment,
Interestportion,
PrincipalPayment,
ClosingBalance,
Acctno,
IntRate,
DealName,
OriginatorName
)
