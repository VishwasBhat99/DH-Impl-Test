#!/usr/bin/env bash

sqlplus -s $CON_STR_GC << EOF

truncate table CCY_IRS_SWAP_MASTER;

exit
EOF
