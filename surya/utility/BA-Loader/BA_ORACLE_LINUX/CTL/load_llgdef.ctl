LOAD DATA
INFILE '*' "STR '\n'"
APPEND
INTO TABLE "BALMProductDef" FIELDS TERMINATED BY '|'
(
"CountryID",
"LLGID",
"LLGDesc"
)