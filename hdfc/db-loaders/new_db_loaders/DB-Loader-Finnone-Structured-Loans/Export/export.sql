set termout off
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL $INPUT/IND/&1/FinLoans.txt;
select M1.ACCOUNT_NUM
||'|'||M1.ACCRUALBASIS
||'|'||M1.ACCRUED_INTEREST
||'|'||M1.BRANCH
||'|'||M1.CURRENCY_CODE
||'|'||M1.CURRENT_BALANCE
||'|'||TO_CHAR(M1.DUE_DATE,'DD-MM-')||'20'||substr(to_char(M1.DUE_DATE,'YYYY'),3)
||'|'||M1.PYMT_FREQ
||'|'||M1.INTEREST_RATE
||'|'||M1.LOAN_TYPE
||'|'||TO_CHAR(M1.MATURITY_DATE,'DD-MM-')||'20'||substr(to_char(M1.MATURITY_DATE,'YYYY'),3)
||'|'||M1.ORIG_BALANCE
||'|'||M1.ORIG_TERM
||'|'||TO_CHAR(M1.ORIGINATION_DATE,'DD-MM-')||'20'||substr(to_char(M1.ORIGINATION_DATE,'YYYY'),3)
||'|'||M1.INSTALMENT
||'|'||M1.INTT_PYMT_FREQ
||'|'||M1.PYMT_TYPE
||'|'||M1.RATE_FLAG
||'|'||M1.REPRICE_INDEX
||'|'||M1.DPD
||'|'||M1.CUSTOMERNAME
||'|'||M1.SCHEME_ID
||'|'||M1.PSL
||'|'||M1.NPA_STAGE_ID
||'|'||TO_CHAR(M1.INST_START_DT,'DD-MM-')||'20'||substr(to_char(M1.INST_START_DT,'YYYY'),3)
||'|'||M1.WEAKER_CODE
||'|'||M1.CURRENTBOOKBALANCE
||'|'||TO_CHAR(M1.FIRST_INSTALLMENT_DATE,'DD-MM-')||'20'||substr(to_char(M1.FIRST_INSTALLMENT_DATE,'YYYY'),3)
||'|'||M1.INSTLNUM
||'|'||M1.NO_OF_PAID_INST
||'|'||TO_CHAR(M1.LAST_INST_RECD_DATE,'DD-MM-')||'20'||substr(to_char(M1.LAST_INST_RECD_DATE,'YYYY'),3)
||'|'||M1.INDV_CORP_FLAG
||'|'||M1.CUSTOMER_TYPE
||'|'||M1.GR_DR
||'|'||M1.GR_CR
||'|'||M1.RE_DR
||'|'||M1.RE_CR
||'|'||M1.IS_DR
||'|'||M1.IS_CR
||'|'||M1.UI_DR
||'|'||M1.UI_CR
||'|'||M1.ASSET_CLASS_ID
||'|'||M1.CUSTOMER_ID
||'|'||M1.PRODUCT_TYPE
||'|'||M1.IS_OFS_GL_CODE
||'|'||M1.GR_OFS_GL_CODE
||'|'||M1.RE_OFS_GL_CODE
||'|'||M1.UI_OFS_GL_CODE
||'|'||C1.INTEREST_AMOUNT
||'|'||C1.PRINCIPLE_AMOUNT
||'|'||TO_CHAR(C1.CASHFLOW_DATE,'DD-MM-')||'20'||substr(to_char(C1.CASHFLOW_DATE,'YYYY'),3)
from Fin_Loans_Master M1 left outer join Fin_Loans_Cashflows C1
    ON M1.ACCOUNT_NUM = C1.ACCOUNT_NUMBER where M1.CURRENCY_CODE is not null
ORDER BY C1.ACCOUNT_NUMBER;
SPOOL OFF;
set termout on 
