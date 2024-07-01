#! /bin/bash

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

dir=$CFDATA/IND/$1
create_dir $dir

dir=$PREPROCESS/IND/$1
create_dir $dir

dir=$SUMMARY/IND/$1
create_dir $dir

dir=$LOGS/IND/$1
create_dir $dir

actual=$( ls $INPUT/IND/$1/alm_bills_*.csv | ls -t )
new=$INPUT/IND/$1/alm_bills_$1.csv
rename_file $actual $new

actual=$INPUT/IND/$1/master-loan.csv
new=$INPUT/IND/$1/master-loans.csv
rename_file $actual $new

actual=$INPUT/IND/$1/cashflow-loan.csv
new=$INPUT/IND/$1/cashflows-loans.csv
rename_file $actual $new

actual=$INPUT/IND/$1/cashflow-inv.csv
new=$INPUT/IND/$1/cashflows-inv.csv
rename_file $actual $new

actual=$INPUT/IND/$1/cashflow-inv.csv
new=$INPUT/IND/$1/cashflows-inv.csv
rename_file $actual $new

actual=$( ls $INPUT/IND/$1/BG_Invocation_Report_*.CSV )
new=$INPUT/IND/$1/BG_Invocation_Report_$1.CSV
rename_file $actual $new

actual=$( ls $INPUT/IND/$1/Bond_Master_ADF_*.csv | ls -t )
new=$INPUT/IND/$1/Bond_Master_ADF_$1.csv
rename_file $actual $new

actual=$( ls $INPUT/IND/$1/FF_FN_ALM_LOAN_EXTRACT_NONMOR_*.txt | ls -t )
new=$INPUT/IND/$1/FF_FN_ALM_LOAN_EXTRACT_NONMOR_$1.txt
rename_file $actual $new

actual=$( ls $INPUT/IND/$1/FF_FN_ALM_LOAN_EXTRACT_MOR_*.txt | ls -t ) 
new=$INPUT/IND/$1/FF_FN_ALM_LOAN_EXTRACT_MOR_$1.txt
rename_file $actual $new

actual=$( ls $INPUT/IND/$1/FF_FN_ALM_LOAN_SCHEDULE_CF_MOR_*.txt | ls -t )
new=$INPUT/IND/$1/FF_FN_ALM_LOAN_SCHEDULE_CF_MOR_$1.txt
rename_file $actual $new

actual=$( ls $INPUT/IND/$1/alm_loans_*.csv | ls -t )
new=$INPUT/IND/$1/alm_loans_$1.csv
rename_file $actual $new

actual=$( ls $INPUT/IND/$1/alm_loan_schedule_*.csv | ls -t )
new=$INPUT/IND/$1/alm_loan_schedule_$1.csv
rename_file $actual $new

actual=$( ls $INPUT/IND/$1/BA_securitisation_* )
new=$INPUT/IND/$1/ba_sec_final_template.xlsx
rename_file $actual $new

actual=$( ls $INPUT/IND/$1/Foreclosure* )
new=$INPUT/IND/$1/foreclosure.csv
rename_file $actual $new

actual=$( ls $INPUT/IND/$1/LC_Devolved* )
new=$INPUT/IND/$1/LC_Devolved.xlsx
rename_file $actual $new

actual=$( ls $INPUT/IND/$1/NDS_OM_Data*.xlsx )
new=$INPUT/IND/$1/NDS_OM_Data.xlsx
rename_file $actual $new

actual=$( ls $INPUT/IND/$1/OTC_Data*.xlsx )
new=$INPUT/IND/$1/OTC_Data.xlsx
rename_file $actual $new

if [ $INPUT/IND/$1/EDW_ALM_TD353_$1.csv == $2 ]
    then
    rm $PREPROCESS/IND/$1/TDOut* -f
    rm $CFDATA/IND/$1/TDCF* -f
    rm $SUMMARY/IND/$1/TDAgg* -f
fi

if [ $INPUT/IND/$1/FF_FN_ALM_LOAN_EXTRACT_NONMOR_$1.txt == $2 ]
    then
    rm $PREPROCESS/IND/$1/Finnone* -f
    rm $CFDATA/IND/$1/Finnone* -f
    rm $SUMMARY/IND/$1/FinLoans* -f
fi

if [ -f $2 ]
then
	echo "$2 exist"
	exit 0
else
	echo "$2 does not exist"
	exit 1
fi
