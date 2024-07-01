set termout off
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL $PREPROCESS/GC/&1/cap-floor.txt;
select A1.entity
||'|'||A1.trade_id
||'|'||A1.structureid_link
||'|'||A1.component_typo
||'|'||A1.contract_type
||'|'||A1.package_typo
||'|'||A1.desk
||'|'||A1.book
||'|'||A1.folder
||'|'||A1.trading_banking
||'|'||A1.internal_external
||'|'||A1.counterparty_group_code
||'|'||A1.counterparty_parent_code
||'|'||A1.counterparty_child_code
||'|'||A1.bank_non_bank
||'|'||TO_CHAR(A1.trade_date,'DD-MM-')||'20'||substr(to_char(A1.trade_date,'YYYY'),3)
||'|'||TO_CHAR(A1.maturity_date,'DD-MM-')||'20'||substr(to_char(A1.maturity_date,'YYYY'),3)
||'|'||A1.buy_sale
||'|'||A1.underlying_index
||'|'||A1.notional_currency
||'|'||A1.original_notional_amount
||'|'||A1.mtm_in_inr
||'|'||A1.net_pv01_in_inr
||'|'||A1.modified_duration_Of_the_deal
||'|'||A1.reset_frequency
||'|'||TO_CHAR(A1.next_reset_date,'DD-MM-')||'20'||substr(to_char(A1.next_reset_date,'YYYY'),3)
||'|'||A1.underlying_pp
||'|'||A1.deal_status
||'||||||'||V1.flowamount
||'|'||TO_CHAR(V1.cashflowdate,'DD-MM-')||'20'||substr(to_char(V1.cashflowdate,'YYYY'),3)
||'|'||V1.flowcurrency
from CAP_FLOOR_MASTER A1 
INNER JOIN CAP_FLOOR_CF_VIEW V1 ON A1.trade_id=V1.trade
ORDER BY A1.entity;
SPOOL OFF;
set termout on
