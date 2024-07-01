#!/usr/bin/env bash

sqlplus -s $CON_STR_HK << EOF

truncate table FCY_IRS_SWAP_MASTER;

exit
EOF
