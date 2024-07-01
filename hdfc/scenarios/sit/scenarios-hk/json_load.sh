#!/usr/bin/env bash

empty_dep=$(<dependencies.json)
dep_ex=$(<dep_ex.json)
dep_mur=$(<dep_mur.json)
dep_recon=$(<dep_rec.json)
dep_gl=$(<dep_gl.json)
dep_load=$(<dep_load.json)

sqlplus -s $SIT_CON_STR_IND << EOF
--delete from batchstream where batchid=7;
insert into batchstream values ( 7, 7000, utl_raw.cast_to_raw ('$empty_dep'));
insert into batchstream values ( 7, 7004, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 7, 7008, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 7, 7012, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 7, 7016, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 7, 7020, utl_raw.cast_to_raw ('$dep_mur'));
insert into batchstream values ( 7, 7024, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 7, 7028, utl_raw.cast_to_raw ('$dep_recon'));
insert into batchstream values ( 7, 7032, utl_raw.cast_to_raw ('$dep_gl'));
insert into batchstream values ( 7, 7036, utl_raw.cast_to_raw ('$dep_gl'));
insert into batchstream values ( 7, 7040, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 7, 7042, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 7, 7044, utl_raw.cast_to_raw ('$dep_load'));
commit;

exit
EOF
