#!/usr/bin/env bash

sqlplus -s $CON_STR_IND << ENDOFSQL
@/home/dbuser/programs/DB-Loader-GL-NPA/Export/export.sql $INPUT $1
ENDOFSQL

echo "End of spooling the data"

file_path=$( ls $INPUT/IND/$1/GLCF.txt )
total=$( wc -l $file_path | awk '{print $1}' )

/home/dbuser/programs/IND/health_checker --output-file $file_path -a $total -s $total 
