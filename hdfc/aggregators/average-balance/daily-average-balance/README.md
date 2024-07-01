# Monthly Average Daily
This program reads a .cf file and writes the average balance into a text file for the day of the run.

## Input
1. A pipe separated text file whose path is to be supplied in the script.
2. A metadata JSON file which has the same fields as the .cf file is to be supplied in the script.
3. A required_fields JSON file which maps the program's required fields to the field in metadata.

## Output
1. Pipe separated text File containing the account number, date, balance and interest for each record.

Format of file

acc_no|date|balance|interest_rate

Example:

A01|01-01-2019|100|1.2

## Link to document

[Google Doc](https://drive.google.com/open?id=1KdSLV1g51ezjsvm9wEv3uDYEbBYfzOwQqz7C4PbFg9c)
