#!/usr/bin/env bash
#Input Parameters:DD-MM-YYYY

mkdir -p test-bed
mkdir -p test-bed/$1
InputDataPath=test-bed/$1/
LogFilePath=test-bed/$1/
LogFileName=PreRunCheckLog.txt
exit_code=0
rm -f $LogFilePath$LogFileName

checkForFile()
{
    # When file does not exist
    if ! [ -e $InputDataPath$1 ]
    then
        echo "$InputDataPath$1 does not exist!"
        echo "$InputDataPath$1 does not exist!" >> $LogFilePath$LogFileName
        exit_code=1
    fi
}

echo "Performing Pre-Run check..."
echo "Pre-Run check report" >> $LogFilePath$LogFileName

checkForFile CoreBase.txt

checkForFile GLBalAcc.txt

checkForFile OtherPlacementBorrow.txt

checkForFile PlacementBorrowCF.txt

checkForFile REPO.txt

checkForFile Schedule.txt

checkForFile SecurityCF.txt

checkForFile SecurityData.txt

checkForFile SPOTRATE.txt

checkForFile TIAVAIL.txt

checkForFile TIIMPL.txt

checkForFile TILCACC.txt

checkForFile UNUTILISED.txt

echo "Pre-Run check completed."
echo "Pre-Run check completed." >> $LogFilePath$LogFileName

echo Exit Code: $exit_code
echo Exit Code: $exit_code >> $LogFilePath$LogFileName

exit $exit_code
