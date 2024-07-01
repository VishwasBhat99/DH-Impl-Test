OPTIONS (SILENT=FEEDBACK)
LOAD DATA
INFILE '../top-n-deposits.txt' "STR '\n'"
APPEND
INTO TABLE IND_BLR02A2 FIELDS TERMINATED BY '|'
 TRAILING NULLCOLS 
(
country_code,
ason_date DATE,
currency_id,
ucic_id,
ucic_name,
sa_bal_lcy,
sa_bal_hcy
ca_bal_lcy,
ca_bal_hcy
td_bal_lcy,
td_bal_hcy
)
