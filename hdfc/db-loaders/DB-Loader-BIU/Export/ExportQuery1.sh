#!/usr/bin/env bash

echo "Start of spooling the BIU Data 1"

sqlplus -s CON_STR_BASEL_IND << ENDOFSQL

SPOOL /data3/Release/biu-23-Oct-2019-1.txt;
select COD_CUST
||'|'||SAL_NEW1
||'|'||OTHER
||'|'||T3
||'|'||NATURE_OF_BUS
||'|'||TXT_BUS_DESC
||'|'||TRANSX_FLAG_4_OTHERS   
from BIU_S1;
SPOOL OFF;
ENDOFSQL

echo "End of spooling the BIU Data 1"
