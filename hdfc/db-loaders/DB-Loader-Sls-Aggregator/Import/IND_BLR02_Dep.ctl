LOAD DATA
INFILE '../sls.txt' "STR '\n'"
APPEND
INTO TABLE IND_BLR02_Dep FIELDS TERMINATED BY '|'
 TRAILING NULLCOLS 
(
CountryCd,
AsOnDt,
CurrencyId,
DepAmtCcy,
DepAmtHcy
)
