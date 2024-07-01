#!/usr/bin/env bash

sqlplus -s $CON_STR_BH << EOF

@/home/dbuser/programs/DB-Loader-Exchange-Rate/BAH/commit.sql

exit
EOF
