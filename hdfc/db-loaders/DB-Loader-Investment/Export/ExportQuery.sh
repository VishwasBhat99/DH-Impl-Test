#!/usr/bin/env bash

echo "Start of deleting the old SECINVST.txt file"

rm /data/oracle18c/app/product/18c/dbhome/ExportLog/SECINVST.txt

echo "End of deleting the old SECINVST.txt file"

echo "Start of spooling securitisation investments' data"

sqlplus -s balmusr/HdFcBank13\$\# << ENDOFSQL
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL /data/oracle18c/app/product/18c/dbhome/ExportLog/Sec-Invst.txt;
select C1.Acctno
||'~'||A1.Cust_Id
||'~'||A1.FC_UBSACC
||'~'||A1.DEALNAME
||'~'||A1.PRINCIPALOUTSTANDING
||'~'||TO_CHAR(A1.DEALSTARTDATE,'DD-MM-')||'20'||substr(to_char(A1.dealstartdate,'YYYY'),3)
||'~'||TO_CHAR(A1.CFENDDATE,'DD-MM-')||'20'||substr(to_char(A1.cfenddate,'YYYY'),3)
||'~'||A1.AccruedInterest
||'~'||A1.COMPOUNDINGFREQUENCY
||'~'||A1.DEALVALUE
||'~'||A1.GL
||'~'||A1.System
||'~'||A1.CNIRB
||'~'||A1.Ratings
||'~'||A1.RATINGAGENCY
||'~'||A1.AssetClass
||'~'||A1.Division
||'~'||A1.IType
||'~'||A1.Originator
||'~'||A1.ContractYield
||'~'||A1.CURRENTANNUALSEDYIELD
||'~'||A1.ResetFrequency
||'~'||A1.InterestRateType
||'~'||A1.Expectedrateresetdate
||'~'||A1.Portfolio_Type
||'~'||TO_CHAR(C1.Date1,'DD-MM-')||'20'||substr(to_char(C1.date1,'YYYY'),3)
||'~'||C1.Interestportion
||'~'||C1.PrincipalPayment
from SEC_INVST_MASTER A1 INNER JOIN SEC_INVST_CASHFLOW C1
ON A1.UBSACCOUNTNUMBER=C1.Acctno
ORDER BY C1.Acctno;
SPOOL OFF;
ENDOFSQL

echo "End of spooling the securitisation investments' data"
