#!/usr/bin/env bash

echo "Start of spooling the Finnone Loans data"

sqlplus -s balmusr/HdFcBank13\$\# << ENDOFSQL
@export.sql;
ENDOFSQL

echo "End of spooling the Finnone Loans data"
