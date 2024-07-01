#!/usr/bin/env bash

sqlplus -s $CON_STR_IND << EOF

@/home/dbuser/programs/DB-Loader-Exchange-Rate/IND/delete.sql $1

exit
EOF
