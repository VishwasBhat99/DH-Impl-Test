#RF file: 
It is a JSON file. This file contains the list and order of fields to be read from cf file and written to output file.
Order of fields in output is same as the order written in RF file.
In addition to fields read from CF file, we can also write derived BALM LLG at any position. 
To write BALM LLG in output use "BALM_LLG" as field name in the field descriptor and output_field_type as "String".

#Struct of field descriptor consist of:
A field descriptor must have following 2 fields
* field_name (name of field exactly same as used in metadata file)
* output_field_type (data type you want to write the data in output file)
eg. all date fields are stored as timestamp in cf file. to write date as Date type(dd-mm-yyyy) in output use output_field_type as Date.

#Possible list of output field type:
* Integer (Default 0)
* Float (Default 0.0)
* Date - format dd-mm-yyyy (Default 01-01-1970)
* String (Default "")

#Sample RF file:
{
	"fields":[
		{
			"field_name": "account_id",
			"output_file_type": "String"
		},
		{
            "field_name": "currency",
            "output_file_type": "String"
        },
		{
            "field_name": "int_rate",
            "output_file_type": "Float"
        },
		{
            "field_name": "start_date",
            "output_file_type": "Date"
        },
		{
            "field_name": "",
            "output_file_type": "Date"
        }, 
		{
            "field_name": "BALM_LLG",
            "output_file_type": "String"
        }
	]
}

#Sample output for above required field:
31011060913000031SYN|USD|2.7773|13-09-2006|01-01-1970|12604
31011070516000015HFP|OMR|3|16-05-2007|01-01-1970|12901
31011070620000003HFP|OMR|3|20-06-2007|01-01-1970|12901
31011070620000004HFP|OMR|3|20-06-2007|01-01-1970|12901
31011070707000025SPL|OMR|3.5|07-07-2007|01-01-1970|12701
31011070707000026HFP|OMR|3|07-07-2007|01-01-1970|12901
31011070708000023HFP|OMR|5.25|08-07-2007|01-01-1970|12901
31011070709000032SPL|OMR|5.25|09-07-2007|01-01-1970|12701
31011070711000031HFP|OMR|3.5|11-07-2007|01-01-1970|12901
31011070712000002SPL|OMR|3.5|12-07-2007|01-01-1970|12701
