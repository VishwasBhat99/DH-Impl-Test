#!/usr/bin/env bash

sqlplus -s $CON_STR_BH << EOF

truncate table bond_master;

exit
EOF
