#!/usr/bin/env bash

echo "Start of deleting the old over-loans-bh.txt file"

rm /data/oracle18c/app/product/18c/dbhome/ExportLog/over-loans-bh.txt

echo "End of deleting the old over-loans-bh.txt file"

echo "Start of spooling the data"

sqlplus -s balmusr/HdFcBank13\$\# << ENDOFSQL
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL /data/oracle18c/app/product/18c/dbhome/ExportLog/over-loans-bh.txt;
select A1.CUSTOMER
||'|'||A1.REFERENCE
||'|'||A1.CUSTOMER_NAME
||'|1|'||A1.NORM_INT_RATE
||'||'||TO_CHAR(A1.BOOKING_DATE,'DD-MM-')||'20'||substr(to_char(A1.BOOKING_DATE,'YYYY'),3)
||'|'||TO_CHAR(A1.VALUEDATE,'DD-MM-')||'20'||substr(to_char(A1.VALUEDATE,'YYYY'),3)
||'|'||TO_CHAR(A1.MATURITY_DATE,'DD-MM-')||'20'||substr(to_char(A1.MATURITY_DATE,'YYYY'),3)
||'|'||TO_CHAR(C1.DUE_DATE,'DD-MM-')||'20'||substr(to_char(C1.DUE_DATE,'YYYY'),3)
||'|'||A1.STATUS
||'|'||A1.PRODUCT_CODE
||'|'||A1.GL
||'|'||A1.CURRENCY
||'|'||A1.LCY_OUTSTANDING_AMOUNT_USD
||'|'||C1.COMPONENT
||'|'||C1.AMOUNT_DUE
||'|'||C1.AMOUNT_SETTLED
||'|'||( NVL( C1.AMOUNT_DUE, 0 ) - NVL( C1.AMOUNT_SETTLED, 0 ) )*(A1.LCY_OUTSTANDING_AMOUNT_USD/(A1.fcy_outstanding_amount))
||'|'||A1.LIBOR_SPREAD
||'|'||A1.DIVISION
||'|||'||A1.RATE_TYPE
||'|'||A1.RATE_CODE
||'|'||A1.LAST_RESET_DATE
||'|'||A1.NEXT_RESET_DATE
from OVER_LOANS_MASTER_BH A1 INNER JOIN OVER_LOANS_CASHFLOW_BH C1
ON A1.REFERENCE=C1.contract_ref_no
where A1.fcy_outstanding_amount !=0 
ORDER BY C1.contract_ref_no;
SPOOL OFF;
ENDOFSQL

echo "End of spooling the data"
