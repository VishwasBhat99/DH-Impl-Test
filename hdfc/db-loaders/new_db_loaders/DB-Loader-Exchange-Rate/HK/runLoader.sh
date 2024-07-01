#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Exchange-Rate/HK/delete.sh $2
/home/dbuser/programs/DB-Loader-Exchange-Rate/HK/commit.sh 
/home/dbuser/programs/DB-Loader-Exchange-Rate/HK/ImportData.sh $1 

/home/dbuser/programs/DB-Loader-Exchange-Rate/HK/commit.sh 
