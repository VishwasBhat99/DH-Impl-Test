#!/usr/bin/env bash

echo "Merging Borrowings..."
cat $PREPROCESS/HK/$1/BorOutput_cf_output.txt > $PREPROCESS/HK/$1/Borrowings.txt
# cat $PREPROCESS/HK/$1/BorOutput_aip_output.txt >> $PREPROCESS/HK/$1/Borrowings.txt
echo "Borrowings Merge Successfull"
echo
echo "Merging Lendings..."
cat $PREPROCESS/HK/$1/LendOutput_cf_output.txt > $PREPROCESS/HK/$1/Lendings.txt
# cat $PREPROCESS/HK/$1/LendOutput_air_output.txt >> $PREPROCESS/HK/$1/Lendings.txt
echo "Lendings Merge Successfull"


