This file is contains the fix-len-to-txt program expexted errors and metadata details
1.sample metadata format
{
    "fields": [
    {
        "name": "customer_id",
        "position": 1,
        "typ": "I64",
        "start_pos":1,
        "max_len":10
    },
    {
        "name": "account_number",
        "position": 2,
        "typ": "String",
        "start_pos":13,
        "max_len":16
    },
    {
        "name": "title",
        "position": 3,
        "typ": "String",
        "start_pos":33,
        "max_len":41
    },
    {
        "name": "product_code",
        "position": 4,
        "typ": "I64",
        "start_pos":75,
        "max_len":5
    }
   
]
  }

Field details:
1.name : This field contains the name as we want header in the output file.
2.position: This field contains on which position the field is present in the fix-len input file.
3.type: this field contains the data type of the input field Only these 4 types are accepted(I64,I32,F64,F32,String).
4.start_pos: This field contains the starting position of the current field.
5.max_len : This field contains the maximum length of the current field.
Possible Errors:
1.Program will throw an error if the position number is not in the correct order and duplicated.
Example: 
A.Ordering->position should start from 1 and go like 2,3,4 .............
B.Duplecation->One position can not be repeated twice.like( 1 can not be came twice)
2.Program will throw an error if any field start position is lesser than the end position of the previous fields.like Two field should not be overlapped.
Example.If the current field will end at 10th then next field should start from more than 10th position(11 or 12 .....).
