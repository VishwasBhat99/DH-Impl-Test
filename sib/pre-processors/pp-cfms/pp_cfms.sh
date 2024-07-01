#!/bin/bash

Input_File="CFMS_Input.txt"
Final_Output_file="pp_out_cfms.txt"
net_bal_amount=0;
net_bal_amount1=0;
result_net_bal_amount=0;
sum=0;
sum1=0;
date_field="";

if [ -f $Input_File ];
then
      cat $Input_File> $Final_Output_file
     net_bal_amount=$(awk -F '|' '{if($4 =="Provisions for Wealth tax" || $4 =="Povisions for Other impaired assets") {sum=sum+$15}}; END{printf "%.5f\n",sum}' $Input_File)
     net_bal_amount_1=$(awk -F '|' '{if($4 =="Tax paid in advance"||$4=="Tax Suspense"||$4=="Wealth Tax Adv." ||$4=="Int. Tax Suspense") {sum1=sum1+$15}}; END{printf "%.5f\n",sum1}' $Input_File)
     date_field=$(awk -F '|' 'END{print $6}' $Input_File)
     outstr1="888|Other Liabilities & Provisions|Liabilities|Provision Diff1|INR|$date_field|0|0|0|0|0|0|0|0";
     result_net_bal_amount=$(awk -v net_bal_amount_comparision="$net_bal_amount" -v net_bal_amount1_comparision="$net_bal_amount1_comparision" 'BEGIN {if(net_bal_amount_comparision>net_bal_amount1_comparision) printf "%.5f\n",net_bal_amount_comparision-net_bal_amount1_comparision; else print 0}')
     outstr1=$outstr1"|"$result_net_bal_amount"|0|0";
     echo $outstr1>> $Final_Output_file;
      net_bal_amount=$(awk -F '|' '{if($4 =="Deferred Tax Liability") {sum=sum+$15}}; END{printf "%.5f\n",sum}' $Input_File)
      net_bal_amount_1=$(awk -F '|' '{if($4 =="Deferred Tax Asset") {sum1=sum1+$15}}; END{printf "%.5f\n",sum1}' $Input_File)
      outstr2="889|Other Liabilities & Provisions|Liabilities|Provision Diff2|INR|$date_field|0|0|0|0|0|0|0|0";
       result_net_bal_amount=$(awk -v net_bal_amount_comparision="$net_bal_amount" -v net_bal_amount1_comparision="$net_bal_amount_1" 'BEGIN {if(net_bal_amount_comparision>net_bal_amount1_comparision) printf "%.5f\n",net_bal_amount_comparision-net_bal_amount1_comparision; else print 0}')
      outstr2=$oustr2"|"$result_net_bal_amount"|0|0";
      echo $outstr2 >> $Final_Output_file;
else
	echo "$Input not found. Make sure file exists and setup correctly."
fi
