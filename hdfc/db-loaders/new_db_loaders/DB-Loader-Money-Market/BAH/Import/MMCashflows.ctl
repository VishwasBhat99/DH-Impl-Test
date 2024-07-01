OPTIONS (SKIP=1, SILENT=FEEDBACK)
LOAD DATA
INFILE '*' "STR '\n'"
TRUNCATE
INTO TABLE MONEY_MARKET_CASHFLOW FIELDS TERMINATED BY ','
 TRAILING NULLCOLS 
(
Deal_ID,
Entity,
Instrument_Name,
Opertype,
Typology,
Usage1,
Sub_Type,
Counterparty,
Creation_Date,
Val_Date,
Deal_Date,
Curr,
Deal_Amount,
Exchange_Rate,
Lcy_Amount,
Roi,
Tenor_Days,
Maturity_Dt,
Interest_Amount,
Flow_Type,
Maturity_Amount,
Dealer_Name,
Ndsreferenceno,
Next_Reset_Date,
Residual_Tenor,
Next_Put_Date,
Next_Call_Date,
Next_Interest_Payout_Date,
Interest_Payout_Tenor,
Air_Aip,
Downgrade_clause,
Average_Monthly_Balance,
GLCode,
Counterparty_Category_1,
Counterparty_Category_2,
Counterparty_Category_3,
Counterparty_Category_4,
Interest_payable_receivable_till_the_next_payout_date,
Bucket_Days
)
