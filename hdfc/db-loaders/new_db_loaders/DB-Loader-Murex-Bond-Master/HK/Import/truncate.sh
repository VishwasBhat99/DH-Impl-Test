#!/usr/bin/env bash

sqlplus -s $CON_STR_HK << EOF

truncate table bond_master;

exit
EOF
