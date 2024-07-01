# Monthly Average Balance
This program reads a file and computes the average balance for each account.

## Input
1. input file containing a pipe separated account data whose path is to be supplied in the script.

Format of file

bal_amt|curr_code|gl_code|sol_id|acc_no|end_eod_date|eod_date

Example:

421.83|INR|21100|2001|011000044|01-APR-22|01-APR-22

## Output
1. Pipe separated text File containing the average monthly balance for each account

Format of file

acc_no|average_balance

Example:

011000044|14.06

## Code Logic

`average_balance(acc_no) = sum(balance(acc_no))/n`

start_date = starting date of the month
end_date = end date of the month
n = Maximum number of days in a month from start date to end date

Conditions for test cases:

1. if the program is run for any other dates which is not month end, then:
end_date = as_on_date and n value is calculated accordingly.

Example: as_on_date = 16-04-2022 then avg_bal will be calculated for 15 days from 01-APR-2022 to 15-APR-2022.

2. if the end_eod_date and eod_date are of different months, i.e., the account balance is maintained constant all over the month
then total_balance(acc_no) = total_balance(acc_no) * n

Example: if bal = 50000 for an account with end_eod_date = 31-DEC-1999 and eod_date = 03-MAR-2022
total_balance(acc_no) = 50000 * 30



