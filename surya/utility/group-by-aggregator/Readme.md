#Metadata file: 
It is a JSON file. This file contains the field-name,data-type and position in the input file.

#Config file:
It is a JSON file.
The config file has the following fields:
* aggr_keys (name of fields exactly in metadata based on which aggregation needs to happen).
* aggr_values (A vector of values that must have the following 2 fields
	* field_name (name of fields exactly in metadata whose values will be aggregated)
	* operator (Accepts 2 values: 
		1. 'neg': the value is multiplied by (-1)
		2. 'abs': The absolute of the value is considered))
* wt_avg_fields (A series of comma separated fields (weight:amount) whose weighted average is to be calculated)
* op_fields: (List of fields to be written to output. They must contain the following 2 fields:
	* field_name: name of fields exactly in metadata
	* operator: 'neg'/'abs' if the field is a floating type)
Note: The output fields must be present in aggr_keys or aggr_values or wt_avg_fields fields.
#Sample CONFIG file:
{
    "aggr_keys": "acct_crncy_code,segment_code,acid,bacid,gl_sub_head_code,final_npa_class,nfs",
    "aggr_values": [
        {
            "field_name":"out_bal_amt",
            "operator": [""]
        }
    ],
    "wt_avg_fields": "int_rt:out_bal_amt,final_int_rt:book_value",
    "op_fields": [
        {
            "field_name":"out_bal_amt",
            "operator": ["neg"]
	},
	{
	    "field_name":"gl_sub_head_code",
            "operator": [""]
        },
        {
            "field_name":"acid",
            "operator": [""]
        },	
	{
            "field_name":"acct_crncy_code",
            "operator": [""]
        }
    ]
}

