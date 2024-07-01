#!/usr/bin/env bash

empty_dep=$(<dependencies.json)
dep_ex=$(<dep_ex.json)
dep_mur=$(<dep_mur.json)
dep_recon=$(<dep_rec.json)
dep_gl=$(<dep_gl.json)
dep_load=$(<dep_load.json)

sqlplus -s $SIT_CON_STR_IND << EOF
--delete from batchstream where batchid=5;
insert into batchstream values ( 5, 5000, utl_raw.cast_to_raw ('$empty_dep'));
insert into batchstream values ( 5, 5004, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 5, 5008, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 5, 5012, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 5, 5016, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 5, 5020, utl_raw.cast_to_raw ('$dep_mur'));
insert into batchstream values ( 5, 5024, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 5, 5028, utl_raw.cast_to_raw ('$dep_recon'));
insert into batchstream values ( 5, 5032, utl_raw.cast_to_raw ('$dep_gl'));
insert into batchstream values ( 5, 5036, utl_raw.cast_to_raw ('$dep_gl'));
insert into batchstream values ( 5, 5040, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 5, 5042, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 5, 5044, utl_raw.cast_to_raw ('$dep_load'));
commit;

exit
EOF
