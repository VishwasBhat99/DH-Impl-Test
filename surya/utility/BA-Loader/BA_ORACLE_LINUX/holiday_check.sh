#!/usr/bin/env bash

HOLIDAY_LIST=$4
is_holiday=false

while IFS= read -r dates
	do
	     	if [ $2 == $dates ]
		then is_holiday=true
		break
	    fi
	done < $HOLIDAY_LIST
echo "$is_holiday"
