sid=$(<stream_id.txt)

for each_sid in $sid; do
    echo "${each_sid::-3}" >./batch_id.txt
    read bid <./batch_id.txt

    echo "#!usr/bin/env bash

read date < /home/readuser/scripts/as-on-date.txt
echo \$date

/home/readuser/scripts/autosys \$date $bid $each_sid \$SOCKET_ADDRESS 2 60" >generate_${each_sid}.sh

done
