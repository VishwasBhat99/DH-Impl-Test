#!/usr/bin/env bash

sqlplus -s $CON_STR_GC << EOF

truncate table FCY_IRS_SWAP_MASTER;

exit
EOF
