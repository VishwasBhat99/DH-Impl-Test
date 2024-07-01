OPTIONS (SILENT=FEEDBACK)
LOAD DATA
INFILE '../sig-cntrprty-deposits.txt' "STR '\n'"
APPEND
INTO TABLE IND_BLR02A11 FIELDS TERMINATED BY '|'
 TRAILING NULLCOLS 
(
country_code,
ason_date DATE,
currency_id,
ucic_id,
ucic_name,
tot_bal_lcy,
tot_bal_hcy
)
