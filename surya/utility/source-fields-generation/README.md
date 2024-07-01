Document: Src-Fields-Generation 

Objective:
This program generates source fields from .CF file

Input Parameters:
1. Log, Diag-Log: Path and file where Logs must be written.
2. Config-file: Vec of Files having [source_file_name, metadata_file, req_fields_file, input_file].
3. Output-Path: Path where output file must be stored.

Output:
Written file(.txt) stored in output path.

Logic Implemented in the Program:
1. Read the req_fields_file (This file contains vector of fields) Eg: [field1, field2]
2. Read the input_cf_file along with the metadata_file
3. For each record in input:
    The below output line is written in output_file:
        source_file_name|field_name|value|Y|admin|as_on_date|admin|as_on_date

Note: 
i) The fields that are mentioned in req_fields_file are only written
ii) All the values are written, the values may be unique or 
    it may contain duplicates if input has same value for some field

Example: Let source_file_name = s1 and if there are 5 records in input and req_fields = [field1, field2]
    Then: Output:
        s1|field1|value10|Y|admin|as_on_date|admin|as_on_date
        s1|field1|value11|Y|admin|as_on_date|admin|as_on_date
        s1|field1|value12|Y|admin|as_on_date|admin|as_on_date
        s1|field1|value13|Y|admin|as_on_date|admin|as_on_date
        s1|field1|value14|Y|admin|as_on_date|admin|as_on_date
        s1|field2|value20|Y|admin|as_on_date|admin|as_on_date
        s1|field2|value21|Y|admin|as_on_date|admin|as_on_date
        s1|field2|value22|Y|admin|as_on_date|admin|as_on_date
        s1|field2|value23|Y|admin|as_on_date|admin|as_on_date
        s1|field2|value24|Y|admin|as_on_date|admin|as_on_date
