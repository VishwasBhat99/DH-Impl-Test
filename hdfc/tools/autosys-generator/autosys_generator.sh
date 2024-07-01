sid=$(<stream_id.txt)
ip=$(<ip.txt)

for each_ip in $ip
do
for each_sid in $sid
do
    echo "${each_sid::-3}" > ./batch_id.txt
    read bid < ./batch_id.txt

    echo "#!/bin/sh

read date < /home/readuser/scripts/as-on-date.txt
echo \$date

curl -X POST --url http://$each_ip/trigger/1 -H 'Content-Type: application/json' -d '{ \"as_on_date\": \"'\"\$date\"'\", \"batch_id\": $bid, \"stream_ids\": [$each_sid] }'

/home/readuser/scripts/autosys-status-checker \$date $bid $each_sid $each_ip " > generate_${each_sid}_${each_ip}.sh

done
done
