#!/usr/bin/env bash

sqlplus -s $CON_STR_GC << EOF

truncate table bond_master;

exit
EOF
