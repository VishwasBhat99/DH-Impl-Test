#!/usr/bin/env bash

MAP_MASTER=$"$RULES/IND/OPERATIONAL-FILES/mapping_master.txt"
INPUT_FILE=$"$INPUT/IND/$1/FC_SIBGSTT.TXT""
OUTPUT_FILE=$"$PREPROCESS/IND/$1/pp-out-gl-gstt.txt"

awk -F'|' 'BEGIN {OFS="|"} NR==FNR {b[$1]=$2"|"$3"|"$5"|"$6; next} {diff=$4-$5; printf "%s|%s|%s|%s|%s|%.0f|%s\n", $1, $2, $3, $4, $5, diff, ($2 in b) ? b[$2] : "|||"}' $MAP_MASTER $INPUT_FILE > $OUTPUT_FILE

