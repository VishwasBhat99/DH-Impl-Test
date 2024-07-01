set termout off
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL &1/GC/$2/rec-lend.txt;
select CF_SUBTYPE
||'|'||C_PARTY
||'|'||CCY
||'|'||TYP
||'|'||SANC_AMT
||'|'||to_char(ST_DT,'DD-MM-')||'20'||substr(to_char(ST_DT,'YYYY'),3)
||'|'||to_char(ED_DT,'DD-MM-')||'20'||substr(to_char(ED_DT,'YYYY'),3)
||'|'||COUNTRY 
||'|'||UTIL_AMT
from vw_reciprocal_cf;
SPOOL OFF;
set termout on
