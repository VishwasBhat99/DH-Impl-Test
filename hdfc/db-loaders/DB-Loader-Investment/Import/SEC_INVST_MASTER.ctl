OPTIONS (SKIP=3)
LOAD DATA
INFILE '../master-inv.csv' "STR '\r\n'"
TRUNCATE
INTO TABLE SEC_INVST_MASTER FIELDS TERMINATED BY ','
 TRAILING NULLCOLS 
(
UBSACCOUNTNUMBER,
Cust_Id,
FC_UBSACC,
DEALNAME,
PRINCIPALOUTSTANDING,	
DEALSTARTDATE DATE ,
CFSTARTDATE DATE ,
CFENDDATE DATE ,
Remarks,
AccruedInterest,
COMPOUNDINGFREQUENCY,
DEALVALUE,
GL,
System,
CNIRB,
Ratings,
RATINGWHETHERDOWNGRADED,
RATINGAGENCY,
AssetClass, 
Division,
IType,
Originator,
BalanceWAM,
BalanceDTDMaturity,
ContractYield,
CURRENTANNUALSEDYIELD,
ResetFrequency,
InterestRateType,
Structure,
Expectedrateresetdate,
CRFCFs,
Portfolio_Type,
OldFCUBSACC
)
