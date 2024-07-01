#!/usr/bin/env bash

input_date=$2

dd=$(echo "$input_date" | awk -F'-' '{print $1}')
mm=$(echo "$input_date" | awk -F'-' '{print $2}')
yyyy=$(echo "$input_date" | awk -F'-' '{print $3}')

#Get the previous-year.
prev_yyyy=$((yyyy-1));

#Initialization
quarter_dd=$dd
quarter_mm=$mm
quarter_yyyy=$yyyy

if [[ "$mm" == "01" || "$mm" == "02" || ( "$mm" == "03" && "$dd" != "31" ) ]];
	then 
		quarter_dd="31"
	       	quarter_mm="12"
	       	quarter_yyyy=$prev_yyyy
	elif [[ "$mm" == "04" || "$mm" == "05" || ( "$mm" == "06" && "$dd" != "30" ) ]];
        then
		quarter_dd="31"
                quarter_mm="03"
	elif [[ "$mm" == "07" || "$mm" == "08" || ( "$mm" == "09" && "$dd" != "30" ) ]];
        then
		quarter_dd="30"
                quarter_mm="06"
	elif [[ "$mm" == "10" || "$mm" == "11" || ( "$mm" == "12" && "$dd" != "31" ) ]];
        then
		quarter_dd="30"
                quarter_mm="09"
fi

quarter_end_date="$quarter_dd-$quarter_mm-$quarter_yyyy";

echo "Quarter end date: $quarter_end_date"

