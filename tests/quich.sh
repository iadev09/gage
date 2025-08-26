#!/usr/bin/env bash

for i in {1..100}; do
	quiche-client https://gage.home/ping --requests 20 > /dev/null 2>&1 &
done
wait
echo "done"
