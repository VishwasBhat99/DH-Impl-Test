#!/usr/bin/env bash

echo "Merging Borrowings..."
cat $PREPROCESS/IND/$1/BorOutput_cf_output.txt > $PREPROCESS/IND/$1/Borrowings.txt
# cat $PREPROCESS/IND/$1/BorOutput_aip_output.txt > $PREPROCESS/IND/$1/Borrowings.txt
echo "Borrowings Merge Successfull"
echo
echo "Merging Lendings..."
cat $PREPROCESS/IND/$1/LendOutput_cf_output.txt > $PREPROCESS/IND/$1/Lendings.txt
# cat $PREPROCESS/IND/$1/LendOutput_air_output.txt > $PREPROCESS/IND/$1/Lendings.txt
echo "Lendings Merge Successfull"


