LOAD DATA
INFILE '*' "STR '\n'"
APPEND
INTO TABLE "BALMInputTotals" FIELDS TERMINATED BY '|'
(
"CountryID",
"BALMLLGID",
"AsOnDt",
"CcyID",
"SLRorIRS",
"SchemeID",
"CashflowType",
"Amount",
"InterestRate"
)
