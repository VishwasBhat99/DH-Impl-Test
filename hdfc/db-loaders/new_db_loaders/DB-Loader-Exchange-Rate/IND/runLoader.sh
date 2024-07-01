#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Exchange-Rate/IND/delete.sh $2
/home/dbuser/programs/DB-Loader-Exchange-Rate/IND/commit.sh 
/home/dbuser/programs/DB-Loader-Exchange-Rate/IND/ImportData.sh $1 

/home/dbuser/programs/DB-Loader-Exchange-Rate/IND/commit.sh 


