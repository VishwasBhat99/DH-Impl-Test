#!/usr/bin/env bash

STA=$"test-bed/static-adder.txt"
STR=$"test-bed/static-remover.txt"
DYN=$"test-bed/dynamic-adder.txt"
EXM=$"test-bed/exclude-master.txt"

cargo run --release -- \
--static-adder ${STA} \
--static-remover ${STR} \
--dynamic-master ${DYN} \
--exclude-master ${EXM} 
