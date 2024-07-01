#!/usr/bin/env bash

ERROR_LOG=$SH_LOGS/$3/BA/$1/ba-loader-log.txt
rm $ERROR_LOG
touch $ERROR_LOG

echo ---------------Deleting Data from Tables--------------- | tee -a $ERROR_LOG

$SCRIPTS/$3/loader-scripts/BA/delete_ba_data.sh $1 $2 $3 | tee -a $ERROR_LOG 

echo ---------------Exporting BALM LLGDef Data from BALM Tables--------------- | tee -a $ERROR_LOG

$SCRIPTS/$3/loader-scripts/BA/export_llgdef.sh $1 $2 $3 | tee -a $ERROR_LOG

echo ---------------Loading BALM LLGDef Data into BA Tables--------------- | tee -a $ERROR_LOG

$SCRIPTS/$3/loader-scripts/BA/load_llgdef.sh $1 $2 $3 | tee -a $ERROR_LOG 

echo ---------------Checking if $2 is a holiday--------------- | tee -a $ERROR_LOG

HOLIDAY_LIST=$"$SH_RULES/$3/OPERATIONAL-FILES/BA-FILES/holiday.txt"
if [ ! -f $HOLIDAY_LIST ]
then 
	echo "$HOLIDAY_LIST not found. Exiting process." | tee -a $ERROR_LOG 
	exit 1
fi

is_holiday=$($SCRIPTS/$3/loader-scripts/BA/holiday_check.sh $1 $2 $3 $HOLIDAY_LIST)

if [ "$is_holiday" == "true" ]
then
	echo "It is a holiday on $2. Data not processed." | tee -a $ERROR_LOG 
else 
	echo "It is not a holiday on $2 Processing BA data." | tee -a $ERROR_LOG 

echo ----------Extracting BALM Data----------- | tee -a $ERROR_LOG

$SCRIPTS/$3/loader-scripts/BA/export_balm_data.sh $1 $2 $3 | tee -a $ERROR_LOG 

echo ----------Loading BALM Data into BA table.---------- | tee -a $ERROR_LOG

$SCRIPTS/$3/loader-scripts/BA/load_balm_data.sh $1 $2 $3 | tee -a $ERROR_LOG 

echo ----------Loading BA Data into BA table.---------- | tee -a $ERROR_LOG

$SCRIPTS/$3/loader-scripts/BA/load_ba_data.sh $1 $2 $3 | tee -a $ERROR_LOG 

fi
