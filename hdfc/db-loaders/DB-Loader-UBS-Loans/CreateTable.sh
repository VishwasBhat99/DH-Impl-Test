#!/usr/bin/env bash

sqlplus -s balmusr/HdFcBank13/$/# << EOF
set echo off 
set heading off

@Create_Table_1.sql
@Create_Table_2.sql

exit
EOF
