#!/bin/bash

set -m

# excute server side
./server &
bg %1
sleep 3

# excute client side
node client.js 