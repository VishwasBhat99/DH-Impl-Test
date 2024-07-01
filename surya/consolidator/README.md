# BALM Consolidator
This program reads the aggregator output and generates a consolidated summary of it.

## Input
1. 12 aggregator output files with each file containing a pipe separated bucketed data.

    Example:

    /input/aggregated-0.txt

    /input/aggregated-1.txt

    ...

    /input/aggregated-11.txt

    Sample file contents

    4200|31-05-2019|INR|Master|INT|10.00|0.00|0.00|...

2. A Summary file

    Example:

    /input/aggregated-summary.txt

    Sample Summary file contents

    4200|31-05-2019|INR|INT|Master|O|0|0

3. Consolidator Configuration
    Format:
    source_currency|consolidation_currency|display_currency

    Example:

    INR|USD|USC
    RUP|USD|USC
    INR|USD|USR

## Output
1. Pipe separated text File containing the consolidated data

    Example:

    4200|31-05-2019|USR|IRS|Master|O|46889959.21|8.59

## Code Logic
1. file_index = 0
2. Read 1 input file
    
    2.1 when file_index == 12

        input file = summary file

        header_size = 6
    
    2.2 when file_index != 12

        input file = inflow/outflow file

        header_size = 5

3. Read the currency from the 3rd field in input file

4. Lookup the consolidation_currency and display_currency for the read currency

5. For the currency get the exchange_rate

6. converted_value = bucketed_value * exchange_rate

7. Loop through input_file record data with index

    7.1 when index < header_size

        replace 3rd field with display_currency

    7.2 when index >= header_size

        replace bucketed_value with converted_value


8. Write record till header_size

9. Loop through input_file record with index in steps of 3

    total_amount = previous_bucket_value[index] + bucketed_value[index]

    weighted_int = (previous_bucket_value[index] * previous_bucket_value[index + 1]) + (bucketed_value[index] * bucketed_value[index + 1])

    9.1 when total_amount != 0

        write weighted_int / total_amount

    9.2 when total_amount == 0

        write 0.0

    9.3. when file_index != 12

        write previous_bucket_value[index + 2]

10. file_index = file_index + 1

11. when file_index <= 12 

    go to step 2