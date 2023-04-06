#!/bin/bash

for i in {10..1}
do
  curl --location 'http://127.0.0.1:8000/test_queue' \
  --header 'Content-Type: text/plain' \
  --data "$i"
done
