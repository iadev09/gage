#!/usr/bin/env bash

URL=${1-https://gage.home/ping}

h2load  -n 3200 -c 100 -m 100 "${URL}"
