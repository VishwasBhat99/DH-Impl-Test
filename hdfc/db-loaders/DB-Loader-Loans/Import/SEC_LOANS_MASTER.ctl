OPTIONS (SKIP=3)
LOAD DATA
INFILE '../master-loans.csv' "STR '\r\n'"
TRUNCATE
INTO TABLE SEC_LOANS_MASTER FIELDS TERMINATED BY ','
 TRAILING NULLCOLS 
(
val1,
UBSACCOUNTNUMBER,
Cust_Id,
FC_UBSACC,
DEALNAME,
PRINCIPALOUTSTANDING,	
DEALSTARTDATE DATE,
CFSTARTDATE DATE,
CFENDDATE DATE,
Remarks,
AccruedInterest,
COMPOUNDINGFREQUENCY,
DEALVALUE,
GL,
System,
CURRENTNOMINALINTERESTRATE_Bookedinthesystem,
Ratings,
RATINGWHETHERDOWNGRADED,
RATINGAGENCY,
AssetClass, 
Division,
Type,
Originator,
BalanceWAM,
BalanceDTDMaturity,
ContractYield,
CURRENTANNUALSEDYIELD,
ResetFrequency,
InterestRateType,
Structure,
Expectedrateresetdate DATE,
CommentsRevisedFutureCashflows
)
