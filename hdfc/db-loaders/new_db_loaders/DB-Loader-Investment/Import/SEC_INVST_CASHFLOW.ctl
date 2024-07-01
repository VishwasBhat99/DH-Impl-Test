OPTIONS (SKIP=2, silent=feedback)
LOAD DATA
INFILE '*' "STR '\n'"
TRUNCATE
INTO TABLE SEC_INVST_CASHFLOW FIELDS TERMINATED BY ','
 TRAILING NULLCOLS 
(
Val1,
val2,
Date1 date ,
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
