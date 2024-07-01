What is a Protocol Buffer (protobuf):

Protocol Buffers (Protobuf) is a method of serializing structured data. It is useful in developing programs to communicate with each other over a wire or for storing data. The method involves an interface description language that describes the structure of data and a program that generates source code from that description for generating or parsing a stream of bytes that represents the structured data.

Google developed Protocol Buffers for internal use and provided a code generator for multiple languages under an open source license.

The design goals for Protocol Buffers emphasized simplicity and performance. In particular, it was designed to be smaller and faster than XML.

Protocol Buffers are widely used at Google for storing and interchanging all kinds of structured information. The method serves as a basis for a custom remote procedure call (RPC) system that is used for nearly all inter-machine communication at Google.


Instruction to install protobuf:

curl -OL https://github.com/google/protobuf/releases/download/v3.3.0/protoc-3.3.0-linux-x86_64.zip
unzip protoc-3.3.0-linux-x86_64.zip -d protoc3
sudo mv protoc3/bin/* /usr/local/bin/
sudo mv protoc3/include/* /usr/local/include/
sudo chown $USER /usr/local/bin/protoc
sudo chown -R $USER /usr/local/include/google


Where and how we use protobuf in rust:

We are using protobuf to generate a .rs file, which is used in our chasflow generator program. The way we generate the file is, we have an API for rust which acts as a code generator (refer zip.). There we pass a ".proto" file, as input, where we specify the structure of how i want my output file of cashflow generator to be. The ouput of this API is a ".rs" file (protobuf generated) which can be used in our cashflow generator program (as an individual module).


Structure of .proto file :

message Account {
string field_name_1: = 1;
double field_name_2: = 2;
int64 field_name_3: = 3;
double field_name_4: = 4;
int64 field_name_5: = 5;
}

use string datatype for all fields that contains characters.
use double for all fields that contain some kind of number(both whole and decimal).
use int64 for all fields that contains date formtted data.


How to run the API:

1.  We first create the .proto file. This file specifies the output file structure of my cashflow generator program. 
2.  After creaing this file, we go to the API that we have for rust, and place the file in the     src folder.
3.  After placing this file, we open the main.rs module and look for the vriable 'out_dir' and     'input'. These two variales specify where my input file(.proto) is placed and in which folder will my output file be generted in.
4.  Here we mention the path of the .proto file in input and output folder name in out_dir.
5.  Use cargo run in terminal to execute the API and generate the .rs file.


---Important stuff to Remember---

We have two types of .proto file. One for maturity products and another for non maturity products. The main and only difference between these two proto files is that the marurity one uses a vector of cashflows (refer the maturity.proto file in attached API). In that file we have a seperate structure that specifies the cshflow, and this structure is placed after all the fields have been specified.
Ex. 

message Cashflow {
    double interest_amount= 1;
    double principal_amount= 2;
    int64 date= 3;
}
.
.
.
'repeated Cashflow cashflows = 7;'.

This cashflow struct consistes of a interest_amount, principal_amount and date variable.
DO NOT CHANGE THIS STRUCT NOR CHANGE THIS LINE 'repeated Cashflow cashflows = 7;' (except for number).

Here the Cashflow is the datatype, 'cashflows' is the name of the struct, '35' is the position and 'repeated' specifies that this struct will be repeated thereby making it a Vec<Cashflows>.

