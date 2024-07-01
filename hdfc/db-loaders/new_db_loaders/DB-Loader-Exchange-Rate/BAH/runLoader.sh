#!/usr/bin/env bash

/home/dbuser/programs/DB-Loader-Exchange-Rate/BAH/delete.sh $2
/home/dbuser/programs/DB-Loader-Exchange-Rate/BAH/commit.sh 
/home/dbuser/programs/DB-Loader-Exchange-Rate/BAH/ImportData.sh $1 

/home/dbuser/programs/DB-Loader-Exchange-Rate/BAH/commit.sh 


