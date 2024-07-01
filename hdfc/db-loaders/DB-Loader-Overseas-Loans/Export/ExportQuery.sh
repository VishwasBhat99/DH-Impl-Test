#!/usr/bin/env bash

echo "Start of dleting the old over-loans.txt file"

rm /data/oracle18c/app/product/18c/dbhome/ExportLog/over-loans.txt

echo "End of dleting the old over-loans.txt file"

echo "Start of spooling the data for Overseas Loans"

sqlplus -s balmusr/HdFcBank13\$\# << ENDOFSQL
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL /data/oracle18c/app/product/18c/dbhome/ExportLog/over-loans.txt;
select A1.CUSTOMER
||'|'||A1.REFERENCE
||'|'||A1.CUSTOMER_NAME
||"|1|"||A1.NORM_INT_RATE
||'|'||A1.NORM_INT_RATE
||"||"||TO_CHAR(A1.BOOKING_DATE,'DD-MM-')||'20'||substr(to_char(A1.BOOKING_DATE,'YYYY'),3)
||'|'||TO_CHAR(A1.VALUEDATE,'DD-MM-')||'20'||substr(to_char(A1.VALUEDATE,'YYYY'),3)
||'|'||TO_CHAR(A1.MATURITY_DATE,'DD-MM-')||'20'||substr(to_char(A1.MATURITY_DATE,'YYYY'),3)
||'|'||TO_CHAR(C1.DUE_DATE,'DD-MM-')||'20'||substr(to_char(C1.DUE_DATE,'YYYY'),3)
||'|'||A1.STATUS
||'|'||A1.PRODUCT_CODE
||'|'||A1.GL
||'|'||A1.CURRENCY
||'|'||A1.LCY_OUTSTANDING_AMOUNT_USD
||'|'||A1.COMPONENT
||'|'||A1.AMOUNT_DUE
||'|'||A1.AMOUNT_SETTLED
||'|'||A1.AMOUNT_DUE - A1.AMOUNT_SETTLED
||'|'||A1.LIBOR_SPREAD
||'|'||A1.DIVISION
||"|||"||A1.RATE_TYPE
||'|'||A1.RATE_CODE
||'|'||TO_CHAR(A1.LAST_RESET_DATE,'DD-MM-')||'20'||substr(to_char(A1.LAST_RESET_DATE,'YYYY'),3)
||'|'||TO_CHAR(A1.NEXT_RESET_DATE,'DD-MM-')||'20'||substr(to_char(A1.NEXT_RESET_DATE,'YYYY'),3)
from OVER_LOANS_MASTER A1 INNER JOIN OVER_LOANS_CASHFLOW C1
ON A1.REFERENCE=C1.REFERENCE
ORDER BY C1.REFERENCE;
SPOOL OFF;
ENDOFSQL

echo "End of spooling the data for Overseas Loans"
