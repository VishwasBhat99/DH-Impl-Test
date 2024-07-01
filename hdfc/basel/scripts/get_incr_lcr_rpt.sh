start_time=$(date +"%s")
echo started at $(date +"%T")
next_date=$1
end_date=$2
next_date_timestamp=$(date -d "$next_date" +%s)
end_date_timestamp=$(date -d "$end_date" +%s)

rm class_id.txt
rm acc_info.txt
rm final_rpt.txt
while [ "$next_date_timestamp" -le "$end_date_timestamp" ];
do
    time_1=$(date +"%s")
    date=$next_date
    date_folder=$(date -d "$date" +%d%m%Y)
    date_lookup=$(date -d "$date" +%d-%m-%Y)
    
    echo "Finding accounts for $date_lookup"
    
    file="$PREPROCESS/IND/$date_folder/TDOutput.txt"

    if [ ! -f "$file" ]
    then
        echo "Skipping accounts for $date_lookup. File does not exist"
	echo
	next_date=$(date -d "$date + 1 days")
     	next_date_timestamp=$(date -d "$next_date" +%s)
	continue        
    fi

    awk -v val_dt="$date_lookup" -F"|" '$9==val_dt {print}' $file > acc_info.txt
    echo "started dump"
    awk -F"|" '{print $1"|"$2"|"$3}' $CFDATA/Basel/LCR/IND/$date_folder/nr-final-td.txt > class_id.txt
    awk -F"|" '{print $1"|"$2"|"$3}' $PREPROCESS/Basel/LCR/IND/$date_folder/td-ret-total.txt >> class_id.txt
    echo "getting class id"
    awk -F"|" 'FNR==NR{a[$2]=$1"|"$3; next};{if($1 in a) {print a[$1]"|"$0} else {print "NA"}}' class_id.txt acc_info.txt >> final_rpt.txt
    time_2=$(date +"%s")
    duration=$(($time_2-$time_1))
    echo $next_date completed in $duration seconds
    next_date=$(date -d "$date + 1 days")
    next_date_timestamp=$(date -d "$next_date" +%s)
done
end_time=$(date +"%s")
total_duration=$(($end_time-$start_time))
echo Report generated in $total_duration seconds
echo Succesful
