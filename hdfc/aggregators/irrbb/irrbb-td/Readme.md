Document for IRRBB program:

Step 1. Creating Rules:
    • In the previous version our irrbb program was only processing the accounts when the following criterias were met:
        i. When the currency in input file is same as that of base currency passes as program argument.
        ii. When the product code is not present in the product codes from the product code file.
    • In the current version we are handling these two cases through the rules file. 
    • We will assign two new rules in the rules file, one for skipping ccy and another  for skipping prod_code. 
        Ex.
        1|1|0|ccy|NE|INR|7777
        2|1|0|prod_cd|EQ|ABC|OR
	 	2|1|1|prod_cd|EQ|PQR|OR
	 	2|1|2|prod_cd|EQ|XYZ|8888

     	NE-> Not Equal
		EQ-> Equal

    • Because of this all the excluded ccy acounts will be written in 7777.txt file and all the excluded prod_code accounts will be written to 	     8888.txt file. And after the program has executed we will load all files except these two files.

Step 2. Configuring req_fields.json file:
    • In the current version of the program, we can configure the output format of the program using the req_fields.json. 
      Ex.
	{
	    "fields" : [
		{
		    "name":"account_number",
		    "type": "String"
		},
		{
		    "name": "institution",
		    "type": "String"
		},
		{
		    "name": "acc_open_date",
		    "type": "I64"
		},
		{
		    "name": "amt",
		    "type": "F64"
		}
	    ]
	}


    • This req_fields.json will give the output of the program as:
      account_number| institution| acc_open_date| amt
    • "name":"account_number" specifies that the variable with name “account_number” in the metadat.json file will be written. And "type": 	 "String" specifies that the type of value expected to be written is of character type. 
      Similarly "type": "F64" means the value if of decimal number type and  "type": "I64" means that the value is of date type.
    • The  "name" and "type" values pair should be exactly the same as found in metadata.json. If any mismatch is there then the program will 	    fail.
    • If any field needs to be added to the output then just replicate one field information:
	{
        "name": "acc_open_date",
        "type": "I64"
 	},
	and rename the name and type field accordingly.
    • The way the req_fields.json is configured will dictate my output structure, so configuring this file properly is of utmost importance.
      




