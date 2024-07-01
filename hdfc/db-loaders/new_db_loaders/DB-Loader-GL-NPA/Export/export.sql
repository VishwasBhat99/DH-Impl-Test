set termout off
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL &1/IND/&2/GLCF.txt;
select 
trim(srcsystem)
||'|'||sum(specprov)
||'|'||asstclass
from INP001_NPA
group by srcsystem, asstclass
UNION ALL
SELECT
'LOAN'
||'|'||(SUM(NVL(TOTALPROV,0))+ SUM(NVL(NETNPA,0)))
||'|'||asstclass
from INP001_NPA
group by  asstclass;
SPOOL OFF;
set termout on
