set echo off
set feedback off
set term off
set feed off
set heading off
set pages 0
set verify off
set trimspool on
set trimout on
spool &4/&3/BA/&1/balm_data.txt
select '&3'
||'|'|| NVL("SubType_ID",'0')
||'|'|| NVL("As_On",'NA')
||'|'|| NVL("Currency_ID",'NA')
||'|'|| NVL("SLRorIRS",'NA')
||'|'|| NVL("SchemeID",'NA')
||'|'|| NVL("CashflowType",'NA')
||'|'|| NVL("Amount",'0')
||'|'|| NVL("InterestRate",'0')
FROM "tblProductTotals" where "As_On"='&2';
spool off
exit
