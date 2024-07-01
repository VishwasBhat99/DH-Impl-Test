#!/usr/bin/env bash

empty_dep=$(<murex-derivatives.json)

sqlplus -s $CON_STR_IND << EOF
update streamdef set streamdesc = utl_raw.cast_to_raw ('$empty_dep') where sreamid = 1052;
commit;

exit
EOF
