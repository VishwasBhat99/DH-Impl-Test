#i/usr/bin/env bash

echo "Start of deleting the old UBSLOANOP.txt file"

echo "End of deleting the old UBSLOANOP.txt file"

echo "Start of spooling the UBS Loans' data"

sqlplus -s balmusr/HdFcBank13\$\# << ENDOFSQL
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL /data/oracle18c/app/product/18c/dbhome/ExportLog/UBSLOANOP.txt;
select A1.COUNTERPARTY
||'|'||C1.REFERENCE
||'|'||A1.CUSTOMER_NAME1
||'|'||A1.BRANCH
||'|'||A1.INT_RATE
||'|'||A1.ACCRUAL_FREQUENCY
||'|'||to_char(A1.BOOKING_DATE,'DD-MM-')||'20'||substr(to_char(A1.Booking_date,'YYYY'),3)
||'|'||to_char(A1.VALUE_DATE,'DD-MM-')||'20'||substr(to_char(A1.VALUE_DATE,'YYYY'),3)
||'|'||to_char(A1.MATURITY_DATE,'DD-MM-')||'20'||substr(to_char(A1.MATURITY_DATE,'YYYY'),3)
||'|'||to_char(C1.SCHEDULE_DUE_DATE,'DD-MM-')||'20'||substr(to_char(C1.SCHEDULE_DUE_DATE,'YYYY'),3)
||'|'||A1.USER_DEFINED_STATUS
||'|'||A1.PRODUCT_CODE
||'|'||A1.GL
||'|'||A1.CONTRACT_CCY
||'|'||A1.LCY_OUTSTND_BAL
||'|'||C1.Component
||'|'||C1.Amount_Due
||'|'||C1.Amount_Settled
||'|'||((C1.Amount_Due-C1.Amount_Settled)*(A1.LCY_OUTSTND_BAL/(A1.AC_CCY_OUTSTND_BAL)))
||'|'||A1.RATESPREAD
||'|'||A1.COMP_MIS_1
||'|'||A1.COMP_MIS_2
||'|'||A1.COMP_MIS_3
||'|'||A1.RATECODE
||'|'||A1.RATETYPE
||'|'||A1.BENCHMARK_RATE
||'|'||A1.NEXT_RESET_DATE
||'|'||A1.LAST_RESET_DATE    
from INP001_LOANS_FC A1, INP003_FC_LOANS04 C1
WHERE A1.CONTRACT_REF_NO=C1.REFERENCE 
and C1.COMPONENT IN ('PRINCIPAL', 'MAIN_INT') 
and C1.AMOUNT_DUE-C1.AMOUNT_SETTLED!=0 
and A1.AC_CCY_OUTSTND_BAL!=0  
ORDER BY C1.REFERENCE;
SPOOL OFF;
ENDOFSQL

echo "End of spooling the UBS Loans' data"
