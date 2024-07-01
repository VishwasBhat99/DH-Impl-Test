set echo off
set feedback off
set term off
set feed off
set heading off
set pages 0
set verify off
set trimspool on
set trimout on
spool &4/&3/BA/&1/balm_llgdef.txt;
select '&3'
||'|'||NVL("LLGId",0)
||'|'||NVL("LLGDesc",'NA')
FROM "LLGDef";
spool off
exit
