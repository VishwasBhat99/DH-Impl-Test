#!/usr/bin/env bash

sqlplus -s $CON_STR_IND << EOF

truncate table bond_master;

exit
EOF
