#!/usr/bin/env bash

create_dir()
  {
    if [ ! -d $1 ]
    	then mkdir $1
    fi
  }

rename_file()
  {
     if [ ! -f $2 ]
       then
         mv $1 $2 
    fi
  }

dir=$SIT_CFDATA/GC/$1
create_dir $dir

dir=$SIT_PREPROCESS/GC/$1
create_dir $dir

dir=$SIT_SUMMARY/GC/$1
create_dir $dir

dir=$SIT_LOGS/GC/$1
create_dir $dir

dir=/home/dbuser/logs/GC/$1
create_dir $dir

old=( ls $SIT_INPUT/GC/$1/Regulatory*.xlsx )
new=$SIT_INPUT/GC/$1/Regulatory_CASA_Listing_Finance_$1.xlsx
rename_file $old $new

old=$( ls $SIT_INPUT/GC/$1/Outstanding_Bills*.xlsx )
new=$SIT_INPUT/GC/$1/Outstanding_Bills_$1.xlsx
rename_file $old $new

old=$( ls $SIT_INPUT/GC/$1/Bond_Master_ADF_*.xlsx )
new=$SIT_INPUT/GC/$1/Bond_Master_ADF_$1.csv
rename_file $old $new

if [ $SIT_INPUT/GC/$1/rec-lend.txt == $2 -a ! -f $2 ]
  then
    touch $2
fi

if [ -f $2 ]
then
	echo "$2 exist"
	exit 0
else
	echo "$2 does not exist"
	exit 1
fi
