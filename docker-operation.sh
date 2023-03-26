#!/bin/bash

if [ $# != 1 ]; then
    echo "Wrong number of arg"
    echo "./docker-operation.sh run or remove"
    exit 1    
fi

ARG=$1

if [ $ARG = "run" ]; then
    docker build -t local-rpc .
    docker run -it --name local-rpc local-rpc:latest
elif [ $ARG = "remove" ]; then
    docker rm local-rpc
    docker rmi local-rpc:latest
fi