set termout off
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL &1/IND/&2/Sec-Loans.txt;
  select
  A1.UBSACCOUNTNUMBER
  ||'~'||A1.Cust_Id
  ||'~'||A1.DEALNAME
  ||'~'||A1.PRINCIPALOUTSTANDING
  ||'~'||TO_CHAR(A1.DEALSTARTDATE,'dd-mm-')||'20'||substr(to_char(A1.dealstartdate,'yyyy'),3)
  ||'~'||TO_CHAR(A1.CFENDDATE,'dd-mm-')||'20'||substr(to_char(A1.cfenddate,'yyyy'),3)
  ||'~'||A1.AccruedInterest
  ||'~'||A1.COMPOUNDINGFREQUENCY
  ||'~'||A1.DEALVALUE
  ||'~'||A1.GL    
  ||'~'||A1.System
  ||'~'||A1.CURRENTNOMINALINTERESTRATE_Bookedinthesystem
  ||'~'||A1.Ratings
  ||'~'||A1.RATINGAGENCY
  ||'~'||A1.AssetClass
  ||'~'||A1.Division
  ||'~'||A1.Type
  ||'~'||A1.Originator
  ||'~'||A1.ContractYield
  ||'~'||A1.CURRENTANNUALSEDYIELD
  ||'~'||A1.ResetFrequency
  ||'~'||A1.InterestRateType
  ||'~'||CASE WHEN A1.Expectedrateresetdate IS NULL
  THEN (TO_CHAR(A1.CFENDDATE,'dd-mm-')||'20'||substr(to_char(A1.cfenddate,'yyyy'),3))
  ELSE
  (TO_CHAR(A1.Expectedrateresetdate,'dd-mm-')||'20'||substr(to_char(A1.expectedrateresetdate,'yyyy'),3))
  END
  ||'~'||CASE WHEN C1.Date1 IS NULL
  THEN (TO_CHAR(A1.CFENDDATE,'dd-mm-')||'20'||substr(to_char(A1.cfenddate,'yyyy'),3))
  ELSE
  (TO_CHAR(C1.Date1,'dd-mm-')||'20'||substr(to_char(C1.date1,'yyyy'),3))
  END
  ||'~'||C1.Interestportion
  ||'~'||NVL(C1.PrincipalPayment,0)
 from SEC_LOANS_MASTER A1 LEFT OUTER  JOIN SEC_LOANS_CASHFLOW C1
  ON A1.UBSACCOUNTNUMBER=C1.Acctno
WHERE 
A1.UBSACCOUNTNUMBER is not null
ORDER BY A1.UBSACCOUNTNUMBER asc;
SPOOL OFF;
set termout on
