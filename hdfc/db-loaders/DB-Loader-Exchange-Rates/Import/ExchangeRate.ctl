OPTIONS (SILENT=FEEDBACK)
LOAD DATA
INFILE '*' "STR '\n'"
APPEND
INTO TABLE "CurrencyConversion" FIELDS TERMINATED BY '|'
 TRAILING NULLCOLS 
(
"ReferenceCurrency_ID" DATE,
"Currency_ID",
"Conversion_Rate",
"As_On",
"IsActive",
"CreID",
"CreTime",
"ModID",
"ModTime"
)
