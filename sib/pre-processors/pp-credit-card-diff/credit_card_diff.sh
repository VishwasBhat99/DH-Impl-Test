awk -F '|' '
BEGIN {
    OFS="|"
    account_number="8097252000000266"
    currency="INR"
    path_fcgam="test-bed/fc_gam.txt"
    path_fceab="test-bed/fc_eab.txt"
}
ARGIND == 2 {
    sum15th += $15
}
ARGIND == 1 {
    if ($2 == account_number) {
        field2[$2] = $4
        field3[$2] = $1
    }
}
ARGIND == 3 {
    field5[$1] = $3
}
END {
    field4 = (account_number in field2) ? field2[account_number] : 0
    output5 = (account_number in field3) ? ((field3[account_number] in field5) ? field5[field3[account_number]] : 0) : 0
    printf("%s|%s|%.2f|%.2f|%.2f|%.2f\n", account_number, currency, sum15th, field4, output5, field4 - sum15th) 
}' test-bed/credit_card_master.txt test-bed/fc_eab.txt test-bed/fc_gam.txt > output.txt

