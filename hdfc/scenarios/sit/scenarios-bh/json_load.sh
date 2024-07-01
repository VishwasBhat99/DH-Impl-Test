#!/usr/bin/env bash

empty_dep=$(<dependencies.json)
dep_ex=$(<dep_ex.json)
dep_mur=$(<dep_mur.json)
dep_recon=$(<dep_rec.json)
dep_gl=$(<dep_gl.json)
dep_load=$(<dep_load.json)

sqlplus -s $SIT_CON_STR_IND << EOF
insert into batchstream values ( 3, 3000, utl_raw.cast_to_raw ('$empty_dep'));
insert into batchstream values ( 3, 3004, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 3, 3008, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 3, 3012, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 3, 3016, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 3, 3020, utl_raw.cast_to_raw ('$dep_mur'));
insert into batchstream values ( 3, 3024, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 3, 3028, utl_raw.cast_to_raw ('$dep_recon'));
insert into batchstream values ( 3, 3032, utl_raw.cast_to_raw ('$dep_gl'));
insert into batchstream values ( 3, 3036, utl_raw.cast_to_raw ('$dep_gl'));
insert into batchstream values ( 3, 3040, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 3, 3042, utl_raw.cast_to_raw ('$dep_ex'));
insert into batchstream values ( 3, 3044, utl_raw.cast_to_raw ('$dep_load'));
commit;

exit
EOF
