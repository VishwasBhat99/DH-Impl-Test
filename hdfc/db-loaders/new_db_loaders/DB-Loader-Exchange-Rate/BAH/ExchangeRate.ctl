OPTIONS (SILENT=FEEDBACK)
LOAD DATA
INFILE '*' "STR '\n'"
APPEND
INTO TABLE "CurrencyConversion" FIELDS TERMINATED BY '|'
 TRAILING NULLCOLS 
(
"ReferenceCurrency_ID",
"Currency_ID",
"Conversion_Rate",
"As_On" DATE "DD-MM-YYYY",
"IsActive",
"CreID",
"CreTime" DATE "DD-MM-YYYY",
"ModID",
"ModTime" DATE "DD-MM-YYYY"
)
