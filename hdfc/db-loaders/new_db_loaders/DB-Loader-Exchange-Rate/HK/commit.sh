#!/usr/bin/env bash

sqlplus -s $CON_STR_HK << EOF

@/home/dbuser/programs/DB-Loader-Exchange-Rate/HK/commit.sql

exit
EOF
