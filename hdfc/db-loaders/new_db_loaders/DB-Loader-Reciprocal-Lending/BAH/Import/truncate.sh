#!/usr/bin/env bash

sqlplus -s $CON_STR_BH << EOF

truncate table inp_reciprocal_lmt_sanction;

exit
EOF
