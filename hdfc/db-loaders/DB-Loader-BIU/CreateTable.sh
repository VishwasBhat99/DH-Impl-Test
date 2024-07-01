#!/usr/bin/env bash

sqlplus -s $CON_STR_BASEL_IND << EOF
set echo off 
set heading off

@Create_Table_1.sql
@Create_Table_2.sql

exit
EOF
