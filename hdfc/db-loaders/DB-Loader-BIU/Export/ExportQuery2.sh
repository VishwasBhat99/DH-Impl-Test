#!/usr/bin/env bash

echo "Start of spooling the BIU Data 2"

sqlplus -s CON_STR_BASEL_IND << ENDOFSQL

SPOOL /data3/Release/biu-23-Oct-2019-2.txt;
select COD_CUST
||'|'||SAL_TXN
||'|'||T4_FLAG
||'|'||TRANSX_FLAG_2_OTHERS
||'|'||NATURE_OF_BUS
||'|'||TXT_BUS_DESC
||'|N'|| 
from BIU_S2;
SPOOL OFF;
ENDOFSQL

echo "End of spooling the BIU Data 2"
