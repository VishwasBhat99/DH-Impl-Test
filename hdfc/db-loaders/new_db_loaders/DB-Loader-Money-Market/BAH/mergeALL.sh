#!/usr/bin/env bash

echo "Merging Borrowings..."
cat $PREPROCESS/BAH/$1/BorOutput_cf_output.txt > $PREPROCESS/BAH/$1/Borrowings.txt
#$PREPROCESS/BAH/$1/BorOutput_aip_output.txt >> $PREPROCESS/BAH/$1/Borrowings.txt
echo "Borrowings Merge Successfull"
echo
echo "Merging Lendings..."
cat $PREPROCESS/BAH/$1/LendOutput_cf_output.txt > $PREPROCESS/BAH/$1/Lendings.txt
#$PREPROCESS/BAH/$1/LendOutput_air_output.txt >> $PREPROCESS/BAH/$1/Lendings.txt
echo "Lendings Merge Successfull"


