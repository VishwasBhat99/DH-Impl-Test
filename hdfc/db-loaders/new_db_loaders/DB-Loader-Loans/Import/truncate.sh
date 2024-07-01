#!/usr/bin/env bash

sqlplus -s $CON_STR_IND << EOF

truncate table SEC_LOANS_MASTER;
truncate table SEC_LOANS_CASHFLOW;

exit
EOF
