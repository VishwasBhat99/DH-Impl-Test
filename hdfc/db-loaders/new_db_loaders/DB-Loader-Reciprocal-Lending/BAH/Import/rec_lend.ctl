OPTIONS (SKIP=1, silent=feedback)
LOAD DATA
INFILE '*' "STR '\n'"
TRUNCATE
INTO TABLE inp_reciprocal_lmt_sanction FIELDS TERMINATED BY ','
 TRAILING NULLCOLS 
(
C_PARTY, 
CCY,
TYP, 
SANC_AMT,
ST_DT date "dd-MON-YY",     
ED_DT date "dd-MON-YY",        
COUNTRY
)
