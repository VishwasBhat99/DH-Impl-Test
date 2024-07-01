#!/usr/bin/env bash

sqlplus -s $CON_STR_IND  << EOF

truncate table MUREX_INSTRUMENTS_SECURITY_COMPOSITION;
truncate table MUREX_SEC_COMP_CF;

exit
EOF
