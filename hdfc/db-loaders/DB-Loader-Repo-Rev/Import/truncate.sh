#!/usr/bin/env bash

sqlplus -s $CON_STR_BALMUSR << EOF

truncate table Repo_Rev_Repo;

exit
EOF
