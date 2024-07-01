#!/usr/bin/env bash

sqlplus -s $CON_STR_BALMUSR << EOF

@commit.sql

exit
EOF
