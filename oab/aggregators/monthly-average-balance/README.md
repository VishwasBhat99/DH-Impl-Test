# Monthly Average Balance
This program reads a file for each date of a month and computes the average balance for each account.

## Input
1. n folders in DDMMYYYY format with each folder containing a pipe separated text file whose path is to be supplied in the script. Here n = Maximum number of days in a month.

Example:

/input/01012019/FinnoneLoans.txt

/input/02012019/FinnoneLoans.txt

...

/input/31012019/FinnoneLoans.txt

Format of file

acc_no|date|balance|interest_rate

Example:

A01|01-01-2019|100|1.2

## Output
1. Pipe separated text File containing the average monthly balance for each account

Format of file

acc_no|average_balance|average_interest_rate

Example:

A01|3.23|0.04

## Code Logic

`average_balance(acc_no) = sum(balance(acc_no))/n`

`average_interest_rate(acc_no) = sum(interest_rate(acc_no))/n`

n = Maximum number of days in a month
