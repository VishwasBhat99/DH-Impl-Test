OPTIONS (SKIP=1, SILENT=FEEDBACK)
LOAD DATA
INFILE '*' "STR '\n'"
TRUNCATE
INTO TABLE MONEY_MARKET_MASTER FIELDS TERMINATED BY ','
 TRAILING NULLCOLS 
(
Deal_Id,
Branch,
Instrument_Name,
Typology,
Usage1,
Lending_Borrowing_Type,
Sub_type_borrowing_lending,
Portfolio,
Desk,
Igaap_Accounting_Section,
IND_as_Accounting_Section,
Counterparty,
Creation_Date,
Val_Date,
Deal_Date,
Interest_Start_Date,
Last_Payment_Date,
Last_Fixing_Date,
Next_Payment_Date,
Next_Fixing_Date,
Currency,
Current_Conversion_Rate_LCY,
Current_Conversion_Rate_INR,
Current_Deal_Amount,
Current_Deal_Amount_LCY,
Current_Deal_Amount_INR,
Deal_Amount,
Deal_Amount_LCY,
Deal_Amount_INR,
RoI,
Interest_Rate_Type_floating_fixed,
Reset_Frequency,
Benchmark,
Tenor_Days,
Maturity_Dt,
Total_Interest,
Interest_AmountO_S,
Interest_Amount_O_S_LCY,
Interest_Amount_O_S_INR,
Maturity_Amount,
Maturity_Amount_LCY,
Maturity_Amount_INR,
Settlement_Freq,
Sett_Type,
Old_Ref_No,
AIP_AIR,
AIP_AIR_in_LCY,
AIP_AIR_in_INR,
Code,
Counterparty_Category_1,
Counterparty_Category_2,
Counterparty_Category_3,
Counterparty_Category_4,
Dealer_Name,
Dealer_Code,
Ndsreferenceno,
Payment_Through_Rtgs,
Back_office_Authorizer,
Reuter_Ref_No,
Deal_Time,
Authorizer_Name,
Deal_Status,
Practice_Day_convention_for_Interest,
Bank_Non_bank,
In_India_Outside_India,
Migrated_deals_Deal_id,
GL_Code,
Nds_Call
)
