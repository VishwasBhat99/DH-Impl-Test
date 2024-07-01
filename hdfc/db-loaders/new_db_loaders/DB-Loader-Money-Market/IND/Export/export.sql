
set termout off
set linesize 2000 trims on  pagesize 0 feedback off;
SPOOL &1/IND/&2/money-market.txt
select A1.Deal_Id
||'~'||A1.Branch
||'~'||A1.Instrument_Name
||'~'||A1.Lending_Borrowing_Type
||'~'||A1.Typology
||'~'||A1.Usage1
||'~'||A1.Sub_type_borrowing_lending
||'~'||A1.Counterparty
||'~'||TO_CHAR(A1.Creation_Date,'DD-MM-')||'20'||substr(to_char(A1.Creation_Date,'YYYY'),3)
||'~'||TO_CHAR(A1.Val_Date,'DD-MM-')||'20'||substr(to_char(A1.Val_Date,'YYYY'),3)
||'~'||TO_CHAR(A1.Deal_Date,'DD-MM-')||'20'||substr(to_char(A1.Deal_Date,'YYYY'),3)
||'~'||A1.Currency
||'~'||A1.Current_Deal_Amount
||'~'||A1.Current_Conversion_Rate_LCY
||'~'||A1.Current_Deal_Amount_LCY
||'~'||A1.RoI
||'~'||A1.Tenor_Days
||'~'||TO_CHAR(A1.Maturity_Dt,'DD-MM-')||'20'||substr(to_char(A1.Maturity_Dt,'YYYY'),3)
||'~'||case when C1.Flow_Type='CAP' THEN
		C1.Deal_Amount
		ELSE C1.Interest_Amount
		END
||'~'||C1.Flow_Type
||'~'||C1.Flow_Type
||'~'||A1.Maturity_Amount
||'~'||A1.Dealer_Name
||'~'||A1.Ndsreferenceno
||'~'||TO_CHAR(A1.Next_Fixing_Date,'DD-MM-')||'20'||substr(to_char(A1.Next_Fixing_Date,'YYYY'),3)
||'~'||C1.Residual_Tenor
||'~'||TO_CHAR(C1.Next_Put_Date,'DD-MM-')||'20'||substr(to_char(C1.Next_Put_Date,'YYYY'),3)
||'~'||TO_CHAR(C1.Next_Call_Date,'DD-MM-')||'20'||substr(to_char(C1.Next_Call_Date,'YYYY'),3)
||'~'||TO_CHAR(C1.Next_Interest_Payout_Date,'DD-MM-')||'20'||substr(to_char(C1.Next_Interest_Payout_Date,'YYYY'),3)
||'~'||C1.Interest_Payout_Tenor
||'~'||A1.AIP_AIR
||'~'||C1.Downgrade_clause
||'~'||C1.Average_Monthly_Balance
||'~'||C1.GLCode
||'~'||A1.Counterparty_Category_1
||'~'||A1.Counterparty_Category_2
||'~'||A1.Counterparty_Category_3
||'~'||A1.Counterparty_Category_4
||'~'||C1.Interest_payable_receivable_till_the_next_payout_date
||'~'||C1.Bucket_Days
from MONEY_MARKET_MASTER A1 INNER JOIN MONEY_MARKET_CASHFLOW C1
ON A1.Deal_Id=C1.Deal_ID
WHERE A1.Branch= 'INDIA_CE'
ORDER BY C1.Deal_ID;
SPOOL OFF;


