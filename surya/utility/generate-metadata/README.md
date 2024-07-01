# Metadata Generator
This program generates metadata (JSON format) file using a .proto file.

## Input
1. .proto file

## Output
1. metadata in JSON format

## Purpose
To write a program for generating a .cf file with a specific structure, a developer needs to write a .proto file with that same structure. This .proto file is used by the generate_protobuf program to generate a .rs file viz. required by the .cf generating program.  When another program reads this .cf file, it requires a metadata file which needs to match with the .proto file's structure. 

Before the developent of this program, the metadata file had to be manually written using the .proto file for each .cf generating program. This program will use that same .proto file to generate metadata in JSON format. Thus eliminating the need for manually writing a metadata file for every .cf generating program.

## Features
1. It will skip message cashflow (case insensitive) when preparing the metadata
2. It formats and beautifies the metadata
