#!/usr/bin/env bash
#Input Parameters: file_name wait_time no_of_re_tries

FileName=$1
WaitTime=$2
NoOfReTries=$3

mkdir -p test-bed
LogFilePath=test-bed/PreFlightCheckLog.txt
exit_code=0

rm -f $LogFilePath

echo "Performing Pre-Run check..."
echo "Pre-Run check report" >> $LogFilePath

for (( i=0; i < $NoOfReTries; ++i ))
do
    # When file does not exist
    if ! [ -e $FileName ]
    then
        echo "$FileName does not exist!"
        echo "$FileName does not exist!" >> $LogFilePath
        exit_code=1
        sleep $WaitTime
    fi
done

echo "Pre-Run check completed."
echo "Pre-Run check completed." >> $LogFilePath

echo Exit Code: $exit_code
echo Exit Code: $exit_code >> $LogFilePath

exit $exit_code
